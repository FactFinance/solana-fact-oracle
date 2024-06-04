// Import necessary items from the prelude and other modules
use anchor_lang::prelude::*;
use crate::{Settings, Subscribers,  OracleErrors};

use anchor_spl::token::{self, Token, TokenAccount, Transfer};



pub fn subscribe(ctx: Context<Subscribe>, address: Pubkey) -> Result<()> {

    let settings_account = &mut ctx.accounts.settings;

    // if ctx.accounts.receiver.key() != settings_account.owner {
    //     return err!(OracleErrors::AccessDenied);
    // }

    // Amount of BONK to transfer 
    let amount: u64 = settings_account.price ;

    // Create the transfer instruction
    let cpi_accounts = Transfer {
        from: ctx.accounts.sender.to_account_info().clone(),
        to: ctx.accounts.receiver.to_account_info().clone(),
        authority: ctx.accounts.sender.to_account_info().clone(),
    };
    let cpi_program = ctx.accounts.token.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // Perform the token transfer
    token::transfer(cpi_ctx, amount)?;

    msg!("Received 1 BONK from {} to new signer {}", ctx.accounts.sender.key(), address);

    let subscribers_account = &mut ctx.accounts.subscribers;

    // Add the subscriber address if it's not already present
    if !subscribers_account.subscribers.iter().any(|p| p == &address) {
        subscribers_account.subscribers.push(address);        
    }

    Ok(())
}


// Adds a subscription to the data feed
pub fn add_subscription(ctx: Context<AddSubscription>, address: Pubkey) -> Result<()> {
    // Get mutable references to the data feed and subscribers account
    let settings_account = &mut ctx.accounts.settings;
    let subscribers_account = &mut ctx.accounts.subscribers;

    // Check if the signer is the owner of the data feed
    if ctx.accounts.signer.key() != settings_account.owner {
        return err!(OracleErrors::AccessDenied);
    }

    // Add the subscriber address if it's not already present
    if !subscribers_account.subscribers.iter().any(|p| p == &address) {
        subscribers_account.subscribers.push(address);
        msg!("New Subscription {}", address);
    }

    Ok(())
}



// Revokes a subscription from the data feed
pub fn revoke_subscription(ctx: Context<RevokeSubscription>, address: Pubkey) -> Result<()> {
    // Get mutable references to the data feed and subscribers account
    let settings_account = &mut ctx.accounts.settings;
    let subscribers_account = &mut ctx.accounts.subscribers;

    // Check if the signer is the owner of the data feed
    if ctx.accounts.signer.key() != settings_account.owner {
        return err!(OracleErrors::AccessDenied);            
    }

    // Remove the subscriber address if it's present
    subscribers_account.subscribers.retain(|pubkey| pubkey != &address);

    msg!("Revoked Subscription {}", address);

    Ok(())
}



#[derive(Accounts)]
pub struct Subscribe<'info> {    
    #[account(mut)]   
    pub sender: Account<'info, TokenAccount>,           
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,    
    #[account(seeds = [ b"_settings"],bump)]
    pub settings: Account<'info, Settings>,
    #[account(mut)]
    pub subscribers: Account<'info, Subscribers>,    
    pub token: Program<'info, Token>,
}



// Definition of accounts for adding a subscription to the data feed
#[derive(Accounts)]
pub struct AddSubscription<'info> {    
    #[account(seeds = [ b"_settings"],bump)]
    pub settings: Account<'info, Settings>,
    pub signer: Signer<'info>,
    #[account(mut)]
    pub subscribers: Account<'info, Subscribers>,
} 

// Definition of accounts for revoking a subscription from the data feed
#[derive(Accounts)]
pub struct RevokeSubscription<'info> {
    #[account(seeds = [ b"_settings"],bump)]
    pub settings: Account<'info, Settings>,
    #[account(seeds = [ b"_subscribers"],bump)]
    #[account(mut)]
    pub subscribers: Account<'info, Subscribers>,
    pub signer: Signer<'info>,
}
