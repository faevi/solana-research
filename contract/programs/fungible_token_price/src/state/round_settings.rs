use anchor_lang::prelude::*;

#[account]
pub struct RoundSetting {
    pub completed_rounds: u64,

    pub total_sol_deposited: u64,

    pub total_fee_collected: u64,

    pub total_participants: u64,

    pub round_duration: u64,

    pub min_sol_to_deposit: u64,

    pub fee_percent: u64,
    
    pub can_initialize_next_round: bool,

   pub last_round_ends_at: u64,

    pub last_transacted_at: u64,

    pub placeholder_one: Pubkey,
}
