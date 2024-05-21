use anchor_lang::prelude::*;

pub mod errors;
pub mod helpers;
pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("HHtpy5cez4guhvwoXVCZzo8EUce6ouJyXaxZ7r9CVR24");

#[program]
pub mod dex {
    use super::*;

    pub fn initialize_dex(ctx: Context<InitializeDexConfiguration>, fee: f64) -> Result<()> {
        instructions::initialize_dex_configuration(ctx, fee)
    }

    pub fn create_liquidity_pool(ctx: Context<CreateLiquidityPool>) -> Result<()> {
        instructions::create_liquidity_pool(ctx)
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_one: u64,
        amount_two: u64,
    ) -> Result<()> {
        instructions::add_liquidity(ctx, amount_one, amount_two)
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, shares: u64) -> Result<()> {
        instructions::remove_liquidity(ctx, shares)
    }

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        instructions::swap(ctx, amount)
    }
}
