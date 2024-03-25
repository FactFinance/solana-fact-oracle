#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("9UYoqKcSHFhTBRoiYBcrkabsBbUKAdx68TZGLKokZKR1");

#[program]
pub mod oracle {
    use super::*;
    pub fn initialize(ctx: Context<InitializeOracle>, _feedid: u16) -> Result<()> {
        let datafeed = &mut ctx.accounts.datafeed;
        datafeed.owner = ctx.accounts.signer.key();

        Ok(())
    }

    pub fn get_datafeed(ctx: Context<GetDataFeed>) -> Result<(u32, u32)> {
        let datafeed = &mut ctx.accounts.datafeed;

        if
            datafeed.license > 0 &&
            !datafeed.subscribers.iter().any(|p| p == &ctx.accounts.signer.key())
        {
            return err!(OracleErrors::Subscribe);
        }

        let value_slice = (datafeed.value >> 32) as u32;
        let timestamp = (datafeed.value & 0xffffffff) as u32;

        let sq = (timestamp as f64).sqrt() as u32;
        let value = value_slice - sq;

        msg!("Returning {} as {} and {} ", datafeed.value, value, timestamp);

        Ok((value, timestamp))
    }

    pub fn set_value(ctx: Context<SetValue>, value: u32, timestamp: u32) -> Result<()> {
        let datafeed = &mut ctx.accounts.datafeed;

        let coded_value = (value as f64) + (timestamp as f64).sqrt();
        let combined_value = ((coded_value as u64) << 32) | (timestamp as u64);

        datafeed.value = combined_value;

        msg!("New value {} that means {}", datafeed.value, value);

        Ok(())
    }

    
    pub fn set_license(ctx: Context<SetLicense>, license: u8) -> Result<()> {
        let datafeed = &mut ctx.accounts.datafeed;

        if ctx.accounts.signer.key() != datafeed.owner {
            return err!(OracleErrors::AccessDenied);
        }

        datafeed.license = license;

        msg!("New license {}", datafeed.license);

        Ok(())
    }

    pub fn add_subscription(ctx: Context<AddSubscription>, address: Pubkey) -> Result<()> {
        let datafeed = &mut ctx.accounts.datafeed;

        // if ctx.accounts.signer.key() != datafeed.owner {
        //     return err!(OracleErrors::AccessDenied);
        // }

        if !datafeed.subscribers.iter().any(|p| p == &address) {
            datafeed.subscribers.push(address);
            msg!("New Subscrption {}", address);
        }

        Ok(())
    }
    pub fn revoke_subscription(ctx: Context<RevokeSubscription>, address: Pubkey) -> Result<()> {
        let datafeed = &mut ctx.accounts.datafeed;

       if ctx.accounts.signer.key() != datafeed.owner {
            return err!(OracleErrors::AccessDenied);            
        }

        datafeed.subscribers.retain(|pubkey| pubkey != &address);

        msg!("Revoked subscrption {}", address);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(feedid: u16)]
pub struct InitializeOracle<'info> {
    #[account(
        init,
        payer = signer,
        space = 256,
        seeds = [signer.key().as_ref(), b"_", feedid.to_string().as_ref()],
        bump
    )]
    pub datafeed: Account<'info, DataFeed>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetDataFeed<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetValue<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,    
}

#[derive(Accounts)]
pub struct SetLicense<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddSubscription<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct RevokeSubscription<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub signer: Signer<'info>,
}

#[account]
pub struct DataFeed {
    value: u64,
    license: u8,
    owner: Pubkey,
    subscribers: Vec<Pubkey>,
}

#[error_code]
pub enum OracleErrors {
    #[msg("You dont have access!")]
    AccessDenied,
    #[msg("Subscribe this feed at https://fact.finance")]
    Subscribe,
}
