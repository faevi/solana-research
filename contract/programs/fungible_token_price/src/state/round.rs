use anchor_lang::prelude::*;

#[account]
pub struct Round {
    pub round_state: RoundState,

    pub round_value: u64,
    
    pub started_at: u64,
    
    pub sol_amount: u64,

    pub fee_amount: u64,

    pub participants: u64,
    
    pub round_ends_at: u64,

    pub last_transacted_at: u64,

    pub winner: Pubkey,

    pub round_number: u64,

    pub placeholder_one: Pubkey,
}

#[derive(Clone, Copy, AnchorDeserialize, AnchorSerialize, PartialEq, Eq)]
pub enum RoundState {
    Uninitialized,
    Initialized,
    Open,
    Drawn,
}
