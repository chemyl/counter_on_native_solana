use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

use crate::state::update_account::UpdateArgs;

pub enum CounterInstruction {
    Increment(u32),
    Decrement(u32),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match &variant {
            0 => Self::Increment(u32::try_from_slice(rest).expect("msg")),
            1 => Self::Decrement(u32::try_from_slice(rest).unwrap()),
            2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
