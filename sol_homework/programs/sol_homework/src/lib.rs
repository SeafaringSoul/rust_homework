use anchor_lang::prelude::*;

declare_id!("HpK9xd5DiAbxiU1BogshSZYiJCtC7rTyACzFB4AUitYq");

#[program]
pub mod sol_homework {
    use super::*;

    pub fn create(ctx: Context<Create>, init_string: String) -> Result<()> {
        //获取可变引用的字符串账户
        let string_account = &mut ctx.accounts.string_account;
        //将字符串写入字符串账户
        string_account.data = init_string;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_string: String) -> Result<()> {
        //获取可变引用的字符串账户
        let string_account = &mut ctx.accounts.string_account;
        //将字符串写入字符串账户
        string_account.data = new_string;
        Ok(())
    }

    pub fn delete(ctx: Context<Delete>) -> Result<()> {
        //获取字符串账户
        let string_account = &mut ctx.accounts.string_account;
        //将字符串账户数据清空
        string_account.data = String::new();
        Ok(())
    }

    pub fn read(ctx: Context<Read>) -> Result<()> {
        //获取字符串账户
        let string_account = &ctx.accounts.string_account;
        // 使用 msg! 宏打印字符串账户的数据
        msg!("查询的数据是: {}", string_account.data);
        Ok(())
    }
}

#[account]
pub struct StringAccount {
    // 使用字符串存储数据
    pub data: String,
}

// 使用 #[derive(Accounts)] 宏为 create 指令定义所需的账户结构
#[derive(Accounts)]
// 'info 是一个生命周期参数，确保账户引用的有效性
pub struct Create<'info> {
    // 使用 #[account] 宏标记 string_account 账户
    // init 表示要初始化这个账户
    // payer = user 表示由 user 账户来支付初始化账户所需的费用
    // space = 8 + 100 表示为账户分配的存储空间，8 字节用于账户头，100 字节用于存储字符串
    #[account(init, payer = user, space = 8 + 100)]
    pub string_account: Account<'info, StringAccount>,
    // 使用 #[account(mut)] 标记 user 账户为可变的
    #[account(mut)]
    // Signer<'info> 表示这个账户需要签名，即操作的发起者
    pub user: Signer<'info>,
    // 引入 Solana 的系统程序，创建新账户时需要和系统程序交互
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    // 使用 #[account(mut)] 标记 string_account 账户为可变的
    #[account(mut)]
    pub string_account: Account<'info, StringAccount>,
    // 使用 #[account(signer)] 标记 user 账户为签名账户
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    // 使用 #[account(mut)] 标记 string_account 账户为可变的
    #[account(mut)]
    pub string_account: Account<'info, StringAccount>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Read<'info> {
    // 使用 #[account] 标记 string_account 账户
    #[account()]
    pub string_account: Account<'info, StringAccount>,
}
