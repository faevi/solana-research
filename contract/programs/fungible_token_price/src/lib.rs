use anchor_lang::prelude::*;
pub mod instructions;
use crate::instructions::*;
pub mod constants;
pub use constants::*;
pub mod errors;
pub mod state;
pub mod utils;
use crate::{state::*, utils::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod fungible_token_price {
    use super::*;

    pub fn test_instruction<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, TestInstruction<'info>>,
    ) -> Result<()> {
        instructions::test_instruction::handler(ctx)
    }
}
