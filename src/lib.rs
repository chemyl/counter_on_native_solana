use solana_program::entrypoint;

pub mod instructions;
pub mod processor;
pub mod state;

use processor::process_instruction;

entrypoint!(process_instruction);

#[cfg(test)]
mod test {
    use crate::state::counter_account::CounterAccount;

    use super::*;
    use borsh::BorshDeserialize;
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    use solana_sdk::account_info::AccountInfo;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        let mut increment_instruction_data: Vec<u8> = vec![0];
        let increment_value = 33u32;
        increment_instruction_data.extend_from_slice(&increment_value.to_le_bytes());

        let mut decrement_instruction_data: Vec<u8> = vec![1];
        let decrement_value = 10u32;
        decrement_instruction_data.extend_from_slice(&decrement_value.to_le_bytes());

        let mut update_instruction_data: Vec<u8> = vec![2];
        let reset_instruction_data: Vec<u8> = vec![3];

        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            increment_value
        );

        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            increment_value - decrement_value
        );

        let update_value = 33u32;

        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            33
        );

        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
