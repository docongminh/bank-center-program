use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct LeaderboardData<'info> {
    #[account(init, payer = owner, space = LeaderBoard::LEN)]
    pub leaderboard_account: Account<'info, LeaderBoard>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn update_score<'info>(
    ctx: Context<'_, '_, '_, 'info, LeaderboardData<'info>>,
    user_address: Pubkey,
    score: u64
) {
    let leaderboard_account = &mut ctx.accounts.leaderboard_account;

    leaderboard_account.scores.push(Score {
        user_address: user_address,
        score,
    });
}

pub fn get_top_scores<'info>(
    ctx: Context<'_, '_, '_, 'info, LeaderboardData<'info>>
) -> Vec<Score> {
    let leaderboard_account = &mut ctx.accounts.leaderboard_account;
    let mut top_scores = leaderboard_account.scores.clone();
    top_scores.truncate(10); // Only return the top 10 scores
    top_scores
}