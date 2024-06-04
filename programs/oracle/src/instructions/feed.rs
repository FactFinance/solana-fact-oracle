// Import necessary items from the prelude and other modules
use anchor_lang::prelude::*;
use crate::{Settings,DataFeed, Subscribers, FeedAuditor, OracleErrors, check_limit};


// Initializes a data feed
pub fn initialize_datafeed(_ctx: Context<InitializeDatafeed>, _feedid: u16) -> Result<()> {    
    Ok(())
}

// Gets data from the data feed
pub fn get_datafeed(ctx: Context<GetDataFeed>) -> Result<(i32, u32)> {
    // Borrow the data feed and subscribers accounts mutably from the context
    let datafeed = &mut ctx.accounts.datafeed;
    let subscribers_account = &mut ctx.accounts.subscribers;

    // Check if the caller is subscribed to the data feed
    if datafeed.license > 0 && !subscribers_account.subscribers.iter().any(|p| p == &ctx.accounts.signer.key()) {
        // Return an error if the caller is not subscribed
        return err!(OracleErrors::Subscribe);
    }

    // Log the return of data from the data feed
    msg!("Returning value {} from {} to {}", datafeed.value, datafeed.timestamp, &ctx.accounts.signer.key() );

    // Return the value and timestamp
    Ok((datafeed.value, datafeed.timestamp))
}

// Sets the value of the data feed
pub fn set_value(ctx: Context<SetValue>, value: i32, timestamp: u32, symbol: String) -> Result<()> {
    // Borrow the data feed and auditor accounts mutably from the context
    let datafeed = &mut ctx.accounts.datafeed;
    let auditor_account = &mut ctx.accounts.auditor;
    let settings_account = &mut ctx.accounts.settings;

    // Check if the caller is the owner of the data feed
    if ctx.accounts.signer.key() != settings_account.owner {
        // Return an error if the caller is not the owner
        return err!(OracleErrors::AccessDenied);
    }

    // Check if the value is within the auditor-defined limits
    if check_limit(value, auditor_account.min, auditor_account.max) {
        // Set the value and timestamp of the data feed
        datafeed.value = value;
        datafeed.timestamp = timestamp;

        // Log the new value
        msg!("New value {} for {}", datafeed.value, symbol);
    }

    // Return success
    Ok(())
}


// Definition of accounts for initializing the Oracle
#[derive(Accounts)]
#[instruction(feedid: u16)]
pub struct InitializeDatafeed<'info> {     
    #[account(
        init,
        payer = signer,
        space = 256,
        seeds = [signer.key().as_ref(), b"_datafeed", feedid.to_string().as_ref()],
        bump
    )]
    pub datafeed: Account<'info, DataFeed>,
    #[account(
        init,
        payer = signer,
        space = 256,
        seeds = [signer.key().as_ref(), b"_auditor", feedid.to_string().as_ref()],
        bump
    )]    
    pub auditor: Account<'info, FeedAuditor>,   
    pub system_program: Program<'info, System>,
    #[account(mut)]     
    pub signer: Signer<'info>,    
}

// Definition of accounts for getting data from the data feed
#[derive(Accounts)]
pub struct GetDataFeed<'info> {    
    pub datafeed: Account<'info, DataFeed>,
    #[account(seeds = [b"_subscribers"],bump)]
    pub subscribers: Account<'info, Subscribers>,
    pub signer: Signer<'info>,
}

// Definition of accounts for setting the value of the data feed
#[derive(Accounts)]
pub struct SetValue<'info> {
    #[account(seeds = [ b"_settings"],bump)]
    pub settings: Account<'info, Settings>,  
    pub auditor: Account<'info, FeedAuditor>,    
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,  
    pub signer: Signer<'info>,
}
