use anchor_lang::prelude::*;

declare_id!("4yjpZFEUwYvUyPH8hJ1hmVteXTbJUA5ie1nVvDy1YTG9");

#[program]
pub mod first_anchor_program {
    use super::*;
    pub fn initialize(ctx : Context<Initialize>)-> Result<()>{
        let counter = &mut ctx.accounts.counter;
        msg!("Counter Account Created");
        msg!("Current Count: { }", counter.count );
        Ok(())
    }

    pub fn increment(ctx : Context<Update>)-> Result<()>{
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }

}

#[account]
#[derive(InitSpace)]
pub struct Counter{
    pub count : u64,
}


#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(
        init,
        payer = user,
        space = 8 + Counter::INIT_SPACE
    )]
    pub counter : Account<'info, Counter>,
    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct Update<'info>{
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user : Signer<'info>
}
