use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };

use crate::constants::{ CONFIG_PDA_SEED, ESCROW_PDA_SEED };
use crate::error::CustomError;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    #[account(mut,
    seeds=[ESCROW_PDA_SEED, authority.key().as_ref(), mint_address.key().as_ref()],
    bump = config_account.escrow_bump,
    token::mint = mint_address,
    token::authority = config_account
  )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [CONFIG_PDA_SEED, authority.key().as_ref(), mint_address.key().as_ref()],
        bump = config_account.config_bump
    )]
    pub config_account: Account<'info, ConfigAccount>,
    pub mint_address: Account<'info, Mint>,
    /// CHECK: this account use to verify escrow_token_account and config_account seed
    pub authority: AccountInfo<'info>,
    #[account(mut,
    token::mint=mint_address,
    token::authority = depositor,
    constraint = depositor_token_account.amount >= amount @ CustomError::InsufficientFunds)]
    pub depositor_token_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = depositor.lamports() > 0 && depositor.data_is_empty())]
    pub depositor: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn deposit_token<'info>(
    ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
    amount: u64
) -> Result<()> {
    transfer_token_to_account(
        ctx.accounts.depositor_token_account.to_account_info(),
        ctx.accounts.escrow_token_account.to_account_info(),
        ctx.accounts.depositor.to_account_info(),
        amount,
        ctx.accounts.token_program.to_account_info(),
        None
    )?;
    Ok(())
}