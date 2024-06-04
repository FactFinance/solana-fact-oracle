use anchor_lang::prelude::*;

// Import utility functions and error definitions
mod utils;
use utils::*;

// Import state definitions (accounts)
mod state;
pub use crate::state::*;

// Import instruction modules
mod instructions;
use instructions::*;

// Declares the ID for the program
declare_id!("GVphyWPDGnsrxCGfXnBkeddqhoSRmVFSYUrhFMfsXrCy");

/// Definition of the Oracle program module
/// This module contains all the instruction handlers for the Oracle program.
#[program]
pub mod oracle {
    use super::*;

    /// Initializes the Oracle .
    /// Sets the signer as the owner .
    pub fn initialize_oracle(ctx: Context<InitializeOracle>, ) -> Result<()> {
        instructions::initialize_oracle(ctx)
    }

    /// Initializes a new data feed with a unique identifier.
    /// Sets the signer as the owner of the data feed.
    pub fn initialize_datafeed(ctx: Context<InitializeDatafeed>, feedid: u16) -> Result<()> {
        instructions::initialize_datafeed(ctx, feedid)
    }

    /// Subscribe the service
    pub fn subscribe(ctx: Context<Subscribe>, address: Pubkey ) -> Result<()> {
        instructions::subscribe(ctx, address)
    }


    /// Retrieves the current value and timestamp from the data feed.
    /// Only accessible if the user has the required subscription or the feed is open.
    pub fn get_datafeed(ctx: Context<GetDataFeed>) -> Result<(i32, u32)> {
        instructions::get_datafeed(ctx)
    }

    /// Sets the value of the data feed, with the provided timestamp and symbol.
    /// Only the owner of the data feed can set the value.
    pub fn set_value(ctx: Context<SetValue>, value: i32, timestamp: u32, symbol: String) -> Result<()> {
        instructions::set_value(ctx, value, timestamp, symbol)
    }

    /// Sets the license type of the data feed.
    /// License type can be 0 (open) or 1 (subscription required).
    /// Only the owner of the data feed can set the license.
    pub fn set_license(ctx: Context<SetLicense>, license: u8) -> Result<()> {                
        instructions::set_license(ctx, license)
    }

    /// Sets the auditor for the data feed.
    /// Only the owner of the data feed can set the auditor.
    pub fn set_auditor(ctx: Context<SetAuditor>, address: Pubkey) -> Result<()> {
        instructions::set_auditor(ctx, address)
    }

    /// Sets the minimum and maximum allowed values for the data feed.
    /// Only the owner or the auditor can set these limits.
    pub fn set_limit(ctx: Context<SetLimit>, min: i32, max: i32) -> Result<()> {
        instructions::set_limit(ctx, min, max)
    }

    /// Adds a subscriber to the list of addresses allowed to query the data feed.
    /// Only the owner of the data feed can add subscribers.
    pub fn add_subscription(ctx: Context<AddSubscription>, address: Pubkey) -> Result<()> {  
        instructions::add_subscription(ctx, address)
    }

    /// Revokes a subscriber's access to the data feed.
    /// Only the owner of the data feed can revoke subscriptions.
    pub fn revoke_subscription(ctx: Context<RevokeSubscription>, address: Pubkey) -> Result<()> { 
        instructions::revoke_subscription(ctx, address)
    }  
}
