// Import necessary items from the prelude and other modules
use anchor_lang::prelude::*;
use crate::{Settings, Subscribers};


// Initializes the Oracle
pub fn initialize_oracle(ctx: Context<InitializeOracle>) -> Result<()> {
    // Borrow the settings account mutably from the context
    let settings_account = &mut ctx.accounts.settings;
    
    // Set the Oracle owner 
    settings_account.owner = ctx.accounts.signer.key();
    settings_account.price = 1_000_000_000;

    // Return success
    Ok(())
}

// Definition of accounts for initializing the Oracle
#[derive(Accounts)]
pub struct InitializeOracle<'info> {        
    #[account(
        init,
        payer = signer,
        space = 256,
        seeds = [ b"_settings"],     
        bump
    )]
    pub settings: Account<'info, Settings>,
    #[account(
        init,
        payer = signer,
        space = 256,
        seeds = [ b"_subscribers"],
        bump
    )]
    pub subscribers: Account<'info, Subscribers>,    
    #[account(mut)]
    pub signer: Signer<'info>,    
    pub system_program: Program<'info, System>,
}

