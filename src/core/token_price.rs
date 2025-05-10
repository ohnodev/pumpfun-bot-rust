use {
    anyhow::Result,
    solana_client::rpc_client::RpcClient,
    solana_sdk::pubkey::Pubkey,
    num_bigint::BigUint,
    num_traits::ToPrimitive,
};

pub struct TokenPriceInfo {
    pub token_supply: u64,
    pub token_price: u64,
}

pub struct BondingCurveData {
    pub real_token_reserves: BigUint,
    pub virtual_token_reserves: BigUint,
    pub virtual_sol_reserves: BigUint,
}

pub fn get_token_price_info(
    rpc_client: &RpcClient,
    bonding_curve: &Pubkey,
) -> Result<TokenPriceInfo> {
    let bonding_curve_account = rpc_client.get_account(bonding_curve)?;
    let bonding_curve_data = bonding_curve_account.data;
    
    // The bonding curve data structure is:
    // - First 8 bytes: discriminator
    // - Next 8 bytes: token supply
    // - Next 8 bytes: price in lamports
    let token_supply = u64::from_le_bytes(bonding_curve_data[8..16].try_into()?);
    let token_price = u64::from_le_bytes(bonding_curve_data[16..24].try_into()?);
    
    Ok(TokenPriceInfo {
        token_supply,
        token_price,
    })
}

pub fn get_bonding_curve_data(
    rpc_client: &RpcClient,
    bonding_curve: &Pubkey,
) -> Result<BondingCurveData> {
    let bonding_curve_account = rpc_client.get_account(bonding_curve)?;
    let data = bonding_curve_account.data;
    if data.len() < 24 {
        anyhow::bail!("Bonding curve account data too short");
    }
    let real_token_reserves = BigUint::from(u64::from_le_bytes(data[0..8].try_into()?));
    let virtual_token_reserves = BigUint::from(u64::from_le_bytes(data[8..16].try_into()?));
    let virtual_sol_reserves = BigUint::from(u64::from_le_bytes(data[16..24].try_into()?));
    Ok(BondingCurveData {
        real_token_reserves,
        virtual_token_reserves,
        virtual_sol_reserves,
    })
}

/// Accurate buy quote using the constant product formula
pub fn calculate_tokens_to_get_bonding_curve(
    sol_amount: u64,
    bonding_curve: &BondingCurveData,
    percentage: f64, // e.g. 0.99 for 1% slippage
) -> u64 {
    let sol_amount_big = BigUint::from(sol_amount);
    let virtual_sol_reserves = &bonding_curve.virtual_sol_reserves;
    let virtual_token_reserves = &bonding_curve.virtual_token_reserves;
    let new_virtual_sol_reserves = virtual_sol_reserves + &sol_amount_big;
    let invariant = virtual_sol_reserves * virtual_token_reserves;
    let new_virtual_token_reserves = &invariant / &new_virtual_sol_reserves;
    let tokens_to_buy = virtual_token_reserves - &new_virtual_token_reserves;
    // Apply percentage reduction for slippage/protocol fee
    let tokens_to_buy_f64 = tokens_to_buy.to_f64().unwrap_or(0.0);
    let final_tokens = (tokens_to_buy_f64 * percentage).floor() as u64;
    final_tokens
}

pub fn calculate_tokens_to_get(
    sol_amount: f64,
    token_supply: u64,
    token_price: u64,
) -> Result<u64, anyhow::Error> {
    // Convert to u128 for precise integer math
    let sol_amount_lamports = (sol_amount * 1_000_000_000.0) as u128;
    let token_supply_128 = token_supply as u128;
    let token_price_128 = token_price as u128;
    
    // Calculate tokens using integer math with overflow checks
    let tokens = sol_amount_lamports
        .checked_mul(token_supply_128)
        .ok_or_else(|| anyhow::anyhow!("Overflow in token calculation: sol_amount * token_supply"))?
        .checked_div(token_price_128)
        .ok_or_else(|| anyhow::anyhow!("Division by zero in token calculation"))?
        .checked_mul(99)
        .ok_or_else(|| anyhow::anyhow!("Overflow in fee calculation"))?
        .checked_div(100)
        .ok_or_else(|| anyhow::anyhow!("Division by zero in fee calculation"))?;

    // Convert back to u64, checking for overflow
    if tokens > u64::MAX as u128 {
        return Err(anyhow::anyhow!("Token amount exceeds u64::MAX"));
    }
    
    Ok(tokens as u64)
}

pub fn calculate_sol_to_get(
    token_amount: u64,
    token_supply: u64,
    token_price: u64,
) -> Result<u64, anyhow::Error> {
    // Convert to u128 for precise integer math
    let token_amount_128 = token_amount as u128;
    let token_supply_128 = token_supply as u128;
    let token_price_128 = token_price as u128;
    
    // Calculate SOL using integer math with overflow checks
    let sol_amount = token_amount_128
        .checked_mul(token_price_128)
        .ok_or_else(|| anyhow::anyhow!("Overflow in SOL calculation: token_amount * token_price"))?
        .checked_div(token_supply_128)
        .ok_or_else(|| anyhow::anyhow!("Division by zero in SOL calculation"))?;

    // Convert back to u64, checking for overflow
    if sol_amount > u64::MAX as u128 {
        return Err(anyhow::anyhow!("SOL amount exceeds u64::MAX"));
    }
    
    Ok(sol_amount as u64)
}