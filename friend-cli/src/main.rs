use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};
use std::{str::FromStr, time::Instant};

use solana_sdk::signature::read_keypair_file;

use solana_program::instruction::AccountMeta;

static PROFILE_SEED: &str = "profile"; 
static POST_SEED: &str = "post"; 

// 指令枚举（与合约保持一致）
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum SocialInstruction {
    InitializeUser{seed_type: String},
    FollowUser { user_to_follow: Pubkey },
    UnfollowUser { user_to_unfollow: Pubkey },
    PostContent { content: String },
    QueryFollowers,
    QueryPosts,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UserProfile {
    pub data_len: u16,
    pub followers: Vec<Pubkey>,  // 关注的用户列表
}

impl UserProfile {
    pub fn new() -> Self {
        Self {
            data_len: 0,
            followers: vec![],
        }
    }
}


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Post {
    pub content: String,
    pub timestamp: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UserPost {
    pub post_count: u64,
}

impl UserPost {
    pub fn new() -> Self {
        Self {
            post_count: 0,
        }
    }

    pub fn add_post(&mut self) {
        self.post_count += 1;
    }

    pub fn get_count(&self) -> u64 {
        self.post_count
    }
}


// 客户端核心逻辑
pub struct SocialClient {
    rpc_client: RpcClient,
    program_id: Pubkey,
}

impl SocialClient {
    pub fn new(rpc_url: &str, program_id: Pubkey) -> Self {
        let rpc_client = RpcClient::new(rpc_url.to_string());
        Self { rpc_client, program_id }
    }

    pub fn initialize_user(&self, user_keypair: &Keypair, seed_type: &str) -> Result<(), Box<dyn std::error::Error>> {
       
        let pda = get_profile_pda( &self.program_id, &[user_keypair.pubkey().as_ref(), seed_type.as_bytes()]);

        let initialize_user_instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::InitializeUser{seed_type: seed_type.to_string()},  // 初始化用户的指令
            vec![
                AccountMeta::new(user_keypair.pubkey(), true),
                AccountMeta::new(pda, false),
                AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            ], // 设置账户元数据
        );

        self.send_instruction(user_keypair, vec![ initialize_user_instruction])?;
    
        println!("User initialized successfully.");
        Ok(())
    }
    

    pub fn follow_user(&self, user_keypair: &Keypair, user_to_follow: Pubkey) -> Result<(), Box<dyn std::error::Error>> {
        let pda = get_profile_pda( &self.program_id,&[user_keypair.pubkey().as_ref(), PROFILE_SEED.as_bytes()]);

        let instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::FollowUser { user_to_follow },
            vec![AccountMeta::new(pda, false)],
        );

        self.send_instruction(user_keypair, vec![instruction])?;
        println!("User followed successfully.");
        Ok(())
    }

    pub fn unfollow_user(&self, user_keypair: &Keypair, user_to_unfollow: Pubkey) -> Result<(), Box<dyn std::error::Error>> {

        let pda = get_profile_pda( &self.program_id,&[user_keypair.pubkey().as_ref(), PROFILE_SEED.as_bytes()]);

        let instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::UnfollowUser { user_to_unfollow },
            vec![AccountMeta::new(pda, false)],
        );

        self.send_instruction(user_keypair, vec![instruction])?;
        println!("User unfollowed successfully.");
        Ok(())
    }

    pub fn query_followers(&self, user_keypair: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
        let pda = get_profile_pda( &self.program_id,&[user_keypair.pubkey().as_ref(), PROFILE_SEED.as_bytes()]);

        let instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::QueryFollowers,
            vec![AccountMeta::new_readonly(pda, false)],
        );

        self.send_instruction(user_keypair, vec![instruction])?;
        println!("Followers queried successfully.");
        Ok(())
    }


    pub fn post_content(&self, user_keypair: &Keypair, content: String, id : u64) -> Result<(), Box<dyn std::error::Error>> {
        let pda = get_profile_pda( &self.program_id,&[user_keypair.pubkey().as_ref(), POST_SEED.as_bytes()]);

        let post_pda = get_profile_pda( &self.program_id,&[user_keypair.pubkey().as_ref(), POST_SEED.as_bytes(), &[id as u8]]);

        let instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::PostContent { content },
            vec![
                    AccountMeta::new(user_keypair.pubkey(), true),
                    AccountMeta::new(pda, false),
                    AccountMeta::new(post_pda, false),
                    AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
                ],
        );

        self.send_instruction(user_keypair, vec![instruction])?;
        println!("Post created successfully.");
        Ok(())
    }


    pub fn query_posts(&self, user_keypair: &Keypair, id : u64) -> Result<(), Box<dyn std::error::Error>> {
        let post_pda = get_profile_pda( &self.program_id,&[user_keypair.pubkey().as_ref(), POST_SEED.as_bytes(), &[id as u8]]);

        let pda = get_profile_pda( &self.program_id,&[user_keypair.pubkey().as_ref(), POST_SEED.as_bytes()]);

        let instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::QueryPosts,
            vec![
                    AccountMeta::new(pda, false),
                    AccountMeta::new(post_pda, false),
                ],
        );

        self.send_instruction(user_keypair, vec![instruction])?;
        println!("Posts queried successfully.");
        Ok(())
    }

    fn send_instruction(
        &self,
        payer: &Keypair,
        instructions: Vec<Instruction>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let latest_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&payer.pubkey()),
            &[payer],
            latest_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        println!("Transaction successful: {}", signature);
        Ok(())
    }
}

fn get_profile_pda(program_id : &Pubkey, seed: &[&[u8]]) -> Pubkey {
    let (pda,_bump) = Pubkey::find_program_address(seed, &program_id);
    println!("pad: {:?}",pda);
    return pda; 
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // println!("{:?}", borsh::to_vec(&UserPost::new()).unwrap().len());

    let program_id = Pubkey::from_str("2NGEHiBALDk4fRfBkpCyuzu317YNcqT2HLFKM2hVXVHJ")?;
    let user_keypair = read_keypair_file("/Users/lihao/.config/solana/id.json").expect("failed");
    // // let user_keypair = read_keypair_file("/Users/admin/.config/solana/study1.json").expect("failed");

    let client = SocialClient::new("http://127.0.0.1:8899", program_id);

    // // 初始化用户
    // // client.initialize_user(&user_keypair, PROFILE_SEED)?;

    // // 关注用户
    // let user_to_follow = Pubkey::from_str("Azb6KFfzDpn8RCf94F8nYzAbS1LyiczxdsAtn3ryDGzR")?;
    // client.follow_user(&user_keypair, user_to_follow)?;


    // client.query_followers(&user_keypair)?;
    
    // client.unfollow_user(&user_keypair, user_to_follow)?;

    // client.query_followers(&user_keypair)?;

    // 初始化帖子账户
    // client.initialize_user(&user_keypair, POST_SEED)?;


    // 初始化用户
    client.initialize_user(&user_keypair, POST_SEED)?;
    // 发布动态
    let post_content = "Hello Solana!".to_string();
    client.post_content(&user_keypair, post_content,1 )?;


    let post_content_2 = "Hello Solana! id: 2".to_string();
    client.post_content(&user_keypair, post_content_2,2 )?;

    // 查询动态
    client.query_posts(&user_keypair, 1)?;

    client.query_posts(&user_keypair, 2)?;

    // let accounts = client.rpc_client.get_program_accounts(&program_id).unwrap();
    // println!("accounts: {:?}", accounts);
    // for (pubkey, account) in accounts.iter() {
    //     println!("pubkey: {:?}, account: {:?}", pubkey, account);
    //     let data = UserProfile::try_from_slice(&account.data).unwrap();
    //     println!("data: {:?}", data);
    // }

    // let mut large_array = vec![];

    // let mut large_array = Box::new(vec![]);

    // for i in 0..1_000_000  {
    //     large_array.push(i);
    // }

    // let start = Instant::now(); 

    // // let _large_array = Box::new([0u8; 1_000_000]);
    // large_array.push(1);
    
    // let duration = start.elapsed();
    
    
    // let mut large_array = vec![];

    // for i in 0..1_000_000  {
    //     large_array.push(i);
    // }

    // // let mut large_array = [0u8; 1_000_000];
    // let start = Instant::now(); 
    // large_array.push(10);
    
    // let duration = start.elapsed();
    
    // println!("{:?}",duration);

    Ok(())
}
