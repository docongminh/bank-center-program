pub mod utils;
pub mod state;
pub mod error;
pub mod constants;
pub mod instructions;

use crate::instructions::*;
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bank_center {
    use super::*;

    pub fn initialize<'info>(
        ctx: Context<'_, '_, '_, 'info, Initialize<'info>>,
        price: u32
    ) -> Result<()> {
        handler_init(ctx, price)?;
        Ok(())
    }

    pub fn deposit<'info>(
        ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
        amount: u64
    ) -> Result<()> {
        deposit_token(ctx, amount)?;
        Ok(())
    }

    pub fn buy<'info>(ctx: Context<'_, '_, '_, 'info, Deposit<'info>>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn buy_discount<'info>(
        ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
        amount: u64
    ) -> Result<()> {
        Ok(())
    }

    pub fn initialize_leaderboard(ctx: Context<LeaderboardData>) -> Result<()> {
        Ok(())
    }

    pub fn update_score(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}