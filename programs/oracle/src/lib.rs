use anchor_lang::prelude::*;

// Declares the ID for the program
declare_id!("9UYoqKcSHFhTBRoiYBcrkabsBbUKAdx68TZGLKokZKR1");

// Definition of the Oracle program module
#[program]
pub mod oracle {
    use super::*;

    // Initializes a data feed
    pub fn initialize(ctx: Context<InitializeOracle>, _feedid: u16) -> Result<()> {
        let datafeed = &mut ctx.accounts.datafeed;
        datafeed.owner = ctx.accounts.signer.key();

        Ok(())
    }

    // Gets data from the data feed
    pub fn get_datafeed(ctx: Context<GetDataFeed>) -> Result<(i32, u32)> {
        let datafeed = &mut ctx.accounts.datafeed;

        if
            datafeed.license > 0 &&
            !datafeed.subscribers.iter().any(|p| p == &ctx.accounts.signer.key())
        {
            return err!(OracleErrors::Subscribe);
        }

        msg!("returning value {} from {} to {}", datafeed.value, datafeed.timestamp,&ctx.accounts.signer.key() );

        Ok((datafeed.value, datafeed.timestamp))
    }

    // Sets the value of the data feed
    pub fn set_value(ctx: Context<SetValue>, value: i32, timestamp: u32, symbol: String) -> Result<()> {
        let datafeed = &mut ctx.accounts.datafeed;

        if ctx.accounts.signer.key() != datafeed.owner {
            return err!(OracleErrors::AccessDenied);
        }

        datafeed.value = value;
        datafeed.timestamp = timestamp;

        msg!("New value {} for {}", datafeed.value, symbol);

        Ok(())
    }

    // Sets the license of the data feed
    // 0 for OpenOracle
    // 1 for Subscription required
    pub fn set_license(ctx: Context<SetLicense>, license: u8) -> Result<()> {
        let datafeed = &mut ctx.accounts.datafeed;

        if ctx.accounts.signer.key() != datafeed.owner {
            return err!(OracleErrors::AccessDenied);
        }

        datafeed.license = license;

        msg!("New license {}", datafeed.license);

        Ok(())
    }

    // Adds a subscription to the data feed
    pub fn add_subscription(ctx: Context<AddSubscription>, address: Pubkey) -> Result<()> {
        let datafeed = &mut ctx.accounts.datafeed;

        if ctx.accounts.signer.key() != datafeed.owner {
            return err!(OracleErrors::AccessDenied);
        }

        if !datafeed.subscribers.iter().any(|p| p == &address) {
            datafeed.subscribers.push(address);
            msg!("New Subscrption {}", address);
        }

        Ok(())
    }

    // Revokes a subscription from the data feed
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

// Definition of accounts for initializing the Oracle
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

// Definition of accounts for getting data from the data feed
#[derive(Accounts)]
pub struct GetDataFeed<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub signer: Signer<'info>,
}

// Definition of accounts for setting the value of the data feed
#[derive(Accounts)]
pub struct SetValue<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,  
    pub signer: Signer<'info>,
}

// Definition of accounts for setting the license of the data feed
#[derive(Accounts)]
pub struct SetLicense<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub signer: Signer<'info>,
}

// Definition of accounts for adding a subscription to the data feed
#[derive(Accounts)]
pub struct AddSubscription<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub signer: Signer<'info>,
}

// Definition of accounts for revoking a subscription from the data feed
#[derive(Accounts)]
pub struct RevokeSubscription<'info> {
    #[account(mut)]
    pub datafeed: Account<'info, DataFeed>,
    pub signer: Signer<'info>,
}

// Definition of the data feed account
#[account]
pub struct DataFeed {
    value: i32,
    timestamp: u32,
    license: u8,
    owner: Pubkey,
    subscribers: Vec<Pubkey>,
}

// Definition of custom error codes for the Oracle program
#[error_code]
pub enum OracleErrors {
    #[msg("Access denied!")]
    AccessDenied,
    #[msg("Subscribe to this feed at https://fact.finance")]
    Subscribe,
}
