use {
    anyhow::Result,
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        compute_budget::ComputeBudgetInstruction,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
    std::time::Instant,
    tokio::time::sleep,
    std::time::Duration,
};

pub async fn send_transaction(
    rpc_client: &RpcClient,
    keypair: &Keypair,
    instructions: Vec<solana_sdk::instruction::Instruction>,
) -> Result<()> {
    let tx_start = Instant::now();
    let mut retries = 3;
    let mut last_error = None;

    while retries > 0 {
        // Get fresh blockhash for each attempt
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&keypair.pubkey()),
            &[keypair],
            recent_blockhash,
        );

        match rpc_client.send_and_confirm_transaction_with_spinner_and_config(
            &transaction,
            CommitmentConfig::confirmed(),
            solana_client::rpc_config::RpcSendTransactionConfig {
                skip_preflight: false,
                preflight_commitment: Some(CommitmentConfig::confirmed().commitment),
                max_retries: Some(1), // We handle retries ourselves
                min_context_slot: None,
                encoding: None,
            },
        ) {
            Ok(signature) => {
                let confirmation_time = tx_start.elapsed();
                println!("Transaction successful! Signature: {}", signature);
                println!("Transaction confirmed in: {:.2?}", confirmation_time);
                println!("You can view the transaction at: https://solscan.io/tx/{}", signature);
                return Ok(());
            }
            Err(err) => {
                last_error = Some(err);
                retries -= 1;
                if retries > 0 {
                    println!("Transaction failed, retrying... ({} attempts left)", retries);
                    sleep(Duration::from_secs(1)).await; // Use tokio::time::sleep instead
                }
            }
        }
    }

    // If we get here, all retries failed
    if let Some(err) = last_error {
        println!("Transaction failed after all retries with error: {}", err);
        // Try to get simulation logs
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&keypair.pubkey()),
            &[keypair],
            recent_blockhash,
        );
        if let Ok(sim_result) = rpc_client.simulate_transaction(&transaction) {
            println!("\nSimulation logs:");
            if let Some(logs) = sim_result.value.logs {
                for log in logs {
                    println!("{}", log);
                }
            }
        }
    }

    Ok(())
}

pub fn create_compute_budget_instructions(compute_units: u32, priority_fee: u64) -> Vec<solana_sdk::instruction::Instruction> {
    let mut instructions = vec![];
    instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(compute_units));
    instructions.push(ComputeBudgetInstruction::set_compute_unit_price(priority_fee));
    instructions
} 