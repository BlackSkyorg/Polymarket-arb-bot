//! Risk management and circuit breaker system.
//!
//! This module provides configurable risk limits, position tracking, and
//! automatic trading halt mechanisms to protect against excessive losses.

use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{error, warn, info};

/// Circuit breaker configuration from environment
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Maximum position size per market (in contracts)
    pub max_position_per_market: i64,
    
    /// Maximum total position across all markets (in contracts)
    pub max_total_position: i64,
    
    /// Maximum daily loss (in dollars) before halting
    pub max_daily_loss: f64,
    
    /// Maximum number of consecutive errors before halting
    pub max_consecutive_errors: u32,
    
    /// Cooldown period after a trip (seconds)
    pub cooldown_secs: u64,
    
    /// Whether circuit breakers are enabled
    pub enabled: bool,
}

impl CircuitBreakerConfig {
    pub fn from_env() -> Self {
        Self {
            max_position_per_market: std::env::var("CB_MAX_POSITION_PER_MARKET")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50000),
            
            max_total_position: std::env::var("CB_MAX_TOTAL_POSITION")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100000),
            
            max_daily_loss: std::env::var("CB_MAX_DAILY_LOSS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(500.0),
            
            max_consecutive_errors: std::env::var("CB_MAX_CONSECUTIVE_ERRORS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
            
            cooldown_secs: std::env::var("CB_COOLDOWN_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(300), // 5 minutes default
            
            enabled: std::env::var("CB_ENABLED")
                .map(|v| v == "1" || v == "true")
                .unwrap_or(true), // Enabled by default for safety
        }
    }
}

/// Reason why circuit breaker was tripped
#[derive(Debug, Clone, PartialEq)]
pub enum TripReason {
    MaxPositionPerMarket { market: String, position: i64, limit: i64 },
    MaxTotalPosition { position: i64, limit: i64 },
    MaxDailyLoss { loss: f64, limit: f64 },
    ConsecutiveErrors { count: u32, limit: u32 },
    ManualHalt,
}

impl std::fmt::Display for TripReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TripReason::MaxPositionPerMarket { market, position, limit } => {
                write!(f, "Max position per market: {} has {} contracts (limit: {})", market, position, limit)
            }
            TripReason::MaxTotalPosition { position, limit } => {
                write!(f, "Max total position: {} contracts (limit: {})", position, limit)
            }
            TripReason::MaxDailyLoss { loss, limit } => {
                write!(f, "Max daily loss: ${:.2} (limit: ${:.2})", loss, limit)
            }
            TripReason::ConsecutiveErrors { count, limit } => {
                write!(f, "Consecutive errors: {} (limit: {})", count, limit)
            }
            TripReason::ManualHalt => {
                write!(f, "Manual halt triggered")
            }
        }
    }
}

/// Position tracking for a single market
#[derive(Debug, Default)]
pub struct MarketPosition {
    pub kalshi_yes: i64,
    pub kalshi_no: i64,
    pub poly_yes: i64,
    pub poly_no: i64,
}

#[allow(dead_code)]
impl MarketPosition {
    pub fn net_position(&self) -> i64 {
        // Net exposure: positive = long YES, negative = long NO
        (self.kalshi_yes + self.poly_yes) - (self.kalshi_no + self.poly_no)
    }
    
    pub fn total_contracts(&self) -> i64 {
        self.kalshi_yes + self.kalshi_no + self.poly_yes + self.poly_no
    }
}

/// Circuit breaker state
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    
    /// Whether trading is currently halted
    halted: AtomicBool,
    
    /// When the circuit breaker was tripped
    tripped_at: RwLock<Option<Instant>>,
    
    /// Reason for trip
    trip_reason: RwLock<Option<TripReason>>,
    
    /// Consecutive error count
    consecutive_errors: AtomicI64,
    
    /// Daily P&L tracking (in cents)
    daily_pnl_cents: AtomicI64,
    
    /// Positions per market
    positions: RwLock<std::collections::HashMap<String, MarketPosition>>,
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        info!("[CB] Circuit breaker initialized:");
        info!("[CB]   Enabled: {}", config.enabled);
        info!("[CB]   Max position per market: {} contracts", config.max_position_per_market);
        info!("[CB]   Max total position: {} contracts", config.max_total_position);
        info!("[CB]   Max daily loss: ${:.2}", config.max_daily_loss);
        info!("[CB]   Max consecutive errors: {}", config.max_consecutive_errors);
        info!("[CB]   Cooldown: {}s", config.cooldown_secs);
        
        Self {
            config,
            halted: AtomicBool::new(false),
            tripped_at: RwLock::new(None),
            trip_reason: RwLock::new(None),
            consecutive_errors: AtomicI64::new(0),
            daily_pnl_cents: AtomicI64::new(0),
            positions: RwLock::new(std::collections::HashMap::new()),
        }
    }
    
    /// Check if trading is allowed
    #[allow(dead_code)]
    pub fn is_trading_allowed(&self) -> bool {
        if !self.config.enabled {
            return true;
        }
        !self.halted.load(Ordering::SeqCst)
    }
    
    /// Check if we can execute a trade for a specific market
    pub async fn can_execute(&self, market_id: &str, contracts: i64) -> Result<(), TripReason> {
        if !self.config.enabled {
            return Ok(());
        }
        
        if self.halted.load(Ordering::SeqCst) {
            let reason = self.trip_reason.read().await;
            return Err(reason.clone().unwrap_or(TripReason::ManualHalt));
        }
        
        // Check position limits
        let positions = self.positions.read().await;
        
        // Per-market limit
        if let Some(pos) = positions.get(market_id) {
            let new_position = pos.total_contracts() + contracts;
            if new_position > self.config.max_position_per_market {
                return Err(TripReason::MaxPositionPerMarket {
                    market: market_id.to_string(),
                    position: new_position,
                    limit: self.config.max_position_per_market,
                });
            }
        }
        
        // Total position limit
        let total: i64 = positions.values().map(|p| p.total_contracts()).sum();
        if total + contracts > self.config.max_total_position {
            return Err(TripReason::MaxTotalPosition {
                position: total + contracts,
                limit: self.config.max_total_position,
            });
        }
        
        // Daily loss limit
        let daily_loss = -self.daily_pnl_cents.load(Ordering::SeqCst) as f64 / 100.0;
        if daily_loss > self.config.max_daily_loss {
            return Err(TripReason::MaxDailyLoss {
                loss: daily_loss,
                limit: self.config.max_daily_loss,
            });
        }
        
        Ok(())
    }
    
    /// Record a successful execution
    pub async fn record_success(&self, market_id: &str, kalshi_contracts: i64, poly_contracts: i64, pnl: f64) {
        // Reset consecutive errors
        self.consecutive_errors.store(0, Ordering::SeqCst);
        
        // Update P&L
        let pnl_cents = (pnl * 100.0) as i64;
        self.daily_pnl_cents.fetch_add(pnl_cents, Ordering::SeqCst);
        
        // Update positions
        let mut positions = self.positions.write().await;
        let pos = positions.entry(market_id.to_string()).or_default();
        pos.kalshi_yes += kalshi_contracts;
        pos.poly_no += poly_contracts;
    }
    
    /// Record an error
    pub async fn record_error(&self) {
        let errors = self.consecutive_errors.fetch_add(1, Ordering::SeqCst) + 1;
        
        if errors >= self.config.max_consecutive_errors as i64 {
            self.trip(TripReason::ConsecutiveErrors {
                count: errors as u32,
                limit: self.config.max_consecutive_errors,
            }).await;
        }
    }
    
    /// Record P&L update (for tracking without execution)
    #[allow(dead_code)]
    pub fn record_pnl(&self, pnl: f64) {
        let pnl_cents = (pnl * 100.0) as i64;
        self.daily_pnl_cents.fetch_add(pnl_cents, Ordering::SeqCst);
    }

    /// Trip the circuit breaker
    pub async fn trip(&self, reason: TripReason) {
        if !self.config.enabled {
            return;
        }
        
        error!("🚨 CIRCUIT BREAKER TRIPPED: {}", reason);
        
        self.halted.store(true, Ordering::SeqCst);
        *self.tripped_at.write().await = Some(Instant::now());
        *self.trip_reason.write().await = Some(reason);
    }
    
    /// Manually halt trading
    #[allow(dead_code)]
    pub async fn halt(&self) {
        warn!("[CB] Manual halt triggered");
        self.trip(TripReason::ManualHalt).await;
    }

    /// Reset the circuit breaker (after cooldown or manual reset)
    #[allow(dead_code)]
    pub async fn reset(&self) {
        info!("[CB] Circuit breaker reset");
        self.halted.store(false, Ordering::SeqCst);
        *self.tripped_at.write().await = None;
        *self.trip_reason.write().await = None;
        self.consecutive_errors.store(0, Ordering::SeqCst);
    }

    /// Reset daily P&L (call at midnight)
    #[allow(dead_code)]
    pub fn reset_daily_pnl(&self) {
        info!("[CB] Daily P&L reset");
        self.daily_pnl_cents.store(0, Ordering::SeqCst);
    }

    /// Check if cooldown has elapsed and auto-reset if so
    #[allow(dead_code)]
    pub async fn check_cooldown(&self) -> bool {
        if !self.halted.load(Ordering::SeqCst) {
            return true;
        }

        let tripped_at = self.tripped_at.read().await;
        if let Some(tripped) = *tripped_at {
            if tripped.elapsed() > Duration::from_secs(self.config.cooldown_secs) {
                drop(tripped_at); // Release read lock before reset
                self.reset().await;
                return true;
            }
        }

        false
    }

    /// Get current status
    #[allow(dead_code)]
    pub async fn status(&self) -> CircuitBreakerStatus {
        let positions = self.positions.read().await;
        let total_position: i64 = positions.values().map(|p| p.total_contracts()).sum();
        
        CircuitBreakerStatus {
            enabled: self.config.enabled,
            halted: self.halted.load(Ordering::SeqCst),
            trip_reason: self.trip_reason.read().await.clone(),
            consecutive_errors: self.consecutive_errors.load(Ordering::SeqCst) as u32,
            daily_pnl: self.daily_pnl_cents.load(Ordering::SeqCst) as f64 / 100.0,
            total_position,
            market_count: positions.len(),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CircuitBreakerStatus {
    pub enabled: bool,
    pub halted: bool,
    pub trip_reason: Option<TripReason>,
    pub consecutive_errors: u32,
    pub daily_pnl: f64,
    pub total_position: i64,
    pub market_count: usize,
}

impl std::fmt::Display for CircuitBreakerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.enabled {
            return write!(f, "Circuit Breaker: DISABLED");
        }
        
        if self.halted {
            write!(f, "Circuit Breaker: 🛑 HALTED")?;
            if let Some(reason) = &self.trip_reason {
                write!(f, " ({})", reason)?;
            }
        } else {
            write!(f, "Circuit Breaker: ✅ OK")?;
        }
        
        write!(f, " | P&L: ${:.2} | Pos: {} contracts across {} markets | Errors: {}",
               self.daily_pnl, self.total_position, self.market_count, self.consecutive_errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_circuit_breaker_position_limit() {
        let config = CircuitBreakerConfig {
            max_position_per_market: 10,
            max_total_position: 50,
            max_daily_loss: 100.0,
            max_consecutive_errors: 3,
            cooldown_secs: 60,
            enabled: true,
        };
        
        let cb = CircuitBreaker::new(config);
        
        // Should allow initial trade
        assert!(cb.can_execute("market1", 5).await.is_ok());
        
        // Record the trade
        cb.record_success("market1", 5, 5, 0.0).await;
        
        // Should reject trade exceeding per-market limit
        let result = cb.can_execute("market1", 10).await;
        assert!(matches!(result, Err(TripReason::MaxPositionPerMarket { .. })));
    }
    
    #[tokio::test]
    async fn test_consecutive_errors() {
        let config = CircuitBreakerConfig {
            max_position_per_market: 100,
            max_total_position: 500,
            max_daily_loss: 100.0,
            max_consecutive_errors: 3,
            cooldown_secs: 60,
            enabled: true,
        };
        
        let cb = CircuitBreaker::new(config);
        
        // Record errors
        cb.record_error().await;
        cb.record_error().await;
        assert!(cb.is_trading_allowed());
        
        // Third error should trip
        cb.record_error().await;
        assert!(!cb.is_trading_allowed());
    }
}