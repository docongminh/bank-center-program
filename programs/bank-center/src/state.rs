use anchor_lang::prelude::*;
#[account]
pub struct ConfigAccount {
    pub escrow_bump: u8,
    pub config_bump: u8,
    pub vault_bump: u8,
    pub mint_address: Pubkey,
    pub ticket_token_account: Pubkey,
    pub withdraw_token_account: Pubkey,
    pub price: u64,
    pub withdraw_wallet: Pubkey,
    pub authority: Pubkey,
}

impl ConfigAccount {
    pub const LEN: usize = 8 // 
    + 1 * 3 // u8
    + 5 * 32 // Pubkey
    + 1 * 8;

}