use std::{collections::hash_map::DefaultHasher, hash::{Hasher, Hash}};
use crate::{constants::*, errors::ErrorCodes};
use anchor_lang::{prelude::*, solana_program::clock};


pub fn create_transfer_signed<'a>(
    sender: &AccountInfo<'a>,
    receiver: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    amount: u64,
    seeds: &[&[&[u8]]],
) -> Result<()> {
    let ix =
        anchor_lang::solana_program::system_instruction::transfer(sender.key, receiver.key, amount);
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[sender.clone(), receiver.clone(), system_program.clone()],
        seeds,
    )?;
    Ok(())
}

pub fn create_transfer<'a>(
    sender: &AccountInfo<'a>,
    receiver: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    let ix =
        anchor_lang::solana_program::system_instruction::transfer(sender.key, receiver.key, amount);
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[sender.clone(), receiver.clone(), system_program.clone()],
    )?;

    Ok(())
}

pub fn now_ts() -> Result<u64> {
    //i64 -> u64 ok to unwrap
    Ok(clock::Clock::get()?.unix_timestamp.try_into().unwrap())
}

pub fn generate_encrypted_value_for_round(timestamp: u64, round_key: Pubkey) -> u64 {
    let mut hasher = DefaultHasher::new();
    timestamp.hash(&mut hasher);
    SALT_NUMBER.hash(&mut hasher);
    round_key.hash(&mut hasher);
    let random_value = hasher.finish();
    let encrypted_number = random_value  ^ SECRET_KEY;
    encrypted_number
}

pub fn generate_random_value() -> f64 {
    let now: u64 = now_ts().unwrap();
    let mut hasher = DefaultHasher::new();
    now.hash(&mut hasher);
    hasher.finish();
    let random_value = (hasher.finish() as f64) / (u64::MAX as f64);
    random_value
}
