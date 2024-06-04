// Import necessary items from the prelude and other modules
use anchor_lang::prelude::*;
use crate::{Settings, DataFeed, OracleErrors};

// Sets the license of the data feed
pub fn set_license(ctx: Context<SetLicense>, license: u8) -> Result<()> {
    // Borrow the data feed account mutably from the context
    let datafeed = &mut ctx.accounts.datafeed;
    let settings_account = &mut ctx.accounts.settings;

    // Check if the caller is the owner of the data feed
    if ctx.accounts.signer.key() != settings_account.owner {
        // Return an error if the caller is not the owner
        return err!(OracleErrors::AccessDenied);
    }

    // Set the license of the data feed to the provided value
    datafeed.license = license;

    // Log the new license
    msg!("New license {}", datafeed.license);

    // Return success
    Ok(())
}

// Definition of accounts for setting the license of the data feed
#[derive(Accounts)]
pub struct SetLicense<'info> {    
    #[account(seeds = [ b"_settings"],bump)]
    pub settings: Account<'info, Settings>,    
    pub signer: Signer<'info>,
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
}
