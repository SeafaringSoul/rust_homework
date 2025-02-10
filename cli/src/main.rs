use std::str::FromStr;

use borsh::{BorshDeserialize,BorshSerialize};
use solana_client::rpc_client::RpcClient;

use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::{read_keypair_file, Keypair}, signer::Signer, sysvar, transaction::Transaction};
use spl_associated_token_account_client::address::get_associated_token_address;

fn main() {
    println!("Hello, world!");
}

#[derive(BorshDeserialize,BorshSerialize)]
//合约指令
pub enum TokenInstruction{
    CreateToken {decimals: u8},
    Mint{amount: u64},
}
// 7w5TqYaipZvb8fb5vs3tCFuHNpZH9KaYAdWbSuFZPMFs

#[test]

fn test_fn(){
    let rpc_client = RpcClient::new("http://127.0.0.1:8899".to_string());
    let payer = read_keypair_file("/Users/lihao/.config/solana/id.json").expect("failed");
    let program_id = Pubkey::from_str("7w5TqYaipZvb8fb5vs3tCFuHNpZH9KaYAdWbSuFZPMFs").unwrap();

    let mint_account = Keypair::new();

    println!("mint_accout is {:?}",mint_account.pubkey().to_string());
    _ = create_token(&rpc_client,&program_id,&payer,&mint_account,&payer.pubkey(),6); 
    _ = mint(&rpc_client, &program_id, &payer, &mint_account,100_000_000);

}

fn create_token(
    rpc_client: &RpcClient,
    program_id: &Pubkey,
    payer:&Keypair,
    mint_account:&Keypair,
    mint_authority:&Pubkey,
    decimals:u8,
) -> Result<(),Box<dyn std::error::Error>>{
    let instruction_data = borsh::to_vec(&TokenInstruction::CreateToken { decimals }).unwrap();

    let accounts = vec![
        AccountMeta::new(mint_account.pubkey(), true),
        AccountMeta::new_readonly(*mint_authority, false),
        AccountMeta::new_readonly(payer.pubkey(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];
    let token_instrucftion = Instruction{
        program_id: *program_id,
        accounts,
        data:instruction_data
    };

    let lastest_blockhash = rpc_client.get_latest_blockhash()?;

    let tx = Transaction::new_signed_with_payer(
        &[token_instrucftion],
        Some(&payer.pubkey()), 
        &[payer,mint_account], 
        lastest_blockhash,
    );

    let r = rpc_client.send_and_confirm_transaction(&tx)?;

    println!("{:?}",r);

    println!("create token success");


    Ok(())
}


fn mint(
    rpc_client: &RpcClient,
    program_id: &Pubkey,
    payer:&Keypair,
    mint_account:&Keypair,
    amounts: u64,
) -> Result<(),Box<dyn std::error::Error>>{
    let ata = get_associated_token_address(
        &payer.pubkey(), 
        &mint_account.pubkey()
    );
    println!("ata is {:?}",ata.to_string());

    let instruction_data = borsh::to_vec(&TokenInstruction::Mint { amount: amounts } ).unwrap();

    let accounts = vec![
        AccountMeta::new(mint_account.pubkey(), true),
        AccountMeta::new(ata,false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(solana_sdk::system_program::id(), false),
        AccountMeta::new(spl_token::id(), false),
        AccountMeta::new(spl_associated_token_account::id(), false),

    ];

    let token_instuction = Instruction{
        program_id: *program_id,
        accounts,
        data: instruction_data,
    };

    // 发送交易
    let lastest_blockhash = rpc_client.get_latest_blockhash()?;

    let tx = Transaction::new_signed_with_payer(
        &[token_instuction],
        Some(&payer.pubkey()), 
        &[payer,mint_account], 
        lastest_blockhash,
    );
    let r = rpc_client.send_and_confirm_transaction(&tx)?;

    println!("{:?}",r);
    println!("mint token success");
    Ok(())
}