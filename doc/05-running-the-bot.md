# Running the Bot Guide

Now that everything is set up, let's run your bot! This guide will show you how to start it and understand what it's doing.

## Before You Start

✅ **Checklist:**
- [ ] Rust is installed (`rustc --version` works)
- [ ] Bot is built (`cargo build --release` completed successfully)
- [ ] `.env` file is created with your credentials
- [ ] `DRY_RUN=1` in your `.env` file (we'll start in safe mode!)
- [ ] You have funds in both Kalshi and Polymarket accounts

## Step 1: Make Sure You're in the Right Folder

Open Terminal/PowerShell and navigate to your bot folder:

```bash
cd prediction-market-arbitrage
```

(Or wherever you saved the bot)

## Step 2: Run the Bot (Dry Run Mode)

**Start with dry run mode first!** This lets you see what the bot would do without using real money.

```bash
dotenvx run -- cargo run --release
```

### What Should Happen

You'll see output like this:

```
🚀 Prediction Market Arbitrage System v2.0
   Profit threshold: <0.5¢ (0.5% minimum profit)
   Monitored leagues: []
   Mode: DRY RUN (set DRY_RUN=0 to execute)
[KALSHI] API key loaded
[POLYMARKET] Creating async client and deriving API credentials...
[POLYMARKET] Client ready for 0x742d35Cc...
📂 Loaded 1234 team code mappings
🔍 Market discovery...
📊 Market discovery complete:
   - Matched market pairs: 45

📋 Discovered market pairs:
   ✅ Lakers vs Warriors | poly_yes_kalshi_no | Kalshi: KXLALGAME-12345
   ✅ Chiefs vs Bills | kalshi_yes_poly_no | Kalshi: KXNFLGAME-67890
   ...
```

The bot will then start monitoring markets and looking for arbitrage opportunities.

### Understanding the Output

Here's what each part means:

- **🚀 Prediction Market Arbitrage System v2.0** - Bot version
- **Profit threshold** - Minimum profit % needed to trade (0.5% default)
- **Mode: DRY RUN** - Safe mode, no real trades
- **[KALSHI] API key loaded** - Connected to Kalshi ✓
- **[POLYMARKET] Client ready** - Connected to Polymarket ✓
- **Team code mappings** - Markets matched between platforms
- **Discovered market pairs** - Markets ready to monitor

## Step 3: Let It Run

Once started, the bot will:

1. **Discover markets** - Find matching markets between platforms
2. **Connect to WebSockets** - Get real-time price updates
3. **Monitor prices** - Watch for arbitrage opportunities
4. **Log activity** - Show what it's doing

You'll see messages like:

```
[INFO] Monitoring 45 market pairs
[INFO] Connected to Kalshi WebSocket
[INFO] Connected to Polymarket WebSocket
[DEBUG] Price update: LAL-GSW-YES @ 0.42
[DEBUG] Price update: LAL-GSW-NO @ 0.58
```

### What to Look For

When an arbitrage opportunity is found (in dry run mode), you'll see:

```
[INFO] 🔍 ARB OPPORTUNITY DETECTED (DRY RUN):
   Market: Lakers vs Warriors
   Type: poly_yes_kalshi_no
   Cost: $0.98
   Profit: $0.02 (2.04%)
   Would execute: Buy Polymarket YES @ $0.40, Buy Kalshi NO @ $0.58
```

In dry run mode, it shows what it **would** do but doesn't actually trade.

## Step 4: Run in Live Mode (When Ready)

⚠️ **ONLY do this when you're confident everything works!**

1. **Edit your `.env` file:**
   - Change `DRY_RUN=1` to `DRY_RUN=0`

2. **Save the file**

3. **Run the bot again:**
   ```bash
   dotenvx run -- cargo run --release
   ```

4. **Watch carefully:**
   - You should see: `Mode: LIVE EXECUTION`
   - The bot will now actually place orders
   - Monitor it closely, especially at first!

### Or Run Live Mode Temporarily (Without Editing .env)

You can also override the setting for one run:

```bash
DRY_RUN=0 dotenvx run -- cargo run --release
```

This runs live mode once without changing your `.env` file.

## Step 5: Stop the Bot

To stop the bot:

- Press `Ctrl + C` (Windows/Linux) or `Cmd + C` (Mac)
- The bot will stop gracefully
- Any open positions will remain (you'll need to manage them manually)

## Common Command Examples

### Basic Dry Run (Safe Testing)
```bash
dotenvx run -- cargo run --release
```

### Dry Run with Verbose Logging
```bash
RUST_LOG=debug dotenvx run -- cargo run --release
```

### Live Trading (Real Money!)
```bash
DRY_RUN=0 dotenvx run -- cargo run --release
```

### Force Market Re-Discovery
If markets aren't matching, force refresh:
```bash
FORCE_DISCOVERY=1 dotenvx run -- cargo run --release
```

### Live Trading with Custom Loss Limit
```bash
DRY_RUN=0 CB_MAX_DAILY_LOSS=1000.0 dotenvx run -- cargo run --release
```

## Understanding Bot Behavior

### When the Bot Finds Opportunities

The bot continuously monitors prices. When it finds an opportunity:

1. **Calculates profit** - Checks if YES + NO < $1.00
2. **Checks circuit breaker** - Makes sure limits aren't exceeded
3. **Executes trades** - Places orders on both platforms simultaneously
4. **Confirms fills** - Waits for orders to fill
5. **Logs result** - Shows what happened

### Expected Behavior

- **Opportunities are rare** - Don't expect trades every minute
- **Opportunities are short-lived** - They may disappear quickly
- **Not all opportunities execute** - Some may fill partially or not at all
- **You need funds on both platforms** - Bot can't trade without money!

## Monitoring Your Bot

### What to Monitor

1. **Connections** - Make sure both WebSocket connections stay open
2. **Error messages** - Any `[ERROR]` messages need attention
3. **Circuit breaker** - If it trips, the bot stops automatically
4. **Positions** - Check your positions on both platforms regularly
5. **P&L** - Monitor your profit and loss

### Log Levels Explained

| Level | When Used | Example |
|-------|-----------|---------|
| `error` | Critical problems | Connection failures, auth errors |
| `warn` | Important warnings | Circuit breaker warnings, partial fills |
| `info` | Normal operations | Opportunities found, trades executed |
| `debug` | Detailed information | Every price update, order details |
| `trace` | Very detailed | Internal state, network packets |

Set `RUST_LOG=info` for normal use, or `RUST_LOG=debug` to see more details.

## Running the Bot 24/7

If you want the bot to run continuously:

### Option 1: Keep Terminal Open
- Just leave Terminal/PowerShell open
- Bot will run until you close it or it crashes

### Option 2: Use Screen (Linux/Mac)
```bash
screen -S arbitrage-bot
dotenvx run -- cargo run --release
# Press Ctrl+A then D to detach
# Reattach later with: screen -r arbitrage-bot
```

### Option 3: Use a Process Manager (Advanced)
- Windows: Run as a service or use Task Scheduler
- Linux: Use systemd or supervisor
- Mac: Use launchd

## What's Next?

Your bot should now be running! If you encounter any problems, check:

**[Troubleshooting Guide →](./06-troubleshooting.md)**

Or review the other guides:
- [Getting Started](./01-getting-started.md)
- [Installation](./02-installation.md)
- [Credentials](./03-credentials.md)
- [Configuration](./04-configuration.md)

Happy trading! 🚀

