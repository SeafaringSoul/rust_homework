use crate::{instruction::SocialInstruction, state::*};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, 
    borsh1::try_from_slice_unchecked, 
    clock::Clock, 
    entrypoint::ProgramResult, 
    program::invoke_signed, 
    program_error::ProgramError, 
    program_pack::Pack, 
    rent::Rent, 
    system_instruction, 
    sysvar::Sysvar,
};
use solana_program::{msg, pubkey::Pubkey};
use spl_token::state::Mint;

// use std::str::FromStr;

pub struct Processor;

const MAX_FOLLOWER_SIZE: u16 = 200;
const PUBKEY_SIZE: usize = 32;
const USER_PROFILE_SIZE : usize = 6;
const U16_SIZE : usize = 2;

const USER_POST_SIZE : usize = 8;



impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = SocialInstruction::try_from_slice(instruction_data)?;

        match instruction {
            SocialInstruction::InitializeUser{seed_type} => {
                Self::initialize_user(program_id, accounts, seed_type)
            }
            SocialInstruction::FollowUser { user_to_follow } => {
                Self::follow_user(accounts, user_to_follow)
            }
            SocialInstruction::UnfollowUser{ user_to_unfollow } => {
                Self::unfollow_user(accounts, user_to_unfollow)
            }
            SocialInstruction::QueryFollowers => {
                Self::query_followers(accounts)
            }
            SocialInstruction::PostContent { content } => {
                Self::post_content(program_id, accounts, content)
            }
            SocialInstruction::QueryPosts => {
                Self::query_posts(accounts)
            }
        }
    }

    fn initialize_user(program_id: &Pubkey, accounts: &[AccountInfo], seed_type : String) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account_info = next_account_info(account_info_iter)?;
        let pda_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        let seed = match seed_type.as_str() {
            "profile" => "profile",
            "post" => "post",
            _ => return Err(ProgramError::InvalidArgument),
        };

        msg!("seed:{:?}", seed);

        let (pda, bump_seed) = Pubkey::find_program_address(&[user_account_info.key.as_ref(), seed.as_bytes()], program_id);

        msg!("pda:{:?}", pda);

        msg!("pda_account.key.clone():{:?}", pda_account.key.clone());

        if pda != pda_account.key.clone() {
            return Err(ProgramError::InvalidArgument);
        }
    
        let rent = Rent::get()?;

        let space = match seed_type.as_str() {
            "profile" => computer_profile_number(MAX_FOLLOWER_SIZE),
            "post" => {
                USER_POST_SIZE
            },
            _ => return Err(ProgramError::InvalidArgument),
        };

        
        let lamports = rent.minimum_balance( space);
    
        // 创建账户指令
        let create_account_ix = system_instruction::create_account(
            user_account_info.key,
            &pda,
            lamports,
            space as u64,
            program_id,
        );
    
        // 通过带签名的调用创建账户
        invoke_signed(
            &create_account_ix,
            &[user_account_info.clone(), pda_account.clone(), system_program.clone()],
            &[&[user_account_info.key.as_ref(), seed.as_bytes(), &[bump_seed]]],
        )?;

        match seed_type.as_str() {
            "profile" => {
                let user_profile = UserProfile::new();
                user_profile.serialize(&mut *pda_account.try_borrow_mut_data()?)?;
            },
            "post" => {
                let user_post = UserPost::new();
                user_post.serialize(&mut *pda_account.try_borrow_mut_data()?)?;
            },
            _ => return Err(ProgramError::InvalidArgument),
        };

        

        msg!("User initialized successfully.");
        Ok(())
    }

    fn follow_user(accounts: &[AccountInfo], user_to_follow: Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account_info = next_account_info(account_info_iter)?;

        msg!("user_account_info {:?}", user_account_info);

        #[warn(unused_assignments)]
        let mut size : usize = 0;
        {
            let data = &user_account_info.data.borrow();

            let len = &data[..U16_SIZE];
            let pubkey_count = bytes_to_u16(len).unwrap();
            size = computer_profile_number(pubkey_count);
        }

        msg!("size is {}", size);

        let mut user_profile : UserProfile = UserProfile::try_from_slice(&user_account_info.data.borrow()[..size])?;

        msg!("user_profile {:?}", user_profile);
        user_profile.follow(user_to_follow);

        msg!("user_profile follow is {:?}", user_profile);

        user_profile.serialize(&mut *user_account_info.try_borrow_mut_data()?)?;
        msg!("User followed successfully.");
        Ok(())
    }

    fn unfollow_user(accounts: &[AccountInfo], user_to_unfollow: Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account_info = next_account_info(account_info_iter)?;

        #[warn(unused_assignments)]
        let mut size : usize = 0;
        {
            let data = &user_account_info.data.borrow();

            let len = &data[..U16_SIZE];
            let pubkey_count = bytes_to_u16(len).unwrap();

            msg!("len_number is {}", U16_SIZE);

            size = computer_profile_number(pubkey_count);
        }

        let mut user_profile = UserProfile::try_from_slice(&user_account_info.data.borrow()[..size])?;
        user_profile.unfollow(user_to_unfollow);

        user_profile.serialize(&mut &mut user_account_info.data.borrow_mut()[..])?;
        msg!("User unfollowed successfully.");
        Ok(())
    }

    fn query_followers(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account_info = next_account_info(account_info_iter)?;

        msg!("pda:{:?}", user_account_info.key);

        let len = &user_account_info.data.borrow()[..U16_SIZE];
        let pubkey_count = bytes_to_u16(len).unwrap();

        msg!("len_number is {}", U16_SIZE);

        let size = computer_profile_number(pubkey_count);

        let user = try_from_slice_unchecked::<UserProfile>(&user_account_info.data.borrow()).unwrap();
        msg!("Followers user:");
        msg!(" user - {:?}", &user);

        let user_profile : UserProfile = UserProfile::try_from_slice(&user_account_info.data.borrow()[..size])?;

        msg!("Followers:");
        msg!(" - {:?}", &user_profile);
        Ok(())
    }

    fn post_content(program_id: &Pubkey, accounts: &[AccountInfo], content: String) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account_info = next_account_info(account_info_iter)?;
        let pda_account = next_account_info(account_info_iter)?;
        let post_pda_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        let clock = Clock::get()?;
        let timestamp = clock.unix_timestamp as u64;

        let mut user_post = try_from_slice_unchecked::<UserPost>(&pda_account.data.borrow())?;
        user_post.add_post();

        user_post.serialize(&mut *pda_account.try_borrow_mut_data()?)?;

        let count = user_post.get_count();

        let (pda, bump_seed) = Pubkey::find_program_address(&[user_account_info.key.as_ref(), "post".as_bytes(), &[count as u8]], program_id);

        msg!("pda:{:?}", pda);
    
        let rent = Rent::get()?;

        let post = Post::new(content, timestamp);

        let space = borsh::to_vec(&post).unwrap().len();

        msg!("space:{:?}", space);

        let lamports = rent.minimum_balance( space);
    
        // 创建账户指令
        let create_account_ix = system_instruction::create_account(
            user_account_info.key,
            &pda,
            lamports,
            space as u64,
            program_id,
        );

    
        // 通过带签名的调用创建账户
        invoke_signed(
            &create_account_ix,
            &[user_account_info.clone(), post_pda_account.clone(), system_program.clone()],
            &[&[user_account_info.key.as_ref(), "post".as_bytes(), &[count as u8], &[bump_seed]]],
        )?;

        post.serialize(&mut *post_pda_account.try_borrow_mut_data()?)?;

        msg!("Post created successfully. {:?}", post);
        Ok(())
    }

    fn query_posts(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pda_account = next_account_info(account_info_iter)?;
        let post_pda_account = next_account_info(account_info_iter)?;

        let user_post = try_from_slice_unchecked::<UserPost>(&pda_account.data.borrow())?;

        msg!("Posts:{:?}",user_post);

        let post = try_from_slice_unchecked::<Post>(&post_pda_account.data.borrow())?;
        msg!(" - {} at {}", post.content, post.timestamp);
        Ok(())
    }

    pub fn create_pda_account<'a>(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
    ) -> Result<&'a AccountInfo<'a>, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let payer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
        let pda_account = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;
        
        let (pda, bump_seed) = Pubkey::find_program_address(&[payer.key.as_ref()], program_id);
    
        // 验证PDA地址正确
        if pda != *pda_account.key {
            return Err(ProgramError::InvalidArgument);
        }
    
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(Mint::LEN);
    
        // 创建账户指令
        let create_account_ix = system_instruction::create_account(
            payer.key,
            pda_account.key,
            lamports,
            Mint::LEN as u64,
            program_id,
        );
    
        // 通过带签名的调用创建账户
        invoke_signed(
            &create_account_ix,
            &[payer.clone(), pda_account.clone(), system_program.clone()],
            &[&[payer.key.as_ref(), &[bump_seed]]],
        )?;
    
        Ok(pda_account)
    }
    
}

fn bytes_to_u16(bytes: &[u8]) -> Option<u16> {
    if bytes.len() != 2 {
        return None; // 确保输入是16字节
    }
    let mut array = [0u8; 2];
    array.copy_from_slice(bytes); // 将切片复制到固定大小的数组
    Some(u16::from_le_bytes(array)) // 或者使用 from_be_bytes 进行大端序转换
}

// 计算拆分数据
fn computer_profile_number(pubkey_count : u16) -> usize {
    return USER_PROFILE_SIZE + pubkey_count as usize * PUBKEY_SIZE;
}