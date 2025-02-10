use solana_program::pubkey::Pubkey;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SocialInstruction {
    InitializeUser{seed_type: String},
    FollowUser { user_to_follow: Pubkey },
    UnfollowUser { user_to_unfollow: Pubkey },
    PostContent { content: String },
    QueryFollowers,
    QueryPosts,
}