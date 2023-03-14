use crate::constants::{ CONFIG_PDA_SEED };
use crate::state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint };

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct UpdateConfig<'info> {
    #[account(mut,
    seeds=[CONFIG_PDA_SEED, authority.key().as_ref(), mint_address.key().as_ref()],
    bump = config_account.config_bump,
    has_one = authority
  )]
    pub config_account: Account<'info, ConfigAccount>,
    pub mint_address: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler_update_config<'info>(
    ctx: Context<'_, '_, '_, 'info, UpdateConfig<'info>>,
    price: u64
) -> Result<()> {
    let config_account = &mut ctx.accounts.config_account;
    config_account.price = price;
    Ok(())
}