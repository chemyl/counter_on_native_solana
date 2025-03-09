use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}
