// Importing necessary modules and traits from Anchor framework
use anchor_lang::prelude::*;
use oracle::cpi::accounts::GetDataFeed;
use oracle::program::Oracle;
use oracle::{self, DataFeed};

// Declaring the ID for the consumer program
declare_id!("4aFLyxNC94mvefKLXDZ7eBbMVe367nZXW5txec8kX6JW");

// Definition of the consumer program module
#[program]
mod consumer {
    use super::*;

    // Function to pull data from the Oracle program
    pub fn pull_oracle(ctx: Context<PullOracle>) -> anchor_lang::Result<()> {   
        // Calling the CPI method to get data from the Oracle program     
        let result = oracle::cpi::get_datafeed(
            CpiContext::new(
                ctx.accounts.oracle_program.to_account_info(),                
                GetDataFeed {
                    datafeed: ctx.accounts.datafeed.to_account_info(),
                    signer: ctx.accounts.signer.to_account_info(),
                },
            ),             
        );

        // Unpacking the result tuple
        let (value, timestamp) = result?.get();

        // Logging the retrieved data
        msg!("consumer value {} e timestamp {}", value, timestamp);

        Ok(())
    }
}

// Definition of accounts required for the PullOracle function
#[derive(Accounts)]
pub struct PullOracle<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub oracle_program: Program<'info, Oracle>,
    pub signer: Signer<'info>,
}
