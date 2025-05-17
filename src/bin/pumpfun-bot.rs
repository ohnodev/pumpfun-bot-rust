use {
    anyhow::Result,
    clap::Parser,
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signer::Signer,
        system_program,
    },
    spl_associated_token_account::instruction::create_associated_token_account_idempotent,
    std::str::FromStr,
    solana_pump_bot::{
        cli::cli::{Cli, Commands},
        core::{
            instructions::{create_buy_instruction, create_sell_instruction},
            token_price::{calculate_sol_to_get, get_token_price_info, get_bonding_curve_data, calculate_tokens_to_get_bonding_curve},
            transaction::{send_transaction, create_compute_budget_instructions},
            wallet::{load_wallet, print_wallet_info},
        },
        utils::{
            config::{token_program_id, find_bonding_curve_pda, find_associated_bonding_curve_pda, 
                     global_pda, fee_account, event_authority, pump_program_id},
            utils::get_token_account,
        },
    },
    std::time::Instant,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n🚀 Solana Pump Bot");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Buy { token_address, creator_address, creator_vault_address, amount, priority_fee } => {
            execute_buy(token_address, creator_address, creator_vault_address, amount, priority_fee).await?
        },
        Commands::Sell { token_address, creator_address, creator_vault_address, amount, priority_fee } => {
            execute_sell(token_address, creator_address, creator_vault_address, amount, priority_fee).await?
        },
    }

    Ok(())
}

async fn execute_buy(token_mint: String, creator_address: String, creator_vault_address: String, amount_in_lamports: u64, priority_fee: Option<u64>) -> Result<()> {
    let start_time = Instant::now();
    dotenv::dotenv().ok();

    // Initialize RPC client
    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set");
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Load wallet
    let keypair = load_wallet()?;
    print_wallet_info(&rpc_client, &keypair)?;

    // Parse token mint address
    let token_mint = Pubkey::from_str(&token_mint)?;
    println!("\n🟢 Buying token: {}", token_mint);
    
    // Parse creator address
    let token_creator = Pubkey::from_str(&creator_address)?;
    println!("👤 Token creator: {}", token_creator);
    
    // Parse creator vault address
    let creator_vault = Pubkey::from_str(&creator_vault_address)?;
    println!("💰 Creator vault: {}", creator_vault);

    // Get or create associated token account
    let associated_token_account = get_token_account(&keypair.pubkey(), &token_mint);

    // Derive PDAs dynamically
    let (bonding_curve, _) = find_bonding_curve_pda(&token_mint);
    let (associated_bonding_curve, _) = find_associated_bonding_curve_pda(&token_mint, &bonding_curve);

    // Get token price info
    let price_info = get_token_price_info(&rpc_client, &bonding_curve)?;
    let price_per_token = price_info.token_price as f64 / price_info.token_supply as f64;
    println!("📊 Token Price: {} SOL", price_per_token);
    
    // Convert input amount from lamports to SOL
    let sol_amount = amount_in_lamports as f64 / 1_000_000_000.0;
    println!("💰 Amount: {} SOL", sol_amount);
    
    // Get bonding curve data (for accurate buy quote)
    let curve_data = get_bonding_curve_data(&rpc_client, &bonding_curve)?;
    
    let mut retries = 3;
    let mut attempt = 1;
    let mut slippage = 0.98; // Start with 2% slippage
    
    loop {
        println!("\n🔄 Attempt {} of {}", attempt, retries + 1);
        
        // Calculate tokens to get using bonding curve math with current slippage
        let tokens_to_get = calculate_tokens_to_get_bonding_curve(
            amount_in_lamports,
            &curve_data,
            slippage,
        );
        
        println!("📈 Expected tokens: {}", tokens_to_get as f64 / 1_000_000.0);
        let mut instructions = vec![];
        // Add compute budget instructions
        instructions.extend(create_compute_budget_instructions(63665, priority_fee.unwrap_or(2)));
        // Create ATA
        instructions.push(
            create_associated_token_account_idempotent(
                &keypair.pubkey(),
                &keypair.pubkey(),
                &token_mint,
                &token_program_id(),
            ),
        );
        
        // Use the provided creator vault instead of deriving it
        // No longer using: let creator_vault = find_creator_vault(&token_creator);
        
        // Log all account addresses for verification
        println!("\n📋 Instruction accounts for verification:");
        println!("#1 - Global PDA: {}", global_pda());
        println!("#2 - Fee account: {}", fee_account());
        println!("#3 - Token mint: {}", token_mint);
        println!("#4 - Bonding curve: {}", bonding_curve);
        println!("#5 - Associated bonding curve: {}", associated_bonding_curve);
        println!("#6 - User token account: {}", associated_token_account);
        println!("#7 - User (signer): {}", keypair.pubkey());
        println!("#8 - System program: {}", system_program::id());
        println!("#9 - Token program: {}", token_program_id());
        println!("#10 - Creator vault: {}", creator_vault);
        println!("#11 - Event authority: {}", event_authority());
        println!("#12 - Program ID: {}", pump_program_id());
        
        // Add buy instruction
        instructions.push(create_buy_instruction(
            &keypair.pubkey(),
            &token_mint,
            &associated_token_account,
            &bonding_curve,
            &associated_bonding_curve,
            &creator_vault,
            tokens_to_get,
            amount_in_lamports,
        )?);
        let result = send_transaction(&rpc_client, &keypair, instructions).await;
        match result {
            Ok(_) => {
                println!("\n✅ Transaction completed in {:.2?}", start_time.elapsed());
                break Ok(());
            }
            Err(e) => {
                if retries > 1 {
                    slippage = slippage * 0.95; // Reduce slippage by 5% for next attempt
                    retries -= 1;
                    attempt += 1;
                } else {
                    println!("\n❌ Transaction failed: {}", e);
                    break Err(e);
                }
            }
        }
    }
}

async fn execute_sell(token_mint: String, creator_address: String, creator_vault_address: String, amount_str: String, priority_fee: Option<u64>) -> Result<()> {
    let start_time = Instant::now();
    dotenv::dotenv().ok();

    // Initialize RPC client
    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set");
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Load wallet
    let keypair = load_wallet()?;
    print_wallet_info(&rpc_client, &keypair)?;

    // Parse token mint address
    let token_mint = Pubkey::from_str(&token_mint)?;
    println!("\n🔴 Selling token: {}", token_mint);
    
    // Parse creator address
    let token_creator = Pubkey::from_str(&creator_address)?;
    println!("👤 Token creator: {}", token_creator);
    
    // Parse creator vault address
    let creator_vault = Pubkey::from_str(&creator_vault_address)?;
    println!("💰 Creator vault: {}", creator_vault);

    // Get token account
    let token_account = get_token_account(&keypair.pubkey(), &token_mint);

    // Get token balance
    let token_balance = rpc_client.get_token_account_balance(&token_account)?;
    let total_token_amount = token_balance.amount.parse::<u64>()?;
    println!("💰 Balance: {} tokens", total_token_amount as f64 / 1_000_000.0);

    if total_token_amount == 0 {
        println!("❌ No tokens to sell!");
        return Ok(());
    }

    // Calculate sell amount based on input
    let sell_amount = if amount_str.ends_with('%') {
        let percentage = amount_str.trim_end_matches('%').parse::<f64>()?;
        if percentage <= 0.0 || percentage > 100.0 {
            return Err(anyhow::anyhow!("Percentage must be between 0 and 100"));
        }
        let amount = (total_token_amount as f64 * (percentage / 100.0)) as u64;
        println!("📊 Selling {}% ({} tokens)", percentage, amount as f64 / 1_000_000.0);
        amount
    } else {
        let amount = 1_000_000 * amount_str.parse::<u64>()?;
        if amount > total_token_amount {
            return Err(anyhow::anyhow!("Sell amount exceeds token balance"));
        }
        println!("📊 Selling {} tokens", amount as f64 / 1_000_000.0);
        amount
    };

    // Derive PDAs dynamically
    let (bonding_curve, _) = find_bonding_curve_pda(&token_mint);
    let (associated_bonding_curve, _) = find_associated_bonding_curve_pda(&token_mint, &bonding_curve);

    // Get token price info
    let price_info = get_token_price_info(&rpc_client, &bonding_curve)?;
    let price_per_token = price_info.token_price as f64 / price_info.token_supply as f64;
    println!("📈 Token Price: {} SOL", price_per_token);

    // Calculate expected SOL amount
    let expected_sol = calculate_sol_to_get(
        sell_amount,
        price_info.token_supply,
        price_info.token_price,
    )?;
    println!("💰 Expected return: {} SOL", expected_sol as f64 / 1_000_000_000.0);

    let mut instructions = vec![];

    // Add compute budget instructions
    instructions.extend(create_compute_budget_instructions(34848, priority_fee.unwrap_or(2)));

    // Use the provided creator vault instead of deriving it
    // No longer using: let creator_vault = find_creator_vault(&token_creator);
    
    // Log all account addresses for verification
    println!("\n📋 Instruction accounts for verification:");
    println!("#1 - Global PDA: {}", global_pda());
    println!("#2 - Fee account: {}", fee_account());
    println!("#3 - Token mint: {}", token_mint);
    println!("#4 - Bonding curve: {}", bonding_curve);
    println!("#5 - Associated bonding curve: {}", associated_bonding_curve);
    println!("#6 - User token account: {}", token_account);
    println!("#7 - User (signer): {}", keypair.pubkey());
    println!("#8 - System program: {}", system_program::id());
    println!("#9 - Token program: {}", token_program_id());
    println!("#10 - Creator vault: {}", creator_vault);
    println!("#11 - Event authority: {}", event_authority());
    println!("#12 - Program ID: {}", pump_program_id());

    // Add sell instruction
    instructions.push(create_sell_instruction(
        &keypair.pubkey(),
        &token_mint,
        &token_account,
        &bonding_curve,
        &associated_bonding_curve,
        &creator_vault,
        sell_amount,
        0,
    )?);

    // Send transaction
    send_transaction(&rpc_client, &keypair, instructions).await?;
    println!("\n✅ Transaction completed in {:.2?}", start_time.elapsed());

    Ok(())
} 