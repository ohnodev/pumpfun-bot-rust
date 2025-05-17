use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use spl_associated_token_account::get_associated_token_address_with_program_id;

// Program IDs
pub const PUMP_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
pub const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";


// PDAs and Accounts
pub const GLOBAL_PDA: &str = "4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf";
pub const FEE_ACCOUNT: &str = "CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM";
pub const EVENT_AUTHORITY: &str = "Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1";
pub const SYSVAR_RENT: &str = "SysvarRent111111111111111111111111111111111";
pub const WSOL_MINT: &str = "So11111111111111111111111111111111111111112"; // Native SOL wrapped as SPL token

// Helper functions to convert string constants to Pubkey
pub fn pump_program_id() -> Pubkey {
    Pubkey::from_str(PUMP_PROGRAM_ID).unwrap()
}

pub fn token_program_id() -> Pubkey {
    Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap()
}

pub fn associated_token_program_id() -> Pubkey {
    Pubkey::from_str(ASSOCIATED_TOKEN_PROGRAM_ID).unwrap()
}

pub fn global_pda() -> Pubkey {
    Pubkey::from_str(GLOBAL_PDA).unwrap()
}

pub fn fee_account() -> Pubkey {
    Pubkey::from_str(FEE_ACCOUNT).unwrap()
}

pub fn event_authority() -> Pubkey {
    Pubkey::from_str(EVENT_AUTHORITY).unwrap()
}

pub fn sysvar_rent() -> Pubkey {
    Pubkey::from_str(SYSVAR_RENT).unwrap()
}

pub fn wsol_mint() -> Pubkey {
    Pubkey::from_str(WSOL_MINT).unwrap()
}

// PDA derivation functions
pub fn find_bonding_curve_pda(token_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"bonding-curve", token_mint.as_ref()],
        &pump_program_id(),
    )
}

pub fn find_associated_bonding_curve_pda(token_mint: &Pubkey, bonding_curve: &Pubkey) -> (Pubkey, u8) {
    // Find the associated bonding curve for a given mint and bonding curve.
    // This uses the standard ATA derivation.
    Pubkey::find_program_address(
        &[
            bonding_curve.as_ref(),  // First: bonding_curve
            token_program_id().as_ref(),  // Second: TOKEN_PROGRAM_ID
            token_mint.as_ref(),  // Third: mint
        ],
        &associated_token_program_id(),
    )
}

// Updated function to derive the creator_vault_authority PDA
pub fn find_creator_vault_authority(coin_creator: &Pubkey) -> (Pubkey, u8) {
    println!("\n🔍 Step 1: Deriving creator vault authority for: {}", coin_creator);
    
    // This matches JS exactly:
    // const creatorVaultSeed = Buffer.from('creator_vault');  // UTF-8 encoded
    // const [coinCreatorVaultAuthority] = PublicKey.findProgramAddressSync(
    //   [creatorVaultSeed, coinCreator.toBuffer()],
    //   PUMP_AMM_PROGRAM_ID
    // );
    
    // Use explicit UTF-8 encoding to match JavaScript's Buffer.from()
    let creator_vault_utf8 = "creator_vault".as_bytes(); // UTF-8 encoded, same as JS Buffer.from()
    
    let creator_vault_pda = Pubkey::find_program_address(
        &[creator_vault_utf8, coin_creator.as_ref()],
        &pump_program_id(),
    );
    
    println!("Creator vault authority: {} (bump: {})", creator_vault_pda.0, creator_vault_pda.1);
    creator_vault_pda
}

// Updated helper to find the creator vault ATA
pub fn find_creator_vault(coin_creator: &Pubkey) -> Pubkey {
    // Step 1: Get the creator vault authority PDA
    let (authority, bump) = find_creator_vault_authority(coin_creator);
    println!("🔍 Step 2: Deriving creator vault ATA using authority: {} (bump: {})", authority, bump);
    
    // Step 2: This matches the JS exactly:
    // JS: getAssociatedTokenAddressSync(
    //   WSOL_TOKEN_ACCOUNT,           // mint is FIRST in JS
    //   coinCreatorVaultAuthority,    // owner is SECOND in JS
    //   true                          // allowOwnerOffCurve 
    // )
    let wsol = wsol_mint();
    println!("Using WSOL mint: {}", wsol);
    
    // Use the proper SPL associated token address function with the correct parameter order
    // In JavaScript: (mint, owner, allowOwnerOffCurve)
    // In Rust: get_associated_token_address_with_program_id(wallet_address, token_mint_address, token_program_id)
    let ata = get_associated_token_address_with_program_id(
        &authority,     // wallet owner (the PDA from step 1)
        &wsol,          // token mint (WSOL)
        &token_program_id() // token program ID
    );
    
    println!("Final creator vault ATA: {}", ata);
    ata
} 