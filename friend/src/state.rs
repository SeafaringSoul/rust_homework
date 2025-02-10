use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{msg, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Post {
    pub content: String,
    pub timestamp: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UserPost {
    pub post_count: u64,
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
            followers: Vec::new(),
        }
    }

    pub fn follow(&mut self, user: Pubkey) {
        self.followers.push(user);
        self.data_len = self.followers.len() as u16;
        msg!("Followed successfully.");
        msg!("self is {:?}", self);
    }

    pub fn unfollow(&mut self, user: Pubkey) {
        self.followers.retain(|&x| x != user);
        self.data_len = self.followers.len() as u16;
    }
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


impl Post {
    pub fn new(content: String, timestamp: u64) -> Self{
        Post { content, timestamp }
    }
}

