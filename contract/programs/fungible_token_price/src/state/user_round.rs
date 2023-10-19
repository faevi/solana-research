use anchor_lang::prelude::*;

#[account]
pub struct UserRound {
    pub round: Pubkey,
    
    pub sol_deposited: u64,
    
    pub start_sol_position: u64,
   
    pub user: Pubkey,

    pub last_transacted_at: u64,

    pub deposited_at: u64,

    pub placeholder_one: Pubkey,
}
