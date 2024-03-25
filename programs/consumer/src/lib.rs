#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use oracle::cpi::accounts::GetDataFeed;
use oracle::program::Oracle;
use oracle::{self, DataFeed};

declare_id!("4aFLyxNC94mvefKLXDZ7eBbMVe367nZXW5txec8kX6JW");

#[program]
mod consumer {
    use super::*;
    pub fn pull_oracle(ctx: Context<PullOracle>) -> anchor_lang::Result<()> {        
        let result = oracle::cpi::get_datafeed(
            CpiContext::new(
                ctx.accounts.oracle_program.to_account_info(),                
                GetDataFeed {
                    datafeed: ctx.accounts.datafeed.to_account_info(),
                    signer: ctx.accounts.signer.to_account_info(),
                },
            ),             
        );

        let (value, timestamp) = result?.get();

        msg!("consumer value {} e timestamp {}", value, timestamp);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PullOracle<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub oracle_program: Program<'info, Oracle>,
    pub signer: Signer<'info>,
}
