# Trading History Database Guide

Your arbitrage bot now automatically saves all trading activity to a local SQLite database!

##  What Gets Saved

The bot now tracks:

1. **Individual Trades** - Every fill on both platforms
2. **Arbitrage Opportunities** - All detected opportunities (executed or not)
3. **Daily Summaries** - Aggregate statistics per day
4. **Position Snapshots** - Periodic position state

##  Database File

- **Location**: `trading_history.db` (in project root)
- **Format**: SQLite3 (standard database format)
- **Auto-created**: Created automatically on first run
- **Persistent**: Data survives bot restarts

##  Viewing Your Trading History

### Quick View (Recent Activity)

```bash
cargo run --bin trading_history
```

Shows:
- Today's summary
- Last 10 trades

### Available Commands

```bash
# Show today's summary
cargo run --bin trading_history summary

# View last 20 trades
cargo run --bin trading_history recent 20

# View all-time statistics
cargo run --bin trading_history stats

# View recent arbitrage opportunities (executed and missed)
cargo run --bin trading_history opportunities 30

# View daily summaries for last 7 days
cargo run --bin trading_history daily 7

# Export all trades to CSV
cargo run --bin trading_history export

# Show help
cargo run --bin trading_history help
```

##  What You Can Track

### All-Time Statistics
- Total trades executed
- Total trading volume
- Total fees paid
- Arbitrage opportunities detected vs executed
- Success rate
- Average execution time
- Platform breakdown

### Daily Performance
- Trades per day
- Volume per day
- Realized P&L
- Success rate trends

### Individual Trades
- Timestamp
- Market
- Platform (Kalshi/Polymarket)
- Side (YES/NO)
- Price
- Size
- Fees
- Profit

### Arbitrage Opportunities
- All detected opportunities
- Which ones were executed
- Detection latency
- Execution latency

##  Database Schema

### Tables

1. **`trades`** - All individual fills
   - timestamp, market_id, description
   - platform, side, contracts, price
   - cost, fees, order_id
   - arb_type, profit_cents, execution_time_us

2. **`arbitrage_opportunities`** - Detected arbs
   - timestamp, market_id, description
   - arb_type, yes_price, no_price
   - yes_size, no_size, profit_cents
   - executed (true/false)
   - detection_latency_ns, execution_latency_ns

3. **`daily_summary`** - Daily aggregates
   - date, total_trades, total_volume
   - realized_pnl, total_fees
   - opportunities_detected, opportunities_executed
   - execution_success_rate

4. **`position_snapshots`** - Position states
   - timestamp, market_id
   - contracts by platform and side
   - total_cost, guaranteed_profit
   - unmatched_exposure, status

##  Advanced Queries (SQLite)

You can also query the database directly using SQLite:

```bash
# Install SQLite browser (free GUI tool)
# Or use command line

sqlite3 trading_history.db

# Example queries:
SELECT * FROM trades ORDER BY timestamp DESC LIMIT 10;
SELECT date, total_trades, realized_pnl FROM daily_summary;
SELECT COUNT(*), SUM(profit_cents) FROM trades WHERE arb_type IS NOT NULL;
```

##  Exporting Data

### Export to CSV
```bash
cargo run --bin trading_history export
```

Creates a timestamped CSV file: `trades_export_YYYYMMDD_HHMMSS.csv`

You can then:
- Open in Excel/Google Sheets
- Analyze in Python/R
- Import into other tools

### Export to JSON (programmatic)

The database can be queried programmatically from Rust code:

```rust
use prediction_market_arbitrage::database::TradingDatabase;

let db = TradingDatabase::new()?;
let trades = db.get_recent_trades(100)?;
// Process trades...
```

##  Example Output

### Today's Summary
```
 Today's Summary (2026-01-19):
────────────────────────────────────────────────────────────────────────────────
  Total Trades: 24
  Total Volume: $2,450.00
  Total Fees:   $18.50
  Realized P&L: $45.00
  Opportunities Detected: 150
  Opportunities Executed: 24
  Success Rate: 16.0%
```

### Recent Trades
```
 Recent Trades (Last 10):
────────────────────────────────────────────────────────────────────────────────
14:23:45 | POLYMARKET | YES Lakers-Warriors @$0.4200 x10 | Cost: $4.20 | Fees: $0.00
14:23:45 | KALSHI | NO Lakers-Warriors @$0.5800 x10 | Cost: $5.80 | Fees: $0.14
14:15:32 | POLYMARKET | YES Chiefs-Bills @$0.4500 x15 | Cost: $6.75 | Fees: $0.00
14:15:32 | KALSHI | NO Chiefs-Bills @$0.5400 x15 | Cost: $8.10 | Fees: $0.18
```

### All-Time Stats
```
 All-Time Trading Statistics
════════════════════════════════════════════════════════════════════════════════
  Total Trades Executed:      248
  Total Trading Volume:       $25,680.00
  Total Fees Paid:            $189.50
  Net Volume (after fees):    $25,490.50

  Opportunities Detected:     1,250
  Opportunities Executed:     248
  Execution Success Rate:     19.8%

  Average Execution Time:     145 µs (0.15 ms)
  Best Trade Profit:          12 cents

  Platform Breakdown:
    POLYMARKET: 124 trades, $11,340.00 volume, $0.00 fees
    KALSHI: 124 trades, $14,340.00 volume, $189.50 fees
```

##  Data Privacy & Security

- Database is stored **locally only** (never uploaded)
- Added to `.gitignore` (won't be committed to Git)
- Contains only trade data (no private keys)
- **Backup your database** if you want to keep historical data

##  Tips

1. **Regular Backups**: Copy `trading_history.db` to a safe location
2. **Analysis**: Use the CSV export for detailed analysis in Excel
3. **Monitoring**: Check daily summaries to track performance trends
4. **Debugging**: Use opportunity logs to understand execution patterns

##  Troubleshooting

### "Database is locked"
- Close any SQLite browser or tool accessing the database
- Stop the bot before running queries

### Missing data
- Database is created on first run
- Check that bot has write permissions in project directory

### Reset database
```bash
# Delete database (all history lost!)
rm trading_history.db

# It will be recreated on next run
```

## Integration

The database is fully integrated into your bot:

-  **Automatic logging** - Every trade is saved
-  **Real-time updates** - Data written immediately
-  **Non-blocking** - Won't slow down trading
-  **Error resilient** - Bot continues even if DB fails

---

**Ready to view your data?**

```bash
cargo run --bin trading_history
```

Happy trading!
