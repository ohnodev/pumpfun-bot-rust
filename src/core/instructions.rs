use {
    anyhow::Result,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    spl_token::ID as TOKEN_PROGRAM_ID,
    crate::utils::config::{
        global_pda,
        fee_account,
        event_authority,
        pump_program_id,
        associated_token_program_id,
    },
};

// Instruction discriminators
pub const BUY_DISCRIMINATOR: [u8; 8] = [0x66, 0x06, 0x3d, 0x12, 0x01, 0xda, 0xeb, 0xea];
pub const SELL_DISCRIMINATOR: [u8; 8] = [0x33, 0xe6, 0x85, 0xa4, 0x01, 0x7f, 0x83, 0xad];

pub fn create_buy_instruction(
    buyer: &Pubkey,
    token_mint: &Pubkey,
    token_account: &Pubkey,
    bonding_curve: &Pubkey,
    associated_bonding_curve: &Pubkey,
    token_amount: u64,
    max_sol_cost: u64,
) -> Result<Instruction> {
    let mut data = vec![];
    data.extend_from_slice(&BUY_DISCRIMINATOR);
    data.extend_from_slice(&token_amount.to_le_bytes());
    data.extend_from_slice(&max_sol_cost.to_le_bytes());

    let accounts = vec![
        AccountMeta::new_readonly(global_pda(), false),           // Global PDA
        AccountMeta::new(fee_account(), false),                   // Fee account
        AccountMeta::new_readonly(*token_mint, false),           // Token mint
        AccountMeta::new(*bonding_curve, false),                 // Bonding curve
        AccountMeta::new(*associated_bonding_curve, false),      // Associated bonding curve
        AccountMeta::new(*token_account, false),                 // User's token account
        AccountMeta::new(*buyer, true),                          // User (signer)
        AccountMeta::new_readonly(system_program::id(), false),  // System program
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),    // Token program
        AccountMeta::new_readonly(sysvar::rent::id(), false),    // Rent sysvar
        AccountMeta::new(event_authority(), false),              // Event authority
        AccountMeta::new_readonly(pump_program_id(), false),     // Program ID
    ];

    Ok(Instruction {
        program_id: pump_program_id(),
        accounts,
        data,
    })
}

pub fn create_sell_instruction(
    seller: &Pubkey,
    token_mint: &Pubkey,
    token_account: &Pubkey,
    bonding_curve: &Pubkey,
    associated_bonding_curve: &Pubkey,
    token_amount: u64,
    min_sol_output: u64,
) -> Result<Instruction> {
    let mut data = vec![];
    data.extend_from_slice(&SELL_DISCRIMINATOR);
    data.extend_from_slice(&token_amount.to_le_bytes());
    data.extend_from_slice(&min_sol_output.to_le_bytes());

    let accounts = vec![
        AccountMeta::new_readonly(global_pda(), false),           // Global PDA
        AccountMeta::new(fee_account(), false),                   // Fee account
        AccountMeta::new_readonly(*token_mint, false),           // Token mint
        AccountMeta::new(*bonding_curve, false),                 // Bonding curve
        AccountMeta::new(*associated_bonding_curve, false),      // Associated bonding curve
        AccountMeta::new(*token_account, false),                 // User's token account
        AccountMeta::new(*seller, true),                         // User (signer)
        AccountMeta::new_readonly(system_program::id(), false),  // System program
        AccountMeta::new_readonly(associated_token_program_id(), false), // Associated Token Program
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),    // Token program
        AccountMeta::new(event_authority(), false),              // Event authority
        AccountMeta::new_readonly(pump_program_id(), false),     // Program ID
    ];

    Ok(Instruction {
        program_id: pump_program_id(),
        accounts,
        data,
    })
} 