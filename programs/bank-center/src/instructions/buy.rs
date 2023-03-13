use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };

use crate::constants::{ CONFIG_PDA_SEED, ESCROW_PDA_SEED, VAULT_PDA_SEED };
use crate::error::CustomError;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Buy<'info> {
    #[account(mut,
    seeds=[ESCROW_PDA_SEED, authority.key().as_ref(), mint_address.key().as_ref()],
    bump = config_account.escrow_bump,
    token::mint = mint_address,
    token::authority = config_account,
    constraint = escrow_token_account.amount > 0 @ CustomError::InsufficientFunds
    )]
    pub escrow_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        seeds=[VAULT_PDA_SEED, authority.key().as_ref()],
        bump = config_account.vault_bump
    )]
    pub escrow_vault: AccountInfo<'info>,
    #[account(
        seeds = [CONFIG_PDA_SEED, authority.key().as_ref(), mint_address.key().as_ref()],
        bump = config_account.config_bump,
        has_one = authority
    )]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(mut, token::mint = mint_address)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    pub mint_address: Account<'info, Mint>,
    /// CHECK: This account use to check constraint
    pub authority: AccountInfo<'info>,

    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
    let total_price = 0;
    // Transfer SOL: Vault -> partner
    ctx.accounts.transfer_sol(total_price)?;
    // Transfer SPL: partner -> creator
    ctx.accounts.transfer_token(amount)?;

    Ok(())
}

pub fn buy_discount(ctx: Context<Buy>, amount: u64, _discount: u32) -> Result<()> {
    let total_price = 0;
    // Transfer SOL: Vault -> partner
    ctx.accounts.transfer_sol(total_price)?;
    // Transfer SPL: partner -> creator
    ctx.accounts.transfer_token(amount)?;
    Ok(())
}

impl<'info> Buy<'info> {
    fn transfer_sol(&self, amount: u64) -> Result<()> {
        transfer_native_to_account(
            self.buyer.to_account_info(),
            self.escrow_vault.to_account_info(),
            amount,
            self.system_program.to_account_info(),
            None
        )?;
        Ok(())
    }

    fn transfer_token(&self, amount: u64) -> Result<()> {
        let authority_key = self.authority.key();
        let mint_address = self.mint_address.key();
        let escrow_seed = &[
            &[
                ESCROW_PDA_SEED,
                authority_key.as_ref(),
                mint_address.as_ref(),
                bytemuck::bytes_of(&self.config_account.escrow_bump),
            ][..],
        ];
        // transfer token escrow_vault -> partner
        transfer_token_to_account(
            self.escrow_token_account.to_account_info(),
            self.buyer_token_account.to_account_info(),
            self.config_account.to_account_info(),
            amount,
            self.token_program.to_account_info(),
            Some(escrow_seed)
        )?;
        Ok(())
    }
}