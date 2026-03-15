use anchor_lang::prelude::*;
mod state;

declare_id!("EzGAFZDUdtWKXhYkXu8P2LgMno95gBqzACFook4kAgcg");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
