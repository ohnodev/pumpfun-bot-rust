use {
    anyhow::Result,
    solana_sdk::pubkey::Pubkey,
    std::str::FromStr,
    solana_pump_bot::utils::config::{find_creator_vault, find_creator_vault_authority},
};

fn main() -> Result<()> {
    // Values from the transaction example
    let token_creator = "t9QUGC7BnUKbD2SwqkAuk4qL76QoySyYwR6bPzFrPFZ";
    let expected_creator_vault = "73Lrc9pHLuN59pBsBGL2oScGM2SHkbTZwZqTdVLPHqdf";
    
    println!("Testing creator vault derivation");
    println!("===============================");
    println!("Token Creator: {}", token_creator);
    println!("Expected Creator Vault: {}", expected_creator_vault);
    
    // Parse addresses
    let creator_pubkey = Pubkey::from_str(token_creator)?;
    let expected_vault_pubkey = Pubkey::from_str(expected_creator_vault)?;
    
    // First derive the creator vault authority
    let (vault_authority, bump) = find_creator_vault_authority(&creator_pubkey);
    println!("\nStep 1: Deriving Creator Vault Authority");
    println!("Creator Vault Authority: {}", vault_authority);
    println!("Bump: {}", bump);
    
    // Then derive the creator vault ATA
    let derived_vault = find_creator_vault(&creator_pubkey);
    println!("\nStep 2: Deriving Creator Vault ATA");
    println!("Derived Creator Vault: {}", derived_vault);
    
    // Compare
    if derived_vault == expected_vault_pubkey {
        println!("\n✅ SUCCESS: Derived vault matches expected vault!");
    } else {
        println!("\n❌ ERROR: Derived vault does NOT match expected vault!");
        println!("Expected: {}", expected_vault_pubkey);
        println!("Derived:  {}", derived_vault);
    }
    
    Ok(())
} 