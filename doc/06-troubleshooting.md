# Troubleshooting Guide

Having problems? This guide covers common issues and how to fix them.

## Installation Problems

### "cargo: command not found" or "rustc: command not found"

**Problem:** Rust isn't installed or not in your PATH.

**Solution:**
1. **Windows:**
   - Restart your computer (important!)
   - Open a NEW PowerShell window (not the old one)
   - Run: `rustc --version`
   - If still not working, try: `$env:Path += ";$env:USERPROFILE\.cargo\bin"`
   
2. **Mac/Linux:**
   - Run: `source $HOME/.cargo/env`
   - Or restart Terminal
   - Run: `rustc --version`

3. **If still not working:**
   - Reinstall Rust from https://rustup.rs
   - Make sure to restart your computer/terminal after installation

### "dotenvx: command not found"

**Problem:** dotenvx isn't installed or not in PATH.

**Solution:**
1. **Close and reopen Terminal/PowerShell completely**

2. **Reinstall dotenvx:**
   - **Windows:** `iwr https://dotenvx.sh/install.ps1 -useb | iex`
   - **Mac/Linux:** `curl -fsSL https://dotenvx.sh/install.sh | sh`

3. **Verify installation:**
   ```bash
   dotenvx --version
   ```

4. **If still not working:**
   - Check your PATH: `echo $PATH` (Mac/Linux) or `$env:Path` (Windows)
   - Make sure `~/.cargo/bin` (or equivalent) is in your PATH

### Build Errors: "could not compile..."

**Problem:** Code won't compile.

**Solution:**
1. **Update Rust:**
   ```bash
   rustup update
   ```

2. **Clean and rebuild:**
   ```bash
   cargo clean
   cargo build --release
   ```

3. **Check your Rust version:**
   ```bash
   rustc --version
   ```
   - Should be 1.75.0 or higher
   - If not, update: `rustup update stable`

4. **Network issues:**
   - Make sure you have internet connection
   - Try again later (might be temporary network issue)
   - Check firewall isn't blocking cargo

## Credential Problems

### "KALSHI_API_KEY_ID not set"

**Problem:** Can't find your Kalshi API key.

**Solution:**
1. **Check your `.env` file exists:**
   ```bash
   ls -la .env  # Mac/Linux
   Test-Path .env  # Windows
   ```

2. **Check the variable name is correct:**
   - Should be: `KALSHI_API_KEY_ID=your_key_here`
   - No spaces around `=`
   - No quotes (unless value has spaces)

3. **Make sure you're running with dotenvx:**
   ```bash
   dotenvx run -- cargo run --release
   ```
   Not just `cargo run --release`!

### "Failed to read private key from..."

**Problem:** Can't find or read the Kalshi private key file.

**Solution:**
1. **Check the file exists:**
   ```bash
   ls -la kalshi_private_key.pem  # Mac/Linux
   dir kalshi_private_key.pem  # Windows
   ```

2. **Check the path in `.env`:**
   - If file is in same folder as `.env`, use: `KALSHI_PRIVATE_KEY_PATH=kalshi_private_key.pem`
   - Or use full path: `KALSHI_PRIVATE_KEY_PATH=C:/full/path/to/file.pem`

3. **Check file permissions (Mac/Linux):**
   ```bash
   chmod 600 kalshi_private_key.pem
   ```

4. **Check file format:**
   - File should be a `.pem` file from Kalshi
   - Open it in text editor - should start with `-----BEGIN RSA PRIVATE KEY-----`
   - Make sure it's not corrupted

### "POLY_PRIVATE_KEY not set" or "POLY_FUNDER not set"

**Problem:** Polymarket credentials missing or wrong.

**Solution:**
1. **Check your `.env` file:**
   - Make sure both `POLY_PRIVATE_KEY` and `POLY_FUNDER` are set
   - Values should start with `0x`
   - Private key should be 66 characters (0x + 64 hex chars)
   - Wallet address should be 42 characters (0x + 40 hex chars)

2. **Verify your wallet:**
   - Check your MetaMask (or other wallet)
   - Make sure you copied the private key correctly
   - Make sure you copied the wallet address correctly

3. **Common mistakes:**
   - Missing `0x` prefix
   - Extra spaces in the value
   - Quotes around the value (don't use quotes)

## Runtime Problems

### "Mode: DRY RUN" but I want live trading

**Problem:** Bot is running in test mode.

**Solution:**
1. **Check your `.env` file:**
   - Should have: `DRY_RUN=0` for live trading
   - `DRY_RUN=1` means test mode

2. **Or override for one run:**
   ```bash
   DRY_RUN=0 dotenvx run -- cargo run --release
   ```

### Bot connects but finds no market pairs

**Problem:** No markets matched between platforms.

**Solution:**
1. **Force market re-discovery:**
   ```bash
   FORCE_DISCOVERY=1 dotenvx run -- cargo run --release
   ```

2. **Check which leagues are enabled:**
   - Look at `ENABLED_LEAGUES` in `src/config.rs`
   - Empty array `[]` means all leagues
   - You can modify this to only specific leagues

3. **Wait for markets to be available:**
   - Markets only appear when there are upcoming games
   - Check both platforms manually to see if markets exist

4. **Check your accounts have access:**
   - Make sure your Kalshi account can access the markets
   - Make sure your Polymarket account is properly set up

### Bot finds opportunities but doesn't execute trades

**Problem:** Opportunities detected but no orders placed.

**Solution:**
1. **Check if you're in DRY_RUN mode:**
   - If `DRY_RUN=1`, bot won't place real orders
   - Change to `DRY_RUN=0` for live trading

2. **Check circuit breaker:**
   - Look for circuit breaker messages in logs
   - If circuit breaker tripped, bot won't trade
   - Wait for cooldown period or reset manually

3. **Check you have funds:**
   - Make sure you have USDC on Polymarket
   - Make sure you have cash on Kalshi
   - Bot needs money to place orders!

4. **Check position limits:**
   - `CB_MAX_POSITION_PER_MARKET` might be too low
   - `CB_MAX_TOTAL_POSITION` might be too low
   - Increase these if needed

### WebSocket connection errors

**Problem:** Can't connect to Kalshi or Polymarket.

**Solution:**
1. **Check internet connection:**
   - Make sure you're online
   - Try pinging: `ping google.com`

2. **Check firewall:**
   - Windows Firewall might be blocking connections
   - Add exception for your terminal/cargo

3. **Check if services are down:**
   - Visit https://kalshi.com - does it work?
   - Visit https://polymarket.com - does it work?
   - Both platforms might be temporarily down

4. **Wait and retry:**
   - Sometimes temporary network issues
   - Wait a few minutes and try again

5. **Check API key permissions:**
   - Make sure your Kalshi API key has trading permissions
   - Make sure it hasn't been revoked

### "Circuit breaker tripped" errors

**Problem:** Bot stops trading due to circuit breaker.

**Solution:**
1. **Check what triggered it:**
   - Look at the error message
   - Common reasons:
     - Too many consecutive errors
     - Daily loss limit exceeded
     - Position limits exceeded

2. **Wait for cooldown:**
   - Bot will automatically retry after cooldown period
   - Default is 300 seconds (5 minutes)

3. **Adjust limits if needed:**
   - If limits are too strict, increase them in `.env`:
     - `CB_MAX_DAILY_LOSS=1000.0` (increase loss limit)
     - `CB_MAX_POSITION_PER_MARKET=100000` (increase position size)

4. **Check for errors:**
   - Look at recent error messages
   - Fix underlying issues before restarting

### Orders not filling

**Problem:** Orders placed but not executing.

**Solution:**
1. **Opportunities disappear quickly:**
   - This is normal - prices change fast
   - Bot tries to execute quickly but can't guarantee fills

2. **Check order status:**
   - Log in to Kalshi and Polymarket
   - Check if orders are pending
   - Cancel stale orders if needed

3. **Check your balance:**
   - Make sure you have enough funds
   - Orders need collateral

4. **Check fees:**
   - Kalshi has trading fees
   - Make sure profit is enough to cover fees

## Performance Issues

### Bot is slow or uses too much CPU

**Solution:**
1. **Use release build (not debug):**
   ```bash
   cargo build --release
   ```
   Always use `--release` for production!

2. **Reduce logging:**
   - Set `RUST_LOG=info` or `RUST_LOG=warn`
   - `RUST_LOG=debug` or `trace` is very verbose

3. **Reduce price logging:**
   - Set `PRICE_LOGGING=0`
   - Price logging is very resource-intensive

### Bot uses too much memory

**Solution:**
1. **Limit market discovery:**
   - Set `ENABLED_LEAGUES` to specific leagues only
   - Fewer markets = less memory

2. **Regular restarts:**
   - Restart bot daily or weekly
   - Clears any memory leaks

## Still Having Problems?

If nothing here helps:

1. **Check the logs:**
   - Run with `RUST_LOG=debug` to see more details
   - Look for `[ERROR]` messages
   - Read the error messages carefully

2. **Test components individually:**
   - Test Kalshi API: Try accessing their website
   - Test Polymarket: Check your wallet connection
   - Test Rust: `rustc --version` should work

3. **Check the code:**
   - Make sure you're using the latest version
   - Pull latest changes: `git pull`
   - Rebuild: `cargo clean && cargo build --release`

4. **Ask for help:**
   - Create an issue on GitHub
   - Include:
     - Error messages
     - Your configuration (without credentials!)
     - What you were trying to do
     - System information (OS, Rust version)

## Quick Reference

### Common Commands

```bash
# Build the bot
cargo build --release

# Run in dry run mode
dotenvx run -- cargo run --release

# Run in live mode
DRY_RUN=0 dotenvx run -- cargo run --release

# Force market refresh
FORCE_DISCOVERY=1 dotenvx run -- cargo run --release

# Verbose logging
RUST_LOG=debug dotenvx run -- cargo run --release

# Clean build
cargo clean && cargo build --release
```

### Check Your Setup

```bash
# Check Rust
rustc --version

# Check dotenvx
dotenvx --version

# Check .env file exists
ls -la .env  # Mac/Linux
Test-Path .env  # Windows

# Check credentials are set (without showing values)
grep -E "^KALSHI|^POLY" .env | cut -d= -f1  # Mac/Linux
Select-String -Pattern "^KALSHI|^POLY" .env  # Windows
```

---

**Good luck!** If you're still stuck, check the other guides or ask for help.

