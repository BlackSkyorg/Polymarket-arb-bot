# Getting Started Guide

Welcome! This guide will help you set up and use the Polymarket-Kalshi Arbitrage Bot, even if you've never coded before.

## What This Bot Does

This bot automatically finds and executes **risk-free arbitrage opportunities** between two prediction market platforms:
- **Polymarket** - A decentralized prediction market
- **Kalshi** - A regulated prediction market

### What is Arbitrage?

Arbitrage means buying and selling the same thing on different platforms to make a guaranteed profit. Here's a simple example:

- **Polymarket**: You can buy "YES" for a market at $0.40
- **Kalshi**: You can buy "NO" for the same market at $0.58
- **Total cost**: $0.98
- **Guaranteed payout**: $1.00 (one will always win)
- **Your profit**: $0.02 per contract (2% risk-free return!)

The bot automatically finds these opportunities and executes trades for you 24/7.

## What You'll Need

Before starting, make sure you have:

1. ✅ A computer (Windows, Mac, or Linux)
2. ✅ An internet connection
3. ✅ Accounts on both Polymarket and Kalshi
4. ✅ Funds in both accounts (USDC on Polymarket, cash on Kalshi)
5. ✅ Basic computer skills (opening files, copying text)

## Quick Overview

Here's what you'll do (don't worry, we'll guide you through each step):

1. **Install Rust** (the programming language this bot uses)
2. **Download the bot code**
3. **Get your API keys** from Kalshi and Polymarket
4. **Set up your configuration file** (we'll show you exactly what to write)
5. **Run the bot** and let it make money!

## Next Steps

Ready to start? Follow these guides in order:

1. **[Installation Guide](./02-installation.md)** - Installing Rust and setting up your computer
2. **[Getting Your Credentials](./03-credentials.md)** - Getting API keys from Kalshi and Polymarket
3. **[Configuration Setup](./04-configuration.md)** - Setting up your bot's settings
4. **[Running the Bot](./05-running-the-bot.md)** - Starting your bot and understanding the output
5. **[Troubleshooting](./06-troubleshooting.md)** - Common problems and how to fix them

## Important Safety Notes

⚠️ **Always start in DRY RUN mode** - This lets you test the bot without risking real money. The bot is set to dry run by default, so your real money is safe until you're ready.

⚠️ **Start with small amounts** - Even when you go live, start with small positions to make sure everything works correctly.

⚠️ **Monitor your bot** - Check on it regularly, especially in the beginning.

---

**Ready?** Let's start with [Installation](./02-installation.md)!

