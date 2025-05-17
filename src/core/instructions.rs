use {
    anyhow::Result,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    spl_token::ID as TOKEN_PROGRAM_ID,
    crate::utils::config::{
        global_pda,
        fee_account,
        event_authority,
        pump_program_id,
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
    creator_vault_ata: &Pubkey,
    token_amount: u64,
    max_sol_cost: u64,
) -> Result<Instruction> {
    let mut data = vec![];
    data.extend_from_slice(&BUY_DISCRIMINATOR);
    data.extend_from_slice(&token_amount.to_le_bytes());
    data.extend_from_slice(&max_sol_cost.to_le_bytes());

    // Accounts must be in exactly this order to match the Pump.fun program expectations
    let accounts = vec![
        AccountMeta::new_readonly(global_pda(), false),           // #1 - Global PDA
        AccountMeta::new(fee_account(), false),                   // #2 - Fee account
        AccountMeta::new_readonly(*token_mint, false),            // #3 - Token mint
        AccountMeta::new(*bonding_curve, false),                  // #4 - Bonding curve
        AccountMeta::new(*associated_bonding_curve, false),       // #5 - Associated bonding curve
        AccountMeta::new(*token_account, false),                  // #6 - User's token account
        AccountMeta::new(*buyer, true),                           // #7 - User (signer)
        AccountMeta::new_readonly(system_program::id(), false),   // #8 - System program
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),       // #9 - Token program
        AccountMeta::new(*creator_vault_ata, false),              // #10 - Creator vault
        AccountMeta::new(event_authority(), false),               // #11 - Event authority
        AccountMeta::new_readonly(pump_program_id(), false),      // #12 - Program ID
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
    creator_vault_ata: &Pubkey,
    token_amount: u64,
    min_sol_output: u64,
) -> Result<Instruction> {
    let mut data = vec![];
    data.extend_from_slice(&SELL_DISCRIMINATOR);
    data.extend_from_slice(&token_amount.to_le_bytes());
    data.extend_from_slice(&min_sol_output.to_le_bytes());

    // Accounts must be in exactly this order to match the Pump.fun program expectations
    // NOTE: Sell has a different account order than Buy!
    let accounts = vec![
        AccountMeta::new_readonly(global_pda(), false),           // #1 - Global PDA
        AccountMeta::new(fee_account(), false),                   // #2 - Fee account
        AccountMeta::new_readonly(*token_mint, false),            // #3 - Token mint
        AccountMeta::new(*bonding_curve, false),                  // #4 - Bonding curve
        AccountMeta::new(*associated_bonding_curve, false),       // #5 - Associated bonding curve
        AccountMeta::new(*token_account, false),                  // #6 - User's token account
        AccountMeta::new(*seller, true),                          // #7 - User (signer)
        AccountMeta::new_readonly(system_program::id(), false),   // #8 - System program
        AccountMeta::new(*creator_vault_ata, false),              // #9 - Creator vault (different from buy!)
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),       // #10 - Token program (different from buy!)
        AccountMeta::new(event_authority(), false),               // #11 - Event authority
        AccountMeta::new_readonly(pump_program_id(), false),      // #12 - Program ID
    ];

    Ok(Instruction {
        program_id: pump_program_id(),
        accounts,
        data,
    })
} 