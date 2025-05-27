use anchor_lang::prelude::*; //imports everything that anchor provides

declare_id!("7P9cdnLothRsgaHRbV1M7kDswYk3krQ47QNioz84mYju");

#[program]
pub mod calculator {
    use core::num;

    use super::*; // For using the imports defined in line 1 in this module scope

    pub fn init(ctx: Context<Initialize>, init_val: u32) -> Result<()> {
        ctx.accounts.account.num = init_val;
        Ok(())
    }

    pub fn double(ctx: Context<Double>) -> Result<()>{
        ctx.accounts.account.num = ctx.accounts.account.num * 2;
        Ok(())
    }

    pub fn add(ctx: Context<Add> ,num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num * num;
        Ok(())
    }

    
}

#[account]

struct DataShape {
    pub num: u32
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer,space =  8 + 4)]
    pub account: Account<'info, DataShape>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    signer: Signer<'info>
}

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    signer: Signer<'info>
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    signer: Signer<'info>
}

