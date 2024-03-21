use anchor_lang::prelude::*;

declare_id!("Cr5qHmgnDgpRQzLpKorQhadioE46mhLHbMHeLxupVN2y");

#[program]
pub mod oracle {
    use super::*;

    pub fn create_ledger(ctx: Context<CreateLedger>, _feedid: u16) -> Result<()> {
        let ledger_account = &mut ctx.accounts.ledger_account;
        ledger_account.value = 0;
        ledger_account.timestamp = 0;
        ledger_account.license = 0;

        Ok(())
    }

    pub fn get_value(ctx: Context<SetValue>) -> Result<(u32, u32)> {
        let ledger_account = &mut ctx.accounts.ledger_account;      
        
        if
            ledger_account.license > 0 &&
            !ledger_account.subscribers.iter().any(|p| p == &ctx.accounts.wallet.key())
        {
            msg!("access denied");
            return Ok(( 0, 0));
        }

        Ok((ledger_account.value, ledger_account.timestamp))
    }

    pub fn set_value(ctx: Context<SetValue>, value: u32, timestamp: u32) -> Result<()> {
        let ledger_account = &mut ctx.accounts.ledger_account;
        ledger_account.value = value;
        ledger_account.timestamp = timestamp;

        Ok(())
    }

    pub fn set_license(ctx: Context<SetLicense>, license: u8) -> Result<()> {
        let ledger_account = &mut ctx.accounts.ledger_account;
        ledger_account.license = license;

        Ok(())
    }

    pub fn add_subscription(ctx: Context<AddSubscription>, address: Pubkey) -> Result<()> {
        let ledger_account = &mut ctx.accounts.ledger_account;
        ledger_account.subscribers.push(address);

        Ok(())
    }

    pub fn revoke_subscription(ctx: Context<RevokeSubscription>, address: Pubkey) -> Result<()> {
        let ledger_account = &mut ctx.accounts.ledger_account;
        ledger_account.subscribers.retain(|pubkey| pubkey != &address);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(feedid: u16)]
pub struct CreateLedger<'info> {
    #[account(
        init,
        payer = wallet,
        space = 82,
        seeds = [wallet.key().as_ref(), b"_", feedid.to_string().as_ref()],
        bump
    )]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetValue<'info> {
    #[account(mut)]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetValue<'info> {
    #[account(mut)]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetLicense<'info> {
    #[account(mut)]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddSubscription<'info> {
    #[account(mut)]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
}

#[derive(Accounts)]
pub struct RevokeSubscription<'info> {
    #[account(mut)]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
}


#[account]
pub struct Ledger {
    value: u32,
    timestamp: u32,
    license: u8,
    subscribers: Vec<Pubkey>,
}
