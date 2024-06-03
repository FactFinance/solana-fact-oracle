use anchor_lang::prelude::*;

// Import necessary items from the crate
use crate::{FeedAuditor, DataFeed, OracleErrors};

// Set an external auditor for the data feed
pub fn set_auditor(ctx: Context<SetAuditor>, address: Pubkey) -> Result<()> {
    // Borrow the data feed and auditor accounts mutably from the context
    let datafeed = &mut ctx.accounts.datafeed;
    let auditor_account = &mut ctx.accounts.auditor;

    // Check if the caller is the owner of the data feed
    if ctx.accounts.signer.key() != datafeed.owner {
        // Return an error if the caller is not the owner
        return err!(OracleErrors::AccessDenied);            
    }

    // Set the auditor's address
    auditor_account.auditor = address;

    // Log the addition of the new auditor
    msg!("New auditor {}", address);

    // Return success
    Ok(())
}

// Set the allowed range of value for the data feed
pub fn set_limit(ctx: Context<SetLimit>, min: i32, max: i32) -> Result<()> {
    // Borrow the data feed and auditor accounts mutably from the context
    let datafeed = &mut ctx.accounts.datafeed;
    let auditor_account = &mut ctx.accounts.auditor;

    // Check if the caller is the owner or auditor of the data feed
    if ctx.accounts.signer.key() != datafeed.owner && ctx.accounts.signer.key() != auditor_account.auditor {
        // Return an error if the caller is neither the owner nor the auditor
        return err!(OracleErrors::AccessDenied);            
    }

    // Set the minimum and maximum allowed values
    auditor_account.min = min;
    auditor_account.max = max;

    // Log the new range defined
    msg!("New range defined by {}: min {} and max {}", ctx.accounts.signer.key(), min, max);

    // Return success
    Ok(())
}

// Definition for setting an external auditor for the data feed
#[derive(Accounts)]
pub struct SetAuditor<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub auditor: Account<'info, FeedAuditor>,
    pub signer: Signer<'info>,
}

// Definition for setting the allowed range of value for the data feed
#[derive(Accounts)]
pub struct SetLimit<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub auditor: Account<'info, FeedAuditor>,
    pub signer: Signer<'info>,
}
