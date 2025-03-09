use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UpdateArgs {
    pub value: u32,
}
