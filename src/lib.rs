use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub mod instructions;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CounterAccount{
    pub counter:u32,
}