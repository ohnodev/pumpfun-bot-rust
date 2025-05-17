use {
    anyhow::Result,
    solana_sdk::pubkey::Pubkey,
    std::str::FromStr,
    solana_pump_bot::utils::config::{find_creator_vault_authority, pump_program_id},
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
    
    // First, let's check the creator vault authority derivation
    let (vault_authority, bump) = find_creator_vault_authority(&creator_pubkey);
    println!("\nStep 1: Creator Vault Authority Derivation");
    println!("Creator Vault Authority: {}", vault_authority);
    println!("Bump: {}", bump);
    
    // Let's try different seed combinations for the vault itself
    println!("\nTrying different seed combinations for creator vault:");
    
    // Option 1: Use the creator vault authority as a seed with "vault" prefix
    let (pda1, bump1) = Pubkey::find_program_address(
        &[b"vault", vault_authority.as_ref()],
        &pump_program_id(),
    );
    println!("\nOption 1: [\"vault\", creator_vault_authority]");
    println!("Derived: {} (bump: {})", pda1, bump1);
    println!("Matches expected: {}", pda1 == expected_vault_pubkey);
    
    // Option 2: Just creator with "vault" prefix
    let (pda2, bump2) = Pubkey::find_program_address(
        &[b"vault", creator_pubkey.as_ref()],
        &pump_program_id(),
    );
    println!("\nOption 2: [\"vault\", creator]");
    println!("Derived: {} (bump: {})", pda2, bump2);
    println!("Matches expected: {}", pda2 == expected_vault_pubkey);
    
    // Option 3: Just "creator_fee_vault" and creator
    let (pda3, bump3) = Pubkey::find_program_address(
        &[b"creator_fee_vault", creator_pubkey.as_ref()],
        &pump_program_id(),
    );
    println!("\nOption 3: [\"creator_fee_vault\", creator]");
    println!("Derived: {} (bump: {})", pda3, bump3);
    println!("Matches expected: {}", pda3 == expected_vault_pubkey);
    
    // Option 4: Two-stage derivation using creator vault authority
    let (pda4, bump4) = Pubkey::find_program_address(
        &[b"creator_fee_vault", vault_authority.as_ref()],
        &pump_program_id(),
    );
    println!("\nOption 4: [\"creator_fee_vault\", creator_vault_authority]");
    println!("Derived: {} (bump: {})", pda4, bump4);
    println!("Matches expected: {}", pda4 == expected_vault_pubkey);
    
    Ok(())
} 