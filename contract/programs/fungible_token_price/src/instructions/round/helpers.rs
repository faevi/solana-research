use crate::{
    constants::*,
    state::{
        user_round::UserRound,
        Round, round_settings::RoundSetting, RoundState,
    }, utils::{now_ts, generate_encrypted_value_for_round, create_transfer},
};
use anchor_lang::prelude::*;

pub fn check_is_round_winner(user_round: &Account<UserRound>, round: &Account<Round>) -> bool {
    let decrypted_value = round.round_value ^ SECRET_KEY;
    let round_win_value = decrypted_value.checked_rem(round.sol_amount).unwrap();

    if round_win_value > user_round.start_sol_position
        && round_win_value
            < user_round
                .start_sol_position
                .checked_add(user_round.sol_deposited)
                .unwrap()
    {
        return true;
    }

    false
}

pub fn inititialize_round<'info>(
    round: &mut Account<'info, Round>,
    round_setting:  &mut Account<'info, RoundSetting>,
    round_number: u64,
) -> Result<()> {
    round.round_state = RoundState::Open;
    round.round_ends_at = now_ts()
        .unwrap()
        .checked_add(round_setting.round_duration)
        .unwrap();
    round_setting.last_round_ends_at = round.round_ends_at; 
    round.round_number = round_number;
    round.started_at = now_ts().unwrap();
    round.round_value = generate_encrypted_value_for_round(round.started_at, round.key());
    
    Ok(())
}

pub fn inititialize_user_round<'info>(
    round: &mut Account<'info, Round>,
    round_setting:  &mut Account<'info, RoundSetting>,
    user_round: &mut Account<'info, UserRound>,
    sol_to_deposit: u64,
    user: &mut Signer<'info>,
    round_vault: &AccountInfo<'info>,
    system_program: &Program<'info, System>,
) -> Result<()> {
    let fee_amount = sol_to_deposit
        .checked_mul(round_setting.fee_percent)
        .unwrap()
        .checked_div(BASE_POINTS)
        .unwrap();

    let sol_in_round = sol_to_deposit.checked_sub(fee_amount).unwrap();

    create_transfer(
        &user,
        &round_vault,
        &system_program.to_account_info().clone(),
        sol_to_deposit,
    )?;

    user_round.start_sol_position = round.sol_amount;
    round.sol_amount = round.sol_amount.checked_add(sol_in_round).unwrap();
    round.fee_amount = round.fee_amount.checked_add(fee_amount).unwrap();
    round.last_transacted_at = now_ts().unwrap();

    user_round.last_transacted_at = now_ts().unwrap();
    user_round.deposited_at = now_ts().unwrap();
    user_round.sol_deposited = sol_in_round;
    user_round.round = round.key();
    user_round.user = user.key();

    round_setting.total_fee_collected = round_setting
        .total_fee_collected
        .checked_add(fee_amount)
        .unwrap();
    round_setting.total_sol_deposited = round_setting
        .total_sol_deposited
        .checked_add(sol_in_round)
        .unwrap();
    round_setting.last_transacted_at = now_ts().unwrap();

    round_setting.total_participants = round_setting.total_participants.checked_add(1).unwrap();
    round.participants = round.participants.checked_add(1).unwrap();
    
    Ok(())
}


