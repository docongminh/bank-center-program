use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct Score {
    pub user_address: Pubkey,
    pub score: u32,
}

#[account]
#[derive(Default)]
pub struct LeaderBoard {
    pub scores: Vec<Score>,
}

impl LeaderBoard {
    pub const LEN: usize = 8; // todo
}

#[account]
pub struct ConfigAccount {
    pub escrow_bump: u8,
    pub config_bump: u8,
    pub vault_bump: u8,
    pub mint_address: Pubkey,
    pub ticket_token_account: Pubkey,
    pub withdraw_token_account: Pubkey,
    pub price: u32,
    pub withdraw_wallet: Pubkey,
    pub authority: Pubkey,
}

impl ConfigAccount {
    pub const LEN: usize = 8;
}