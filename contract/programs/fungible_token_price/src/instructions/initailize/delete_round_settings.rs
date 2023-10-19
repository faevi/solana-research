use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::now_ts;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct DeleteRoundSettings<'info> {
    #[account(mut,
        close = user
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
    ctx: Context<'a, 'b, 'c, 'info, DeleteRoundSettings<'info>>,
) -> Result<()> {
    // CLOSING SECURELY THE USER VAULT PDA ACCOUNT

    **ctx
        .accounts
        .user
        .to_account_info()
        .try_borrow_mut_lamports()? += **ctx
        .accounts
        .round_setting
        .to_account_info()
        .try_borrow_lamports()?;
    **ctx
        .accounts
        .round_setting
        .to_account_info()
        .try_borrow_mut_lamports()? = 0;

    Ok(())
}
