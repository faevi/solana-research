use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::now_ts;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
    pub struct TestInstruction<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, TestInstruction<'info>>,
) -> Result<()> {
    

    Ok(())
}
