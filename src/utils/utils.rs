use {
    solana_program::pubkey::Pubkey,
    spl_associated_token_account::get_associated_token_address,
};

pub fn get_token_account(wallet: &Pubkey, token_mint: &Pubkey) -> Pubkey {
    get_associated_token_address(wallet, token_mint)
}

pub fn format_sol_amount(lamports: u64) -> String {
    format!("{:.9} SOL", lamports as f64 / 1_000_000_000.0)
} 