use anchor_lang::prelude::*;

declare_id!("tEstkZyM2FqJBECcFNw4VcuEib3r2kAqavF83q2oXY2");

#[program]
pub mod codama {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value = 0;
        counter.authority = ctx.accounts.payer.key();
        Ok(())
    }

    pub fn increment_counter(ctx: Context<Update>, input: u32) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        require!(
            ctx.accounts.payer.key() == counter.authority,
            ErrorCode::Unauthorized
        );

        counter.value += input;

        Ok(())
    }
}

#[derive(InitSpace)]
#[account]
pub struct Counter {
    pub value: u32,
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [b"counter", payer.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds = [b"counter", payer.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    pub payer: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not the authority")]
    Unauthorized,
}
