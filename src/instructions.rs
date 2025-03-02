use borsh::BorshDeserialize;
use borsh_derive::BorshSerialize;
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UpdateArgs {
    pub value: u32,
}

pub enum CounterInstruction {
    Increment,
    Decrement,
    Update(UpdateArgs),
    Reset,
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match &variant {
            0 => Self::Increment,
            1 => Self::Decrement,
            2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
