use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::create_transfer_signed;
use crate::utils::now_ts;

use super::check_is_round_winner;

#[derive(Accounts)]
#[instruction()]
pub struct DrawUserTicket<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub round: Account<'info, Round>,

    #[account(mut)]
    pub user_round: Account<'info, UserRound>,

    #[account(
        mut,
        seeds = [ROUND_SETTING.as_bytes(),],
        bump
    )]
    pub round_setting: Account<'info, RoundSetting>,

    /// CHECK vault for sol
    #[account(
        mut,
        seeds = [ROUND_TXN_VAULT.as_bytes(),],
        bump
    )]
    pub round_txn_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, DrawUserTicket<'info>>,
) -> Result<()> {
    let user: &Signer<'_> = &ctx.accounts.user;
    let round_setting: &mut Account<'_, RoundSetting>  = &mut ctx.accounts.round_setting;
    let round: &mut Account<'_, Round> = &mut ctx.accounts.round;
    let system_program: &Program<'_, System> = &ctx.accounts.system_program;
    let round_vault = &ctx.accounts.round_txn_vault;
    let user_round: &mut Account<UserRound> = &mut ctx.accounts.user_round;

    require!(
        round.round_ends_at < now_ts().unwrap(),
        ErrorCodes::RoundIsOpen,
    );

    let is_winner = check_is_round_winner(user_round, round);

    require!(
        is_winner 
        && user.key() == user_round.user,
        ErrorCodes::UserIsNotWinner,
    );

    require!(
        round.round_state == RoundState::Open,
        ErrorCodes::RewardsAlreadyClaimed,
    );

    round.round_state = RoundState::Drawn;
    round.winner = user_round.user;

    //REWARDS TRANSFER:
    let program_seeds = &[
        ROUND_TXN_VAULT.as_bytes(),
        &[*ctx.bumps.get("round_txn_vault").unwrap()],
    ];

    let program_signer = &[&program_seeds[..]];

    create_transfer_signed(
        &round_vault.to_account_info().clone(),
        &user.to_account_info().clone(),
        &system_program.to_account_info().clone(),
        round.sol_amount,
        program_signer,
    )?;

    round_setting.completed_rounds +=1;
    user_round.last_transacted_at = now_ts().unwrap();
    round_setting.last_transacted_at = now_ts().unwrap();
    round.last_transacted_at = now_ts().unwrap();
    Ok(())
}
