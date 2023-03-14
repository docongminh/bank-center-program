use crate::constants::{ ESCROW_PDA_SEED, CONFIG_PDA_SEED, VAULT_PDA_SEED };
use crate::state::*;
use crate::error::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount };

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [ESCROW_PDA_SEED, authority.key().as_ref(), mint_address.key().as_ref()],
        bump,
        token::mint = mint_address,
        token::authority = config_account
    )]
    pub escrow_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = authority,
        seeds = [CONFIG_PDA_SEED, authority.key().as_ref(), mint_address.key().as_ref()],
        bump,
        space = ConfigAccount::LEN
    )]
    pub config_account: Box<Account<'info, ConfigAccount>>,
    /// CHECK:
    #[account(mut,
        seeds=[VAULT_PDA_SEED, authority.key().as_ref(), mint_address.key().as_ref()],
        bump
    )]
    pub escrow_vault: AccountInfo<'info>,
    #[account(mut, token::mint = mint_address, token::authority = withdraw_wallet)]
    pub withdraw_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    #[account(constraint = withdraw_wallet.lamports() > 0 && withdraw_wallet.data_is_empty() @ CustomError::InvalidOwner)]
    pub withdraw_wallet: AccountInfo<'info>,
    pub mint_address: Account<'info, Mint>,
    #[account(mut, constraint = authority.lamports() > 0 && authority.data_is_empty())]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler_init<'info>(
    ctx: Context<'_, '_, '_, 'info, Initialize<'info>>,
    price: u64
) -> Result<()> {
    let config_account = &mut ctx.accounts.config_account;
    let vault_bump = *ctx.bumps.get("config_account").unwrap();
    config_account.escrow_bump = *ctx.bumps.get("escrow_token_account").unwrap();
    config_account.config_bump = *ctx.bumps.get("escrow_vault").unwrap();
    config_account.vault_bump = vault_bump;
    config_account.mint_address = ctx.accounts.mint_address.key();
    config_account.authority = ctx.accounts.authority.key();
    config_account.withdraw_token_account = ctx.accounts.withdraw_token_account.key();
    config_account.withdraw_wallet = ctx.accounts.withdraw_wallet.key();
    config_account.price = price;

    // escrow wallet hold sol
    ctx.accounts.create_native_account_vault(vault_bump)?;

    Ok(())
}

impl<'info> Initialize<'info> {
    fn create_native_account_vault(&self, vault_bump: u8) -> Result<()> {
        let authority_key = self.authority.key();
        let mint_address = self.mint_address.key();
        let vault_seed = &[
            &[
                VAULT_PDA_SEED,
                authority_key.as_ref(),
                mint_address.as_ref(),
                bytemuck::bytes_of(&vault_bump),
            ][..],
        ];
        // assign vault account for system program rather than itself program id
        create_account(
            self.authority.to_account_info(),
            self.escrow_vault.to_account_info(),
            0,
            vault_seed,
            self.system_program.to_account_info(),
            self.rent.clone()
        )?;
        Ok(())
    }
}