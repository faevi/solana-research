use crate::constants::*;
use crate::errors::*;
use crate::instructions::round::inititialize_round;
use crate::instructions::round::inititialize_user_round;
use crate::state::*;
use crate::utils::now_ts;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(sol_to_deposit: u64,
    round_number: u64, _user_round_random_seed: u64)]
    pub struct JoinRound<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<UserRound>(),
        seeds = [USER_ROUND_PREFIX.as_bytes(), user.key().as_ref(), _user_round_random_seed.to_string().as_bytes()],
        bump,
    )]
    pub user_round: Account<'info, UserRound>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<Round>(),
        seeds = [ROUND_PREFIX.as_bytes(), round_number.to_string().as_bytes()],
        bump
    )]
    pub round: Box<Account<'info, Round>>,

    #[account(mut)]
    pub user: Signer<'info>,

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
    ctx: Context<'a, 'b, 'c, 'info, JoinRound<'info>>,
    sol_to_deposit: u64,
    round_number: u64,
    _user_round_random_seed: u64,
) -> Result<()> {
    let round = &mut ctx.accounts.round;
    let user: &mut Signer<'_> = &mut ctx.accounts.user;
    let round_setting: &mut Account<'_, RoundSetting> = &mut ctx.accounts.round_setting;
    let user_round: &mut Account<'_, UserRound> = &mut ctx.accounts.user_round;
    let system_program: &Program<'_, System> = &ctx.accounts.system_program;
    let round_vault = &ctx.accounts.round_txn_vault;

    require!(
        sol_to_deposit > round_setting.min_sol_to_deposit,
        ErrorCodes::LessThenMinDeposit,
    );

    if round.round_state == RoundState::Uninitialized {
        require!(
            round_setting.can_initialize_next_round &&
            round_setting.last_round_ends_at < now_ts().unwrap(),
            ErrorCodes::LastRoundIsOpen,
        );

        inititialize_round(
            round, 
            round_setting, 
            round_number
        ).unwrap();
    }    

    require!(
        round.round_state == RoundState::Open && round.round_ends_at > now_ts().unwrap(),
        ErrorCodes::RoundIsNotOpen,
    );

    inititialize_user_round(
        round, 
        round_setting, 
        user_round, 
        sol_to_deposit, 
        user, 
        round_vault, 
        system_program
    ).unwrap();

    Ok(())
}
