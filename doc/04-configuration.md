# Configuration Setup Guide

Now we'll create a configuration file that tells the bot how to use your accounts. This file is called `.env` and contains all your settings in one place.

## Step 1: Create the .env File

1. **Navigate to your bot folder:**
   ```bash
   cd prediction-market-arbitrage
   ```
   (or wherever you saved the bot)

2. **Create a new file called `.env`:**
   - **Windows:** Right-click in the folder, choose "New > Text Document", rename it to `.env` (make sure to remove `.txt` extension)
   - **Mac/Linux:** In Terminal, run: `touch .env`

3. **Open the `.env` file in a text editor:**
   - Use Notepad (Windows), TextEdit (Mac), or any text editor
   - **Not Microsoft Word!** Use a plain text editor

## Step 2: Add Your Credentials

Copy and paste the following template into your `.env` file, then fill in YOUR actual values:

```bash
# ============================================
# KALSHI CREDENTIALS
# ============================================
KALSHI_API_KEY_ID=YOUR_KALSHI_API_KEY_ID_HERE
KALSHI_PRIVATE_KEY_PATH=C:/full/path/to/kalshi_private_key.pem

# ============================================
# POLYMARKET CREDENTIALS
# ============================================
POLY_PRIVATE_KEY=0xYOUR_POLYMARKET_PRIVATE_KEY_HERE
POLY_FUNDER=0xYOUR_POLYMARKET_WALLET_ADDRESS_HERE

# ============================================
# SYSTEM CONFIGURATION
# ============================================
DRY_RUN=1
RUST_LOG=info
FORCE_DISCOVERY=0
PRICE_LOGGING=0

# ============================================
# TEST MODE (Leave as is for normal use)
# ============================================
TEST_ARB=0
TEST_ARB_TYPE=poly_yes_kalshi_no

# ============================================
# CIRCUIT BREAKER SETTINGS (Risk Management)
# ============================================
CB_ENABLED=true
CB_MAX_POSITION_PER_MARKET=50000
CB_MAX_TOTAL_POSITION=100000
CB_MAX_DAILY_LOSS=500.0
CB_MAX_CONSECUTIVE_ERRORS=5
CB_COOLDOWN_SECS=300
```

## Step 3: Fill in Your Values

### Kalshi Credentials

1. **KALSHI_API_KEY_ID:**
   - Replace `YOUR_KALSHI_API_KEY_ID_HERE` with your actual Kalshi API Key ID
   - Example: `KALSHI_API_KEY_ID=AKIAIOSFODNN7EXAMPLE`
   - No spaces around the `=` sign!

2. **KALSHI_PRIVATE_KEY_PATH:**
   - Replace with the full path to your `.pem` file
   - **Windows examples:**
     - `KALSHI_PRIVATE_KEY_PATH=C:\Users\John\Desktop\prediction-market-arbitrage\kalshi_private_key.pem`
     - Or: `KALSHI_PRIVATE_KEY_PATH=C:/Users/John/Desktop/prediction-market-arbitrage/kalshi_private_key.pem`
   - **Mac/Linux examples:**
     - `KALSHI_PRIVATE_KEY_PATH=/Users/john/Desktop/prediction-market-arbitrage/kalshi_private_key.pem`
   - **Tip:** You can also just put the filename if the file is in the same folder as `.env`:
     - `KALSHI_PRIVATE_KEY_PATH=kalshi_private_key.pem`

### Polymarket Credentials

1. **POLY_PRIVATE_KEY:**
   - Replace `0xYOUR_POLYMARKET_PRIVATE_KEY_HERE` with your actual private key
   - It should start with `0x` and be 66 characters total
   - Example: `POLY_PRIVATE_KEY=0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef`
   - Keep the `0x` at the beginning!

2. **POLY_FUNDER:**
   - Replace `0xYOUR_POLYMARKET_WALLET_ADDRESS_HERE` with your wallet address
   - It should start with `0x` and be 42 characters total
   - Example: `POLY_FUNDER=0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb`
   - Keep the `0x` at the beginning!

## Step 4: Understand Configuration Options

### System Configuration

These settings control how the bot behaves:

| Variable | Value | What It Does |
|----------|-------|--------------|
| `DRY_RUN` | `1` = Safe mode<br>`0` = Live trading | **START WITH `1`** - This makes the bot test without using real money. Only change to `0` when you're ready for real trading! |
| `RUST_LOG` | `info`, `debug`, `warn`, `error` | How much detail to show in logs. `info` is good for most users. |
| `FORCE_DISCOVERY` | `0` = Use cache<br>`1` = Refresh | Usually keep at `0`. Set to `1` if markets aren't matching properly. |
| `PRICE_LOGGING` | `0` = Normal<br>`1` = Verbose | Usually keep at `0`. Set to `1` to see every price update (lots of output!). |

### Circuit Breaker Settings (Risk Management)

These protect you from losing too much money:

| Variable | Default | What It Does |
|----------|---------|--------------|
| `CB_ENABLED` | `true` | Turn circuit breaker on/off. **Keep this `true` for safety!** |
| `CB_MAX_POSITION_PER_MARKET` | `50000` | Maximum contracts to hold in any single market. Adjust based on your capital. |
| `CB_MAX_TOTAL_POSITION` | `100000` | Maximum total contracts across all markets. Adjust based on your capital. |
| `CB_MAX_DAILY_LOSS` | `500.0` | Maximum loss in dollars per day before bot stops. **500 = $500.00** |
| `CB_MAX_CONSECUTIVE_ERRORS` | `5` | How many errors before bot stops. Keep at 5. |
| `CB_COOLDOWN_SECS` | `300` | How long to wait (seconds) after circuit breaker trips. 300 = 5 minutes. |

### Recommended Settings for Beginners

Start with these safe settings:

```bash
DRY_RUN=1
CB_MAX_POSITION_PER_MARKET=100
CB_MAX_TOTAL_POSITION=500
CB_MAX_DAILY_LOSS=100.0
```

This means:
- ✅ Bot runs in test mode (no real money)
- ✅ Max 100 contracts per market
- ✅ Max 500 contracts total
- ✅ Stops if you lose more than $100 in a day

## Step 5: Save and Verify

1. **Save the `.env` file:**
   - Make sure all your values are filled in correctly
   - Double-check there are no extra spaces around the `=` signs
   - Save the file

2. **Verify the file exists:**
   ```bash
   # Windows PowerShell
   Test-Path .env

   # Mac/Linux
   ls -la .env
   ```
   Should return `True` or show the file.

## Common Mistakes to Avoid

❌ **Don't put spaces around `=`:**
   - Wrong: `KALSHI_API_KEY_ID = ABC123`
   - Right: `KALSHI_API_KEY_ID=ABC123`

❌ **Don't use quotes (unless the value has spaces):**
   - Wrong: `KALSHI_API_KEY_ID="ABC123"`
   - Right: `KALSHI_API_KEY_ID=ABC123`

❌ **Don't forget the `0x` prefix for Polymarket values:**
   - Wrong: `POLY_FUNDER=742d35Cc6634C0532925a3b844Bc9e7595f0bEb`
   - Right: `POLY_FUNDER=0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb`

❌ **Don't commit `.env` to Git:**
   - Make sure `.env` is in `.gitignore` (it should be by default)
   - Never upload your `.env` file to GitHub or share it!

## Example .env File (Fake Values)

Here's what a complete `.env` file might look like (with fake values):

```bash
# Kalshi
KALSHI_API_KEY_ID=AKIAIOSFODNN7EXAMPLE12345
KALSHI_PRIVATE_KEY_PATH=kalshi_private_key.pem

# Polymarket
POLY_PRIVATE_KEY=0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
POLY_FUNDER=0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb

# Settings
DRY_RUN=1
RUST_LOG=info
FORCE_DISCOVERY=0
PRICE_LOGGING=0

# Circuit Breaker
CB_ENABLED=true
CB_MAX_POSITION_PER_MARKET=100
CB_MAX_TOTAL_POSITION=500
CB_MAX_DAILY_LOSS=100.0
CB_MAX_CONSECUTIVE_ERRORS=5
CB_COOLDOWN_SECS=300

# Test Mode
TEST_ARB=0
TEST_ARB_TYPE=poly_yes_kalshi_no
```

## What's Next?

Perfect! Your configuration is set up. Now let's test it and run the bot:

**[Running the Bot →](./05-running-the-bot.md)**

