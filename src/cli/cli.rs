use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "A Solana bot for interacting with pump.fun tokens", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Buy tokens with SOL
    Buy {
        /// Token mint address (e.g., 8LbkTskkCx212Tm2LCuAeThZDVBQsxk8hKqkUxhspump)
        #[arg(help = "The pump.fun token address to buy")]
        token_address: String,
        /// Creator address of the token
        #[arg(help = "The creator address of the token")]
        creator_address: String,
        /// Creator vault address 
        #[arg(help = "The creator vault address (fee recipient)")]
        creator_vault_address: String,
        /// Amount in lamports (1 SOL = 1,000,000,000 lamports)
        #[arg(help = "Amount of SOL to spend in lamports (e.g., 16837852 for 0.016837852 SOL)")]
        amount: u64,
        /// Priority fee in lamports per compute unit (default: 2)
        #[arg(short, long, help = "Priority fee in lamports per compute unit")]
        priority_fee: Option<u64>,
    },
    /// Sell tokens for SOL
    Sell {
        /// Token mint address (e.g., 8LbkTskkCx212Tm2LCuAeThZDVBQsxk8hKqkUxhspump)
        #[arg(help = "The pump.fun token address to sell")]
        token_address: String,
        /// Creator address of the token
        #[arg(help = "The creator address of the token")]
        creator_address: String,
        /// Creator vault address 
        #[arg(help = "The creator vault address (fee recipient)")]
        creator_vault_address: String,
        /// Amount to sell (e.g., "50%" for half of balance, or "31000" for 31,000 tokens)
        #[arg(help = "Amount to sell: either a percentage of your balance (e.g., '50%') or a specific number of tokens (e.g., '31000' for 31,000 tokens)")]
        amount: String,
        /// Priority fee in lamports per compute unit (default: 1.65)
        #[arg(short, long, help = "Priority fee in lamports per compute unit")]
        priority_fee: Option<u64>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cli_parsing() {
        // Test buy command
        let args = vec!["pumpfun-bot", "buy", "token123", "creator123", "vault123", "1000000"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Buy { token_address, creator_address, creator_vault_address, amount, priority_fee } => {
                assert_eq!(token_address, "token123");
                assert_eq!(creator_address, "creator123");
                assert_eq!(creator_vault_address, "vault123");
                assert_eq!(amount, 1000000);
                assert_eq!(priority_fee, None);
            }
            _ => panic!("Expected Buy command"),
        }

        // Test buy command with priority fee
        let args = vec!["pumpfun-bot", "buy", "token123", "creator123", "vault123", "1000000", "--priority-fee", "3"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Buy { token_address, creator_address, creator_vault_address, amount, priority_fee } => {
                assert_eq!(token_address, "token123");
                assert_eq!(creator_address, "creator123");
                assert_eq!(creator_vault_address, "vault123");
                assert_eq!(amount, 1000000);
                assert_eq!(priority_fee, Some(3));
            }
            _ => panic!("Expected Buy command"),
        }

        // Test sell command with percentage
        let args = vec!["pumpfun-bot", "sell", "token123", "creator123", "vault123", "50%"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Sell { token_address, creator_address, creator_vault_address, amount, priority_fee } => {
                assert_eq!(token_address, "token123");
                assert_eq!(creator_address, "creator123");
                assert_eq!(creator_vault_address, "vault123");
                assert_eq!(amount, "50%");
                assert_eq!(priority_fee, None);
            }
            _ => panic!("Expected Sell command"),
        }

        // Test sell command with specific token amount
        let args = vec!["pumpfun-bot", "sell", "token123", "creator123", "vault123", "30000"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Sell { token_address, creator_address, creator_vault_address, amount, priority_fee } => {
                assert_eq!(token_address, "token123");
                assert_eq!(creator_address, "creator123");
                assert_eq!(creator_vault_address, "vault123");
                assert_eq!(amount, "30000");
                assert_eq!(priority_fee, None);
            }
            _ => panic!("Expected Sell command"),
        }

        // Test sell command with priority fee
        let args = vec!["pumpfun-bot", "sell", "token123", "creator123", "vault123", "30000", "--priority-fee", "3"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Sell { token_address, creator_address, creator_vault_address, amount, priority_fee } => {
                assert_eq!(token_address, "token123");
                assert_eq!(creator_address, "creator123");
                assert_eq!(creator_vault_address, "vault123");
                assert_eq!(amount, "30000");
                assert_eq!(priority_fee, Some(3));
            }
            _ => panic!("Expected Sell command"),
        }
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn test_priority_fee_must_be_integer() {
        // This should fail because priority fee should be an integer
        let args = vec!["pumpfun-bot", "buy", "token123", "creator123", "vault123", "1000000", "--priority-fee", "3.5"];
        let _cli = Cli::parse_from(args);
    }
} 