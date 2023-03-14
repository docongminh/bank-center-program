use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };

use crate::constants::{ CONFIG_PDA_SEED, VAULT_PDA_SEED, ESCROW_PDA_SEED };
use crate::error::CustomError;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
#[instruction()]
pub struct Withdraw<'info> {
    #[account(mut,
    seeds=[ESCROW_PDA_SEED, authority.key().as_ref(), mint_address.key().as_ref()],
    bump = config_account.escrow_bump,
    token::mint = mint_address,
    token::authority = config_account,
    constraint = escrow_token_account.amount > 0 @ CustomError::InsufficientFunds
  )]
    pub escrow_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        seeds = [CONFIG_PDA_SEED, authority.key().as_ref(), mint_address.key().as_ref()],
        bump = config_account.config_bump
    )]
    pub config_account: Account<'info, ConfigAccount>,
    /// CHECK:
    #[account(mut,
        seeds=[VAULT_PDA_SEED, authority.key().as_ref(), mint_address.key().as_ref()],
        bump = config_account.vault_bump
    )]
    pub escrow_vault: AccountInfo<'info>,
    #[account(mut, token::mint = mint_address)]
    pub withdraw_token_account: Account<'info, TokenAccount>,
    pub mint_address: Account<'info, Mint>,
    /// CHECK: This account use to check constraint
    pub authority: AccountInfo<'info>,
    #[account(constraint = config_authority.lamports() > 0 && config_authority.data_is_empty())]
    pub config_authority: Signer<'info>,
    #[account(constraint = master_authority.lamports() > 0 && master_authority.data_is_empty())]
    pub master_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn withdraw_token_with_amount<'info>(
    ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
    amount: u64
) -> Result<()> {
    //
    require_gte!(ctx.accounts.escrow_token_account.amount, amount, CustomError::InsufficientFunds);
    let authority = ctx.accounts.authority.key();
    let mint = ctx.accounts.mint_address.key();
    let config_bump = ctx.accounts.config_account.config_bump;
    let seeds = &[
        &[CONFIG_PDA_SEED, authority.as_ref(), mint.as_ref(), bytemuck::bytes_of(&config_bump)][..],
    ];
    transfer_token_to_account(
        ctx.accounts.escrow_token_account.to_account_info(),
        ctx.accounts.withdraw_token_account.to_account_info(),
        ctx.accounts.config_account.to_account_info(),
        amount,
        ctx.accounts.token_program.to_account_info(),
        Some(seeds)
    )?;

    Ok(())
}

pub fn withdraw_token_drain<'info>(ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    let mint = ctx.accounts.mint_address.key();
    let config_bump = ctx.accounts.config_account.config_bump;
    let seeds = &[
        &[CONFIG_PDA_SEED, authority.as_ref(), mint.as_ref(), bytemuck::bytes_of(&config_bump)][..],
    ];
    let amount = ctx.accounts.escrow_token_account.amount;
    transfer_token_to_account(
        ctx.accounts.escrow_token_account.to_account_info(),
        ctx.accounts.withdraw_token_account.to_account_info(),
        ctx.accounts.config_account.to_account_info(),
        amount,
        ctx.accounts.token_program.to_account_info(),
        Some(seeds)
    )?;

    Ok(())
}

pub fn withdraw_sol_with_amount<'info>(
    ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
    amount: u64
) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    let mint = ctx.accounts.mint_address.key();
    let vault_bump = ctx.accounts.config_account.vault_bump;
    let seeds = &[
        &[VAULT_PDA_SEED, authority.as_ref(), mint.as_ref(), bytemuck::bytes_of(&vault_bump)][..],
    ];
    transfer_token_to_account(
        ctx.accounts.escrow_token_account.to_account_info(),
        ctx.accounts.withdraw_token_account.to_account_info(),
        ctx.accounts.config_account.to_account_info(),
        amount,
        ctx.accounts.token_program.to_account_info(),
        Some(seeds)
    )?;
    Ok(())
}

pub fn withdraw_sol_drain<'info>(ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    let mint = ctx.accounts.mint_address.key();
    let vault_bump = ctx.accounts.config_account.vault_bump;
    let seeds = &[
        &[VAULT_PDA_SEED, authority.as_ref(), mint.as_ref(), bytemuck::bytes_of(&vault_bump)][..],
    ];
    let amount = ctx.accounts.escrow_vault.lamports();
    transfer_token_to_account(
        ctx.accounts.escrow_token_account.to_account_info(),
        ctx.accounts.withdraw_token_account.to_account_info(),
        ctx.accounts.config_account.to_account_info(),
        amount,
        ctx.accounts.token_program.to_account_info(),
        Some(seeds)
    )?;
    Ok(())
}