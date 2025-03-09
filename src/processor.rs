use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{instructions::CounterInstruction, state::counter_account::CounterAccount};

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("program entry point");

    let instruction: CounterInstruction = CounterInstruction::unpack(instruction_data)?;

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstruction::Increment(args) => counter_account.counter += args,
        CounterInstruction::Decrement(args) => counter_account.counter -= args,
        CounterInstruction::Reset => counter_account.counter = 0,
        CounterInstruction::Update(args) => counter_account.counter = args.value,
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}
