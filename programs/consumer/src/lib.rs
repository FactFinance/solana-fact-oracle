// Importing necessary modules and traits from Anchor framework
use anchor_lang::prelude::*;
use oracle::cpi::accounts::GetDataFeed;
use oracle::program::Oracle;
use oracle::{self, DataFeed, Subscribers};

// Declaring the ID for the consumer program
declare_id!("EjWZ7S5PyFVDSipd2BZTfHbDP5DiWWFNBK8pmiCVQj6q");


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
                    subscribers: ctx.accounts.subscribers.to_account_info(),
                    signer: ctx.accounts.signer.to_account_info(),
                },
            ),             
        );

        // Unpacking the result tuple
        let (value, timestamp, confidence) = result?.get();

        // Logging the retrieved data
        msg!("consumer value {} and timestamp {} with confidence {} ", value, timestamp, confidence);

        Ok(())
    }
}

// Definition of accounts required for the PullOracle function
#[derive(Accounts)]
pub struct PullOracle<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub subscribers: Account<'info, Subscribers>,
    pub oracle_program: Program<'info, Oracle>,
    pub signer: Signer<'info>,
}
