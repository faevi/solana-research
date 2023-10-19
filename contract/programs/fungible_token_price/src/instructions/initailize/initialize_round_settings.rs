use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::now_ts;

#[derive(Accounts)]
#[instruction()]
pub struct InitializeRoundSettings<'info> {
    #[account(
        init_if_needed,
        seeds = [ROUND_SETTING.as_bytes(),],
        payer = user,
        space = 8 + std::mem::size_of::<RoundSetting>(),
        bump,
    )]
    pub round_setting: Account<'info, RoundSetting>,

    #[account(
        mut,
        constraint = user.key() == ADMIN_ID @ ErrorCodes::UserDoesntHaveAuthority,
    )]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
    
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, InitializeRoundSettings<'info>>,
    round_duration: u64,
    min_sol_to_deposit: u64,
    fee_percent: u64,
    completed_rounds: u64,
    can_initialize_next_round: bool,
) -> Result<()> {
    let round_setting = &mut ctx.accounts.round_setting;
    round_setting.can_initialize_next_round = can_initialize_next_round;
    round_setting.fee_percent = fee_percent;
    round_setting.completed_rounds = completed_rounds;
    round_setting.min_sol_to_deposit = min_sol_to_deposit;
    round_setting.round_duration = round_duration;
    round_setting.last_transacted_at = now_ts().unwrap();

    Ok(())
}
