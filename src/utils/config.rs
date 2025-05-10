use solana_program::pubkey::Pubkey;
use std::str::FromStr;

// Program IDs
pub const PUMP_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
pub const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";


// PDAs and Accounts
pub const GLOBAL_PDA: &str = "4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf";
pub const FEE_ACCOUNT: &str = "7hTckgnGnLQR6sdH7YkqFTAA7VwTfYFaZ6EhEsU3saCX";
pub const EVENT_AUTHORITY: &str = "Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1";

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