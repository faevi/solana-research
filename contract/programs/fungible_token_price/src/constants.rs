use anchor_lang::prelude::Pubkey;
use solana_program::pubkey;

pub static FEE_RECEIVER: Pubkey = pubkey!("game1opLvC2iVBL2yw51yroNat3NVHf5h4Bk753qMwn");
pub const BASE_POINTS: u64 = 1e4 as u64;
pub const SECRET_KEY: u64 = 1e4 as u64;
pub const SALT_NUMBER: u64 = 1e4 as u64;
pub const ROUND_TXN_VAULT: &str = "round_txn_vault";
pub const ROUND_SETTING: &str = "round_setting";
pub const ROUND_PREFIX: &str = "round";
pub const USER_ROUND_PREFIX: &str = "user_round";
pub static YOLO_PROGRAM_ID: Pubkey = pubkey!("game1opLvC2iVBL2yw51yroNat3NVHf5h4Bk753qMwn");
pub static ADMIN_ID: Pubkey = pubkey!("6JgexLq1STiDE3MvjvnsZqdevnoHMTeaM7FrJRNt2Mrg");