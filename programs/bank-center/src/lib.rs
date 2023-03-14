pub mod utils;
pub mod state;
pub mod error;
pub mod constants;
pub mod instructions;

use crate::instructions::*;
use anchor_lang::prelude::*;

declare_id!("3iUNmf8zTvnmFTCQyVpo3Kthz8Q1L7uPRvnskijdCJF2");

#[program]
pub mod bank_center {
    use super::*;

    pub fn init_instruction<'info>(
        ctx: Context<'_, '_, '_, 'info, Initialize<'info>>,
        price: u64
    ) -> Result<()> {
        handler_init(ctx, price)?;
        Ok(())
    }

    pub fn update_config_instruction<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateConfig<'info>>,
        price: u64
    ) -> Result<()> {
        handler_update_config(ctx, price)?;
        Ok(())
    }

    pub fn deposit_instruction<'info>(
        ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
        amount: u64
    ) -> Result<()> {
        deposit_token(ctx, amount)?;
        Ok(())
    }

    pub fn buy_instruction<'info>(
        ctx: Context<'_, '_, '_, 'info, Buy<'info>>,
        amount: u64
    ) -> Result<()> {
        buy(ctx, amount)?;
        Ok(())
    }

    pub fn buy_discount_instruction<'info>(
        ctx: Context<'_, '_, '_, 'info, Buy<'info>>,
        amount: u64,
        discount: u64
    ) -> Result<()> {
        buy_discount(ctx, amount, discount)?;
        Ok(())
    }

    pub fn withdraw_token_instruction<'info>(
        ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
        amount: u64
    ) -> Result<()> {
        withdraw_token_with_amount(ctx, amount)?;
        Ok(())
    }

    pub fn drain_withdraw_token_instruction<'info>(
        ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>
    ) -> Result<()> {
        withdraw_token_drain(ctx)?;
        Ok(())
    }

    pub fn withdraw_sol_instruction<'info>(
        ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
        amount: u64
    ) -> Result<()> {
        withdraw_sol_with_amount(ctx, amount)?;
        Ok(())
    }

    pub fn drain_withdraw_sol_instruction<'info>(
        ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>
    ) -> Result<()> {
        withdraw_sol_drain(ctx)?;
        Ok(())
    }
}