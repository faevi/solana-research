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
}
