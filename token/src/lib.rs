use processor::Processor;
use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    account_info::AccountInfo, 
    pubkey::Pubkey
};

mod processor;
mod instruction;

// 合约入口
entrypoint!(process_instruction);

fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
) -> ProgramResult{
    Processor::process(program_id,accounts,instruction_data)
}