use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserAccount{
    pub name: String, // space = 4+256
    pub avatar: String, // space = 4 + 2048 (longest url)
    pub authority: Pubkey, // space = 32
    pub last_post_id: u8, // space = 1
    pub post_count: u8, // space = 1
}

#[account]
#[derive(Default)]
pub struct PostAccount {
    pub id: u8, // 1
    pub title: String,  // 4 + 256
    pub content: String,    // 4 + 2048    
    pub user: Pubkey,   // 32
    pub authority: Pubkey,  // 32

}
