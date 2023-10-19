use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCodes {
    #[msg("RoundIsNotOpen")]
    RoundIsNotOpen,
    #[msg("RoundIsOpen")]
    RoundIsOpen,
    #[msg("UserDoesntHaveAuthority")]
    UserDoesntHaveAuthority,
    #[msg("LessThenMinDeposit")]
    LessThenMinDeposit,
    #[msg("LastRoundIsOpen")]
    LastRoundIsOpen,
    #[msg("RoundPdaIsIncorrect")]
    RoundPdaIsIncorrect,
    #[msg("WrongRoundNumber")]
    WrongRoundNumber,
    #[msg("UserIsNotWinner")]
    UserIsNotWinner,
    #[msg("RewardsAlreadyClaimed")]
    RewardsAlreadyClaimed,
}