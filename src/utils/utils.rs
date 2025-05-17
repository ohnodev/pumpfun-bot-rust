use {
    solana_program::pubkey::Pubkey,
    spl_associated_token_account::get_associated_token_address_with_program_id,
};
use crate::utils::config::token_program_id;

pub fn get_token_account(wallet: &Pubkey, token_mint: &Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(
        wallet, 
        token_mint, 
        &token_program_id()
    )
}

pub fn format_sol_amount(lamports: u64) -> String {
    format!("{:.9} SOL", lamports as f64 / 1_000_000_000.0)
} 