# Getting Your Credentials

To use the bot, you need to give it permission to trade on your behalf using API keys. Think of API keys like passwords that allow the bot to access your accounts.

⚠️ **IMPORTANT:** Keep your API keys secret! Never share them with anyone or post them online. They give full access to your accounts.

## Part 1: Getting Kalshi Credentials

Kalshi requires two things:
1. An **API Key ID** (like a username)
2. A **Private Key file** (like a password file)

### Step 1: Log into Kalshi

1. Go to https://kalshi.com
2. Log in to your account
3. Make sure your account has funds (you need money to trade!)

### Step 2: Create an API Key

1. **Go to Settings:**
   - Click on your profile/account icon (usually top right)
   - Click "Settings" or "Account Settings"

2. **Find API Keys section:**
   - Look for a tab or section called "API Keys" or "Developer Settings"
   - Click on it

3. **Create a new API key:**
   - Click "Create New API Key" or "Generate API Key" button
   - You may be asked to give it a name (e.g., "Arbitrage Bot")
   - Make sure it has **trading permissions** enabled
   - Click "Create" or "Generate"

4. **Save your API Key ID:**
   - You'll see an **API Key ID** (looks like a long string of characters)
   - **WRITE THIS DOWN** or copy it - you'll need it later!
   - Example: `AKIAIOSFODNN7EXAMPLE`

5. **Download your Private Key:**
   - Click "Download Private Key" or similar button
   - This will download a `.pem` file (like `kalshi_private_key.pem`)
   - **SAVE THIS FILE SOMEWHERE SAFE** - you'll need it in the next step
   - ⚠️ **You can only download this once!** Make sure you save it.

### Step 3: Save Your Kalshi Private Key

1. **Move the downloaded file to the bot folder:**
   - Find the `.pem` file you just downloaded
   - Copy or move it to your bot folder (`prediction-market-arbitrage`)
   - You can rename it to `kalshi_private_key.pem` to make it easier

2. **Note the full path to the file:**
   - **Windows example:** `C:\Users\YourName\Desktop\prediction-market-arbitrage\kalshi_private_key.pem`
   - **Mac/Linux example:** `/Users/YourName/Desktop/prediction-market-arbitrage/kalshi_private_key.pem`
   - You'll need this path in the configuration step!

## Part 2: Getting Polymarket Credentials

Polymarket uses your Ethereum wallet as your account. You need:
1. Your **Wallet Address** (your account number)
2. Your **Private Key** (the password to your wallet)

### Step 1: Have a Wallet Ready

You need an Ethereum wallet that works on Polygon network. The most common options:

**Option A: MetaMask (Recommended for beginners)**

1. **Install MetaMask:**
   - Go to https://metamask.io
   - Click "Download" and install the browser extension
   - Create a new wallet or import an existing one
   - ⚠️ **SAVE YOUR SECRET RECOVERY PHRASE** - write it down somewhere safe!

2. **Add Polygon Network:**
   - Open MetaMask
   - Click the network dropdown (usually says "Ethereum Mainnet")
   - Click "Add Network" or "Add a network manually"
   - Enter these details:
     - **Network Name:** Polygon
     - **RPC URL:** `https://polygon-rpc.com`
     - **Chain ID:** `137`
     - **Currency Symbol:** `MATIC`
     - **Block Explorer:** `https://polygonscan.com`
   - Click "Save"

**Option B: Use Existing Wallet**

If you already have a wallet, make sure it's set up for Polygon network.

### Step 2: Fund Your Wallet

1. **Get USDC on Polygon:**
   - Your Polymarket account needs USDC (USD Coin) on Polygon network
   - You can bridge USDC from Ethereum to Polygon, or buy it directly on Polygon
   - Make sure you have USDC, not just MATIC!

2. **Check your balance:**
   - In MetaMask, switch to Polygon network
   - You should see your USDC balance
   - You need at least some USDC to trade (start with a small amount for testing!)

### Step 3: Get Your Wallet Address

1. **In MetaMask:**
   - Click on your account name at the top (it shows "Account 1" or similar)
   - Click to copy your address
   - It starts with `0x` followed by lots of letters and numbers
   - Example: `0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb`
   - **WRITE THIS DOWN** - this is your `POLY_FUNDER`!

### Step 4: Export Your Private Key

⚠️ **WARNING:** Your private key gives full access to your wallet. Never share it!

1. **In MetaMask:**
   - Click the three dots (menu) in the top right
   - Click "Account Details"
   - Click "Export Private Key"
   - Enter your MetaMask password
   - Click to reveal your private key
   - It starts with `0x` followed by 64 characters
   - Example: `0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef`
   - **COPY THIS CAREFULLY** - you'll need it in the configuration step!

2. **Save it securely:**
   - Don't save it in a plain text file that others can access
   - Consider using a password manager
   - You'll paste it into the configuration file in the next step

## Quick Checklist

Before moving to configuration, make sure you have:

- ✅ **Kalshi API Key ID** (copied somewhere safe)
- ✅ **Kalshi Private Key file** (`.pem` file saved in your bot folder)
- ✅ **Polymarket Wallet Address** (starts with `0x`)
- ✅ **Polymarket Private Key** (starts with `0x`, 64 characters after)
- ✅ Both accounts funded with money to trade

## Security Reminders

🔒 **Never share your credentials with anyone**
🔒 **Don't commit your `.env` file to GitHub or share it online**
🔒 **Keep backups of your private keys in a safe place**
🔒 **If someone gets your private keys, they can steal your money!**

## What's Next?

Now that you have all your credentials, let's set up the configuration file:

**[Configuration Setup →](./04-configuration.md)**

