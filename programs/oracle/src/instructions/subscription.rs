// Import necessary items from the prelude and other modules
use anchor_lang::prelude::*;
use crate::{Subscribers, DataFeed, OracleErrors};

// Adds a subscription to the data feed
pub fn add_subscription(ctx: Context<AddSubscription>, address: Pubkey) -> Result<()> {
    // Get mutable references to the data feed and subscribers account
    let datafeed = &mut ctx.accounts.datafeed;
    let subscribers_account = &mut ctx.accounts.subscribers;

    // Check if the signer is the owner of the data feed
    if ctx.accounts.signer.key() != datafeed.owner {
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
    let datafeed = &mut ctx.accounts.datafeed;
    let subscribers_account = &mut ctx.accounts.subscribers;

    // Check if the signer is the owner of the data feed
    if ctx.accounts.signer.key() != datafeed.owner {
        return err!(OracleErrors::AccessDenied);            
    }

    // Remove the subscriber address if it's present
    subscribers_account.subscribers.retain(|pubkey| pubkey != &address);

    msg!("Revoked Subscription {}", address);

    Ok(())
}

// Definition of accounts for adding a subscription to the data feed
#[derive(Accounts)]
pub struct AddSubscription<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub subscribers: Account<'info, Subscribers>,
    pub signer: Signer<'info>,
} 

// Definition of accounts for revoking a subscription from the data feed
#[derive(Accounts)]
pub struct RevokeSubscription<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub subscribers: Account<'info, Subscribers>,
    pub signer: Signer<'info>,
}
