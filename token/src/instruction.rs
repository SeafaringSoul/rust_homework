use borsh::{BorshDeserialize,BorshSerialize};

#[derive(BorshDeserialize,BorshSerialize)]
//合约指令
pub enum TokenInstruction{
    CreateToken {decimals: u8},
    Mint{amount: u64},
}