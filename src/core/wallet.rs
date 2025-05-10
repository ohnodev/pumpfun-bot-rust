use {
    anyhow::Result,
    solana_client::rpc_client::RpcClient,
    solana_sdk::signature::{Keypair, Signer},
    std::env,
    crate::utils::utils::format_sol_amount,
};

pub fn load_wallet() -> Result<Keypair> {
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let keypair = Keypair::from_base58_string(&private_key);
    Ok(keypair)
}

pub fn print_wallet_info(rpc_client: &RpcClient, keypair: &Keypair) -> Result<()> {
    let balance = rpc_client.get_balance(&keypair.pubkey())?;
    println!("Wallet: {}", keypair.pubkey());
    println!("Balance: {}", format_sol_amount(balance));
    Ok(())
} 