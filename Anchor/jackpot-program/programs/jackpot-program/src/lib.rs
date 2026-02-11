use anchor_lang::prelude::*;
pub mod state;
pub mod errors;

pub use state::{MasterKey, Treasury};
pub use errors::ErrorCode;

declare_id!("CDL6RbYAG3cm83FiwbSqpmhRBVZvyN3N6vaEdxQ5FKDm");

#[program]
pub mod jackpot_program {

    use super::*;
    pub fn initialize_account(ctx : Context<Initialize>, duration : i64)-> Result<()>{
        let time = Clock::get()?;
        let current_time = time.unix_timestamp;
        ctx.accounts.master_key.highest_bid = 0;
        ctx.accounts.master_key.highest_bidder = Pubkey::default();
        ctx.accounts.master_key.bump = ctx.bumps.master_key;
        ctx.accounts.master_key.treasury_bump = ctx.bumps.treasury;
        ctx.accounts.master_key.auction_end_time = current_time + duration;
        Ok(())
    }

    pub fn bid(ctx : Context<Bid>, amount : u64)-> Result<()>{
        let master_key = &mut ctx.accounts.master_key;
        let bidder = &ctx.accounts.signer;

        //Get current time : 
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        require!(
            current_time < master_key.auction_end_time,
            ErrorCode::AuctionEnded
        );

        require!(
            amount > master_key.highest_bid,
            ErrorCode::BidTooLow
        );

        let cpi_accounts = anchor_lang::system_program::Transfer{
            from : bidder.to_account_info(),
            to : ctx.accounts.treasury.to_account_info()
        };

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            cpi_accounts
        );

        anchor_lang::system_program::transfer(
            cpi_context,
            amount
        )?;

        master_key.highest_bid = amount;
        master_key.highest_bidder = *ctx.accounts.signer.key;
        
        Ok(())
    }

    pub fn settle(ctx : Context<Settle>)-> Result<()>{
        let master_key = &ctx.accounts.master_key;

        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        require!(
            current_time > master_key.auction_end_time,
            ErrorCode::AuctionStillRunning
        );

        require!(
            master_key.highest_bidder != Pubkey::default(),
            ErrorCode::NoBidsPlaced
        );

        require!(
            ctx.accounts.highest_bidder.key() == master_key.highest_bidder,
            ErrorCode::InvalidHighestBidder
        );

        let auctioneer_address = pubkey!("6KpVFh4ehrWZoWNaNWWsd4MitZ9axhybgS6CXdNiUP1V");
        require!(
            ctx.accounts.auctioneer.key() == auctioneer_address,
            ErrorCode::InvalidAuctioneer
        );

        let treasury_balance = ctx.accounts.treasury.to_account_info().lamports();
        
        let fee = treasury_balance * 10 /100;

        let winner_amount = treasury_balance - fee;

        **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? -= fee;
        **ctx.accounts.auctioneer.try_borrow_mut_lamports()? += fee;

        **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? -= winner_amount;
        **ctx.accounts.highest_bidder.try_borrow_mut_lamports()? += winner_amount;


        Ok(())
    }
}




#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + MasterKey::INIT_SPACE,
        seeds = [b"master_key"],
        bump
    )]
    pub master_key : Account<'info, MasterKey>,

    #[account(
        init,
        payer = signer,
        space = 8, // no data, just discriminator
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program : Program<'info, System>
}


#[derive(Accounts)]
pub struct Bid<'info>{
    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        mut,
        seeds = [b"master_key"],
        bump = master_key.bump
    )]
    pub master_key : Account<'info, MasterKey>,

    #[account(
        mut, 
        seeds = [b"treasury"],
        bump = master_key.treasury_bump
    )]
    pub treasury : Account<'info, Treasury>,

    pub system_program : Program<'info, System>
}


#[derive(Accounts)]
pub struct Settle<'info>{
    #[account(
        mut,
        seeds = [b"master_key"],
        bump
    )]
    pub master_key : Account<'info, MasterKey>,


    #[account(
        mut, 
        seeds = [b"treasury"],
        bump = master_key.treasury_bump
    )]
    pub treasury : Account<'info, Treasury>,

    #[account(mut)]
    pub highest_bidder: UncheckedAccount<'info>,

    #[account(mut)]
    pub auctioneer: UncheckedAccount<'info>,

    pub system_program : Program<'info, System>
}


