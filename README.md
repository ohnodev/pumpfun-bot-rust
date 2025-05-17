# Solana Pump Bot

A fast and efficient command-line bot for interacting with Pump.fun tokens on Solana. This bot allows you to execute trades programmatically, which is much faster than using a web interface. Perfect for users who want to create their own sniping bots or automate their trading strategies.

> **Version Compatibility Notice**: This bot is tested and working with pump.fun contracts as of May 2025. If pump.fun upgrades their contracts or changes their protocol, this bot may need updates to remain compatible. Always verify the current contract versions before using.

## Features

- 🚀 Fast execution of trades (much faster than web interface)
- 💰 Buy and sell Pump.fun tokens with flexible amounts (raw or percentage-based)
- ⚡ Configurable priority fees for transaction speed
- 🔒 Secure private key management
- 📊 Detailed transaction logging
- 🛠️ Easy to extend and customize
- 🔄 Automatic transaction retry with fresh blockhashes
- 📦 Modular architecture for easy maintenance
- 🎯 Smart slippage handling with automatic retries

## Architecture

The bot is organized into a modular structure for better maintainability and separation of concerns:

### Core Module (`src/core/`)
- `instructions.rs` - Defines Solana program instructions for buying and selling tokens
- `transaction.rs` - Handles transaction creation, signing, and submission with retry logic
- `wallet.rs` - Manages wallet operations and balance checks
- `token_price.rs` - Calculates token prices and swap amounts using bonding curve math

### CLI Module (`src/cli/`)
- `cli.rs` - Handles command-line argument parsing using Clap
- Supports commands:
  * `buy <token_address> <amount>` - Buy tokens with specified SOL amount
  * `sell <token_address> <amount>` - Sell tokens (amount can be raw or percentage like "50%")

### Utils Module (`src/utils/`)
- `config.rs` - Manages configuration and constants (RPC URL, program IDs, etc.)
- `utils.rs` - Common utility functions for token accounts and formatting

### Binary (`src/bin/`)
- `pumpfun-bot.rs` - Main entry point that ties everything together
- Implements high-level buy/sell operations using the core modules

## Prerequisites

### Installing Rust

#### macOS
```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add Rust to your PATH
source "$HOME/.cargo/env"

# Verify installation
rustc --version
cargo --version
```

#### Windows
1. Download and run rustup-init.exe from https://rustup.rs/
2. Follow the installation prompts
3. Open a new Command Prompt or PowerShell window
4. Verify installation:
```bash
rustc --version
cargo --version
```

#### Linux
```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add Rust to your PATH
source "$HOME/.cargo/env"

# Verify installation
rustc --version
cargo --version
```

## Setup

1. Clone this repository:
```bash
git clone https://github.com/yourusername/solana-pump-bot.git
cd solana-pump-bot
```

2. Create a `.env` file in the project root with the following configuration:
```env
# Your Solana RPC URL (use a private RPC for better performance)
RPC_URL=https://api.mainnet-beta.solana.com

# Your wallet's private key (base58 encoded)
PRIVATE_KEY=your_private_key_here

# Priority fee in lamports per compute unit (default: 2)
PRIORITY_FEE=2
```

3. Build the project:
```bash
cargo build --release
```

## Usage

Run the bot with an operation, token address, creator address, creator vault address, and amount:
```bash
# Buy tokens (amount in lamports)
cargo run --bin pumpfun-bot buy <token_address> <creator_address> <creator_vault_address> <amount_in_lamports> [--priority-fee <fee>]

# Sell tokens (amount can be raw or percentage)
cargo run --bin pumpfun-bot sell <token_address> <creator_address> <creator_vault_address> <amount> [--priority-fee <fee>]

# Examples
# Buy tokens with 0.01 SOL
cargo run --bin pumpfun-bot buy E5UbfmHh8sMVKBc1kSAHXQeFDyVJEZw7Tyd3o8FCpump t9QUGC7BnUKbD2SwqkAuk4qL76QoySyYwR6bPzFrPFZ 73Lrc9pHLuN59pBsBGL2oScGM2SHkbTZwZqTdVLPHqdf 10000000

# Sell 50% of tokens
cargo run --bin pumpfun-bot sell 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU t9QUGC7BnUKbD2SwqkAuk4qL76QoySyYwR6bPzFrPFZ 73Lrc9pHLuN59pBsBGL2oScGM2SHkbTZwZqTdVLPHqdf 50%

# Sell specific amount of tokens
cargo run --bin pumpfun-bot sell 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU t9QUGC7BnUKbD2SwqkAuk4qL76QoySyYwR6bPzFrPFZ 73Lrc9pHLuN59pBsBGL2oScGM2SHkbTZwZqTdVLPHqdf 1000

# Buy with custom priority fee
cargo run --bin pumpfun-bot buy 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU t9QUGC7BnUKbD2SwqkAuk4qL76QoySyYwR6bPzFrPFZ 73Lrc9pHLuN59pBsBGL2oScGM2SHkbTZwZqTdVLPHqdf 10000000 --priority-fee 5
```

## Transaction Structure

Each transaction consists of multiple instructions:

1. **Compute Budget Setup**
   - Sets compute unit limit:
     * Buy operations: 63,665 units
     * Sell operations: 34,848 units
   - Sets priority fee (configurable via --priority-fee flag)
     * Default for buy: 2 lamports per unit
     * Default for sell: 1.65 lamports per unit
   - Fee calculation example (buy operation with --priority-fee 2):
     * Priority fee: 63,665 units × 2 lamports = 127,330 lamports (≈ 0.000127 SOL)
     * Base transaction fee: ~0.000005 SOL
     * Total fee: ~0.000132 SOL
   - Fee calculation example (sell operation with --priority-fee 2):
     * Priority fee: 34,848 units × 2 lamports = 69,696 lamports (≈ 0.000070 SOL)
     * Base transaction fee: ~0.000005 SOL
     * Total fee: ~0.000075 SOL

2. **Associated Token Account**
   - Creates ATA if it doesn't exist (idempotent operation)

3. **Buy/Sell Operation**
   - Interacts with Pump.fun program
   - Handles token swaps with bonding curve
   - Includes automatic 1% swap fee handling

## Slippage and Retry Mechanism

The bot implements a sophisticated slippage handling system:

1. **Initial Slippage**
   - Buy operations start with 2% slippage by default
   - This accounts for both:
     * 1% protocol swap fee
     * 1% price movement buffer

2. **Automatic Retries**
   - If a transaction fails due to slippage:
     * The bot will automatically retry up to 3 times
     * Each retry reduces slippage by 5%
     * Example progression:
       - Attempt 1: 2.0% slippage
       - Attempt 2: 1.9% slippage
       - Attempt 3: 1.8% slippage
       - Attempt 4: 1.7% slippage

3. **Detailed Logging**
   - Each attempt shows:
     * Current slippage percentage
     * Number of tokens being requested
     * Protocol fee information
     * Retry status and new slippage values

This system helps ensure successful transactions while getting the best possible price for your trades.

## Error Handling and Retries

The bot includes sophisticated error handling:
- Automatic retry on transaction failures
- Fresh blockhash for each retry attempt
- 1-second delay between retries
- Detailed error reporting
- Balance and amount validation

## Getting Your Private Key

To get your private key from Phantom wallet:
1. Open Phantom wallet
2. Click the hamburger menu (three lines)
3. Go to Settings
4. Click "Export Private Key"
5. Enter your password
6. Copy the private key
7. Paste it as your PRIVATE_KEY in the .env file

Make sure to:
1. Fund your wallet with SOL
2. Never share your private key with anyone
3. Keep your .env file secure and never commit it to version control

## Customization

This bot is designed to be a template for building your own trading strategies. You can:

1. Modify the buy/sell logic in `src/bin/pump-bot.rs`
2. Add your own trading strategies
3. Implement additional features like:
   - Price monitoring
   - Automated trading
   - Multiple token support
   - Custom slippage settings

## Contributing

Feel free to submit issues and pull requests. For major changes, please open an issue first to discuss what you would like to change.

## Roadmap

We're actively working on the following features for our production launch:

### 🚀 Performance Optimizations
- **WebSocket Block Subscription**
  - Real-time new mint discovery via WebSocket
  - Eliminates polling overhead
  - Faster reaction time to new token launches

### 🔐 Advanced Transaction Handling
- **Parallel Signature Swarm**
  - Multiple hot keys signing identical transactions
  - Simultaneous broadcast for maximum success rate
  - First transaction lands, others expire safely
  - No slashing risk for failed attempts

### 💰 Smart Fee Management
- **Dynamic Compute Budget Tuning**
  - Real-time fee optimization using `getRecentPrioritizationFees`
  - Automatic CU-price adjustments
  - Cost-effective transaction processing
  - Adapts to network congestion

### 🐳 Deployment Solutions
- **Docker & Kubernetes Support**
  - Ready-to-use Docker image
  - Helm chart for easy deployment
  - Regional swarm deployment support
  - Scalable architecture

## Disclaimer

⚠️ **IMPORTANT SECURITY NOTICE**

This software is provided for educational purposes only. Use at your own risk.

- This project is NOT affiliated with, endorsed by, or connected to pump.fun in any way
- The authors provide NO WARRANTY of any kind, express or implied
- The software is provided "AS IS" without warranty of any kind
- The authors are not responsible for any financial losses incurred while using this software
- Always verify transactions and amounts before confirming
- Never share your private keys or .env file with anyone
- Always test with small amounts first
- Be aware that trading cryptocurrencies involves significant risk

By using this software, you acknowledge that you understand these risks and agree to use the software at your own risk.

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Buy/Sell Process Flow

The bot follows this process when buying or selling tokens:

1. **Parameter Validation**
   - Takes the token address, creator address, and creator vault address as inputs
   - Validates all parameters before proceeding
   - Creator vault is used directly as provided (no automatic derivation)

2. **Transaction Construction**
   - Builds the transaction with the appropriate instructions
   - Includes the provided creator vault in the transaction
   - Signs and sends the transaction to the network

This approach allows for flexibility, as the creator vault address may be derived differently for different tokens or protocol versions.

## Finding Creator Vault Addresses

To find the creator vault address for a token:

1. **Analyze a Transaction on Solscan:**
   - Find a transaction for buying/selling the token on Solscan
   - Look at the "Input Accounts" section under the Pump.fun instruction
   - Find the account labeled "#10 - Creator Vault" - this is the address you need
   - Example: `73Lrc9pHLuN59pBsBGL2oScGM2SHkbTZwZqTdVLPHqdf` for token creator `t9QUGC7BnUKbD2SwqkAuk4qL76QoySyYwR6bPzFrPFZ`

2. **Community Resources:**
   - Check community-maintained lists of token information
   - Some explorers may provide this data in token details
   
3. **A Note on Derivation:**
   - Creator vaults cannot reliably be derived programmatically
   - The protocol's derivation method is not publicly documented
   - Always use the actual vault address from transaction history

## TODO: Future Improvements

- **Creator Vault Derivation:**
  - Find a way to derive the creator vault address directly from the creator address
  - If derivation can be done locally, speed impact would be negligible
  - If derivation requires RPC calls, this could be costly performance-wise
  - Creator addresses themselves cannot be derived and must be provided
  - Automatic derivation would simplify the command by removing one parameter

- **Dynamic Fee Estimation:**
  - Implement automatic priority fee calculation based on network congestion
  - Optimize fee settings for different transaction types

- **Transaction Simulation:**
  - Add a dry-run option to simulate transactions without submitting them
  - Show expected tokens received/SOL returned before execution