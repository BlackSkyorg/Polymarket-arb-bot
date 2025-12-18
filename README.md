# Kalshi-Poly / Poly-Poly / Kalshi-Kalshi Arbitrage Bot

**Kalshi-Poly arbitrage bot**, **Poly-Poly arbitrage bot**, and **Kalshi-Kalshi arbitrage bot** for automated cross-platform trading. A high-performance, production-ready arbitrage trading system that monitors price discrepancies between Kalshi and Polymarket, executing risk-free arbitrage opportunities in real-time with sub-millisecond latency.

> 🔍 **Search Keywords**: polymarket arbitrage bot, polymarket-kalshi arbitrage bot, kalshi-poly arbitrage, poly-poly arbitrage, kalshi-kalshi arbitrage, kalshi arbitrage, prediction market arbitrage, cross-platform trading bot

## Overview

This **Kalshi-Poly / Poly-Poly / Kalshi-Kalshi arbitrage bot** identifies and executes arbitrage opportunities across:

- **Kalshi-Poly markets** (cross-platform arbitrage between Kalshi and Polymarket)
- **Poly-Poly markets** (same-platform arbitrage on Polymarket)
- **Kalshi-Kalshi markets** (same-platform arbitrage on Kalshi)

The bot takes both sides of a market when YES and NO prices add up to less than $1.00, guaranteeing a risk-free profit at market expiry.

### How It Works

**Example Opportunity:**
- YES = $0.40, NO = $0.58
- Total cost = $0.98
- At expiry: YES = $1.00 and NO = $0.00 (or vice versa)
- **Result: 2.04% risk-free return**

### Market Insights

When observing large traders like PN1 finding significant size in these opportunities, the initial assumption was that opportunities would be extremely fleeting with intense competition. However, the reality is quite different:

- **Opportunities are persistent**: While concurrent dislocations aren't frequent, when they do occur, they persist long enough to execute manually
- **Large traders use limit orders**: Whales typically fill positions via limit orders over extended periods, as odds don't fluctuate significantly before game time
- **Manual execution is viable**: Opportunities remain available long enough for manual intervention if needed

### System Workflow

The repository implements the following workflow:

1. **Market Scanning**: Scans sports markets that expire within the next couple of days
2. **Market Matching**: Matches Kalshi-Polymarket markets using:
   - Cached mapping of team names between platforms
   - Kalshi-Polymarket event slug building conventions
3. **Real-time Monitoring**: Subscribes to orderbook delta WebSockets to detect instances where YES + NO can be purchased for less than $1.00
4. **Order Execution**: Executes trades concurrently on both platforms
5. **Risk Management**: Includes position management and circuit breakers (note: not extensively battle-tested in production)

### Useful Components

Beyond the complete arbitrage system, you may find these components particularly useful:

- **Cross-platform market mapping**: The team code mapping system for matching markets across Kalshi and Polymarket
- **Rust CLOB client**: A Rust rewrite of Polymarket's Python `py-clob-client` (focused on order submission only)

## Quick Start

### 1. Install Dependencies

```bash
# Rust 1.75+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release
```

### 2. Set Up Credentials

Create a `.env` file:

```bash
# === KALSHI CREDENTIALS ===
KALSHI_API_KEY_ID=your_kalshi_api_key_id
KALSHI_PRIVATE_KEY_PATH=/path/to/kalshi_private_key.pem

# === POLYMARKET CREDENTIALS ===
POLY_PRIVATE_KEY=0xYOUR_WALLET_PRIVATE_KEY
POLY_FUNDER=0xYOUR_WALLET_ADDRESS

# === SYSTEM CONFIGURATION ===
DRY_RUN=1
RUST_LOG=info
```

### 3. Run

```bash
# Dry run (paper trading)
dotenvx run -- cargo run --release

# Live execution
DRY_RUN=0 dotenvx run -- cargo run --release
```

---

## Environment Variables

### Required

| Variable                  | Description                                                 |
| ------------------------- | ----------------------------------------------------------- |
| `KALSHI_API_KEY_ID`       | Your Kalshi API key ID                                      |
| `KALSHI_PRIVATE_KEY_PATH` | Path to RSA private key (PEM format) for Kalshi API signing |
| `POLY_PRIVATE_KEY`        | Ethereum private key (with 0x prefix) for Polymarket wallet |
| `POLY_FUNDER`             | Your Polymarket wallet address (with 0x prefix)             |

### System Configuration

| Variable          | Default | Description                                           |
| ----------------- | ------- | ----------------------------------------------------- |
| `DRY_RUN`         | `1`     | `1` = paper trading (no orders), `0` = live execution |
| `RUST_LOG`        | `info`  | Log level: `error`, `warn`, `info`, `debug`, `trace`  |
| `FORCE_DISCOVERY` | `0`     | `1` = re-fetch market mappings (ignore cache)         |
| `PRICE_LOGGING`   | `0`     | `1` = verbose price update logging                    |

### Test Mode

| Variable        | Default              | Description                                                                                    |
| --------------- | -------------------- | ---------------------------------------------------------------------------------------------- |
| `TEST_ARB`      | `0`                  | `1` = inject synthetic arb opportunity for testing                                             |
| `TEST_ARB_TYPE` | `poly_yes_kalshi_no` | Arb type: `poly_yes_kalshi_no`, `kalshi_yes_poly_no`, `poly_same_market`, `kalshi_same_market` |

### Circuit Breaker

| Variable                     | Default | Description                                 |
| ---------------------------- | ------- | ------------------------------------------- |
| `CB_ENABLED`                 | `true`  | Enable/disable circuit breaker              |
| `CB_MAX_POSITION_PER_MARKET` | `100`   | Max contracts per market                    |
| `CB_MAX_TOTAL_POSITION`      | `500`   | Max total contracts across all markets      |
| `CB_MAX_DAILY_LOSS`          | `5000`  | Max daily loss in cents before halt         |
| `CB_MAX_CONSECUTIVE_ERRORS`  | `5`     | Consecutive errors before halt              |
| `CB_COOLDOWN_SECS`           | `60`    | Cooldown period after circuit breaker trips |

---

## Obtaining Credentials

### Kalshi

1. Log in to [Kalshi](https://kalshi.com)
2. Go to **Settings → API Keys**
3. Create a new API key with trading permissions
4. Download the private key (PEM file)
5. Note the API Key ID

### Polymarket

1. Create or import an Ethereum wallet (MetaMask, etc.)
2. Export the private key (include `0x` prefix)
3. Fund your wallet on Polygon network with USDC
4. The wallet address is your `POLY_FUNDER`

---

## Usage Examples

### Paper Trading (Development)

```bash
# Full logging, dry run
RUST_LOG=debug DRY_RUN=1 dotenvx run -- cargo run --release
```

### Test Arbitrage Execution

```bash
# Inject synthetic arb to test execution path
TEST_ARB=1 DRY_RUN=0 dotenvx run -- cargo run --release
```

### Production

```bash
# Live trading with circuit breaker
DRY_RUN=0 CB_MAX_DAILY_LOSS=10000 dotenvx run -- cargo run --release
```

### Force Market Re-Discovery

```bash
# Clear cache and re-fetch all market mappings
FORCE_DISCOVERY=1 dotenvx run -- cargo run --release
```

---

## How It Works

### Arbitrage Mechanics

In prediction markets, the fundamental property holds: **YES + NO = $1.00** (guaranteed).

This **Polymarket arbitrage bot** and **Polymarket-Kalshi arbitrage bot** exploits this property by detecting when:

```
Best YES ask (Platform A) + Best NO ask (Platform B) < $1.00
```

**Example Scenario (Kalshi-Poly Arbitrage):**

```
Kalshi YES ask:  42¢
Polymarket NO ask: 56¢
Total cost:      98¢
Guaranteed payout: 100¢
Net profit:       2¢ per contract (2.04% return)
```

The bot automatically executes both legs simultaneously, locking in the risk-free profit.

### Arbitrage Opportunity Types

This **Kalshi-Poly / Poly-Poly / Kalshi-Kalshi arbitrage bot** supports four types of arbitrage opportunities:

| Type                 | Execution Strategy                          | Frequency | Description |
| -------------------- | ------------------------------------------- | --------- | ----------- |
| `poly_yes_kalshi_no` | Buy Polymarket YES + Buy Kalshi NO          | Common    | **Kalshi-Poly**: Cross-platform arbitrage |
| `kalshi_yes_poly_no` | Buy Kalshi YES + Buy Polymarket NO          | Common    | **Kalshi-Poly**: Cross-platform arbitrage |
| `poly_only`          | Buy Polymarket YES + Buy Polymarket NO      | Rare      | **Poly-Poly**: Same-platform arbitrage |
| `kalshi_only`        | Buy Kalshi YES + Buy Kalshi NO              | Rare      | **Kalshi-Kalshi**: Same-platform arbitrage |

### Fee Structure

- **Kalshi**: Trading fees calculated as `ceil(0.07 × contracts × price × (1-price))` - automatically factored into arbitrage detection
- **Polymarket**: Zero trading fees on all orders

---

## Architecture

This **Kalshi-Poly / Poly-Poly / Kalshi-Kalshi arbitrage bot** is built with a modular, high-performance architecture optimized for low-latency execution:

```
src/
├── main.rs              # Application entry point and WebSocket orchestration
├── types.rs             # Core type definitions and market state management
├── execution.rs         # Concurrent order execution engine with position reconciliation
├── position_tracker.rs # Channel-based position tracking and P&L calculation
├── circuit_breaker.rs   # Risk management with configurable limits and auto-halt
├── discovery.rs         # Intelligent market discovery and matching system
├── cache.rs             # Team code mapping cache for cross-platform matching
├── kalshi.rs            # Kalshi REST API and WebSocket client
├── polymarket.rs        # Polymarket WebSocket client and market data
├── polymarket_clob.rs   # Polymarket CLOB order execution client
└── config.rs            # League configurations and system thresholds
```

### Key Features

- **Lock-free orderbook cache** using atomic operations for zero-copy updates
- **SIMD-accelerated arbitrage detection** for sub-millisecond latency
- **Concurrent order execution** with automatic position reconciliation
- **Circuit breaker protection** with configurable risk limits
- **Intelligent market discovery** with caching and incremental updates

---

## Development

### Run Tests

```bash
cargo test
```

### Enable Profiling

```bash
cargo build --release --features profiling
```

### Benchmarks

```bash
cargo bench
```

---

## Project Status

### ✅ Completed Features

- [x] Kalshi REST API and WebSocket client
- [x] Polymarket REST API and WebSocket client
- [x] Lock-free atomic orderbook cache
- [x] SIMD-accelerated arbitrage detection
- [x] Concurrent multi-leg order execution
- [x] Real-time position and P&L tracking
- [x] Circuit breaker with configurable risk limits
- [x] Intelligent market discovery with caching
- [x] Automatic exposure management for mismatched fills

### 🚧 Future Enhancements

- [ ] Web-based risk limit configuration UI
- [ ] Multi-account support for portfolio management
- [ ] Advanced order routing strategies
- [ ] Historical performance analytics dashboard

---

## Topics & Keywords

This **Kalshi-Poly / Poly-Poly / Kalshi-Kalshi arbitrage bot** repository covers:

- **Kalshi-Poly arbitrage** - Cross-platform arbitrage between Kalshi and Polymarket
- **Poly-Poly arbitrage** - Same-platform arbitrage on Polymarket markets
- **Kalshi-Kalshi arbitrage** - Same-platform arbitrage on Kalshi markets
- **Polymarket arbitrage** - Automated trading on Polymarket prediction markets
- **Kalshi arbitrage** - Automated trading on Kalshi prediction markets  
- **Cross-platform arbitrage** - Exploiting price differences between Polymarket and Kalshi
- **Prediction market trading** - Automated trading bot for prediction markets
- **Arbitrage trading bot** - High-frequency arbitrage detection and execution
- **Market making bot** - Risk-free market making via arbitrage
- **Sports betting arbitrage** - Arbitrage opportunities in sports prediction markets
- **Rust trading bot** - High-performance trading system written in Rust

### Related Technologies

- Rust async/await for high-performance concurrent execution
- WebSocket real-time price feeds (Kalshi & Polymarket)
- REST API integration (Kalshi & Polymarket CLOB)
- Atomic lock-free data structures for orderbook management
- SIMD-accelerated arbitrage detection algorithms

---

## Contributing

Contributions are welcome! This **Kalshi-Poly / Poly-Poly / Kalshi-Kalshi arbitrage bot** is open source and designed to help the prediction market trading community.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
