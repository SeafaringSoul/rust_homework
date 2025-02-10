use solana_program::{
    // 账户管理相关模块
    account_info::{next_account_info, AccountInfo}, 
    // 入口点相关模块
    entrypoint::{self, ProgramResult}, 
    // 日志输出
    msg, 
    // 用于调用其他程序的指令
    program::{invoke, invoke_signed}, 
    // 用于解析账户数据
    program_pack::Pack, 
    // 公钥类型
    pubkey::Pubkey, 
    // 租金计算
    rent::Rent, 
    // 系统指令
    system_instruction, 
    // 获取系统变量
    sysvar::Sysvar
};

// SPL Token 相关模块
use spl_token::{instruction::{initialize_mint, mint_to}, state::Mint};

// 解析合约指令
use crate::instruction::TokenInstruction;
use borsh::BorshDeserialize;

// 处理合约逻辑的结构体
pub struct Processor;

impl Processor {
    /// 处理合约指令
    pub fn process(
        program_id: &Pubkey,      // 当前合约程序的 ID
        accounts: &[AccountInfo], // 参与该指令的账户列表
        instruction_data: &[u8],  // 指令数据
    ) -> ProgramResult {
        // 解析指令数据，将其转换为 TokenInstruction 枚举类型
        let instruction = TokenInstruction::try_from_slice(instruction_data)?;

        // 匹配不同的指令类型
        match instruction {
            // 如果是创建 Token 指令
            TokenInstruction::CreateToken { decimals } => {
                Self::create_token(accounts, decimals)
            }
            // 如果是 Mint Token 指令
            TokenInstruction::Mint { amount } => {
                Self::mint_token(accounts, amount)
            }
        }
    }

    /// 创建一个新的 SPL Token
    fn create_token(accounts: &[AccountInfo], decimals: u8) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // 解析账户
        let mint_account = next_account_info(accounts_iter)?;   // Token Mint 账户
        let mint_authority = next_account_info(accounts_iter)?; // Token Mint 账户的所有者
        let payer = next_account_info(accounts_iter)?;          // 交易支付者
        let rent_sysvar = next_account_info(accounts_iter)?;    // 租金账户
        let system_program = next_account_info(accounts_iter)?; // 系统程序
        let token_program = next_account_info(accounts_iter)?;  // SPL Token 程序

        // 打印 Mint 账户地址
        msg!("Creating mint account...");
        msg!("Mint account is {}", mint_account.key);

        // 创建 Mint 账户的指令
        let create_account_ix = &system_instruction::create_account(
            payer.key,                                 // 由支付者支付创建费用
            mint_account.key,                          // 创建的账户
            (Rent::get()?).minimum_balance(Mint::LEN), // 计算创建账户所需的最小租金
            Mint::LEN as u64,                          // 账户大小
            token_program.key,                         // 所属的程序（SPL Token Program）
        );

        // 需要的账户信息
        let account_infos_ix = &[
            mint_account.clone(),
            payer.clone(),
            system_program.clone(),
            token_program.clone(),
        ];

        // 调用系统程序创建账户
        invoke(create_account_ix, account_infos_ix)?;

        // 初始化 Mint 账户的指令
        let mint_init_ix = &initialize_mint(
            token_program.key,  // SPL Token 程序
            mint_account.key,   // Mint 账户
            mint_authority.key, // Mint 账户的所有者
            None,               // 是否支持冻结（None 代表不支持）
            decimals,           // Token 的小数位数
        )?;
        msg!("Initializing mint account...");

        // 执行初始化 Mint 账户的指令
        invoke_signed(
            mint_init_ix,
            &[
                mint_account.clone(),
                rent_sysvar.clone(),
                token_program.clone(),
                mint_authority.clone(),
            ],
            &[],
        )?;

        msg!("SPL Token Mint created successfully");
        Ok(())
    }

    /// Mint（铸造）新的 Token
    fn mint_token(accounts: &[AccountInfo], amount: u64) -> ProgramResult {

        let accounts_iter = &mut accounts.iter();

        let mint_account = next_account_info(accounts_iter)?;
        let associated_token_account = next_account_info(accounts_iter)?;
        let rent_sysvar = next_account_info(accounts_iter)?;
        let payer = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let associated_token_program = next_account_info(accounts_iter)?;
        
        msg!("ATA is :{:?}",associated_token_account);

        if associated_token_account.lamports() == 0{
            msg!("create associated token account...");
            let create_ata_ix = &spl_associated_token_account::instruction::create_associated_token_account(
                payer.key,
                payer.key, 
                mint_account.key, 
                token_program.key,
            );

            invoke(
                create_ata_ix, 
                &[
                    payer.clone(),
                    associated_token_account.clone(),
                    mint_account.clone(),
                    system_program.clone(),
                    token_program.clone(),
                    rent_sysvar.clone(),
                    associated_token_program.clone(),
                ],
            )?;
        }
        
        msg!("Minting {} tokens to ata...",amount);

        let mint_ix = &mint_to(
            token_program.key, 
            mint_account.key, 
            associated_token_account.key, 
            payer.key, 
            &[payer.key], 
            amount
        )?;

        invoke(
            mint_ix, 
            &[
                mint_account.clone(),
                payer.clone(),
                associated_token_account.clone(),
                token_program.clone(),
            ],
        )?;
        msg!("Tokens Minted to ata success");
        Ok(())
    }
}
