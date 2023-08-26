use anchor_lang::prelude::*;

pub mod constant;
pub mod states;

use crate::{constant::*, states::*};

// importing all the libraries and modules we need (globally)

// is the program address
declare_id!("B9u3esGyePjKcL3XpceGaerCaVghMvD8LsVACA9hqb44");

#[program]
pub mod blog_sol {
    use super::*; // just like declaring variables from super class

    pub fn init_user(ctx: Context<InitUser>, name: String, avatar: String) -> Result<()> {  // fn is short  for function
        // ctx is a structure of type InitUser (name of the function)
        // ctx is a mandatory thing 
        // the type of variable should be declared (unlike javascript)
        // the function "init_user" will return a result
        

        // write logic here

        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.name = name;
        user_account.avatar = avatar;
        user_account.last_post_id = 0;
        user_account.post_count = 0;
        user_account.authority = authority.key();

        Ok(())
    } 

    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> Result<()> {
        // Initialize the post and set properties
        // Increament  the post total and the id 

        let post_account = &mut ctx.accounts.post_account;
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        post_account.id = user_account.last_post_id;
        post_account.title = title;
        post_account.content = content;
        post_account.user = user_account.key();
        post_account.authority = authority.key();

        // increment the post id by 1
        user_account.last_post_id = user_account.last_post_id
            .checked_add(1)
            .unwrap();
        
        // increment the total count by 1
        user_account.post_count = user_account.post_count
            .checked_add(1)
            .unwrap();

        Ok(())
    }
        
}


#[derive(Accounts)]
#[instruction()]

pub struct InitUser<'info> {
    // info is a lifetime variable
    // this means that the stuct InitUser will live as long as it needs to

    #[account(
        // we can use this macro to initialize/ setup the account
        init, // the account is initaillized
        seeds = [USER_SEED, authority.key().as_ref()],
            // the seed is used to generate the public address 
            // USER_SEED is a variable (here, defined in the constant.rs )
            // authority means who is the incharrge of this account 
            // .as_ref() ormats the key so that it can be read by the seed
        bump, 
            // bump will go to the next available address to store the user data
        payer = authority,
        space = 2312 + 8 // bytes
            // how much space does the user take
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    
}


#[derive(Accounts)]
#[instruction()]
pub struct CreatePost<'info> {
    #[account(
        init, 
        seeds = [POST_SEED, authority.key().as_ref(), &[user_account.last_post_id as u8].as_ref()],
        bump,
        payer = authority,
        space = 2376 + 8        
    )]

    pub post_account: Account<'info, PostAccount>,

    #[account(
        mut, 
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        has_one = authority
    )]

    pub user_account: Account<'info, UserAccount>,
  
    #[account(mut)]

    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
