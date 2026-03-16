use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken};
use anchor_spl::{token_interface::{Mint, TokenAccount, TokenInterface, Transfer, transfer}};

use crate::error::ErrorCode;
use crate::state::offer::Offer;


#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info>{
    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        init,
        space = 8 + Offer::INIT_SPACE,
        payer = signer,
        seeds = [
            b"offer",
            id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub offer : Account<'info, Offer>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a : InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b : InterfaceAccount<'info, Mint>,

    #[account(
        init, 
        payer = signer,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = signer, 
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a : InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program : Program<'info, AssociatedToken>,

    pub token_program : Interface<'info, TokenInterface>, // Interafce means program with multiple possible ID's

    pub system_program : Program<'info, System>
}

pub fn make_offer(ctx : Context<MakeOffer>, id : u64, token_a_offered_amount : u64, token_b_wanted_amount : u64)-> Result<()>{
    let offer = &mut ctx.accounts.offer;

    //Validate accounts
    require!(token_a_offered_amount > 0, ErrorCode::InvalidAmount);
    require!(token_b_wanted_amount > 0, ErrorCode::InvalidAmount);

    //Check mints;
    require!(ctx.accounts.token_mint_a.key() != ctx.accounts.token_mint_b.key(), ErrorCode::InvalidTokenMint);

    //Transferring the token to the maker's ATA to the vault;
    let transfer_account  = Transfer{
        authority : ctx.accounts.signer.to_account_info(),
        from : ctx.accounts.maker_token_account_a.to_account_info(),
        to : ctx.accounts.vault.to_account_info()
    };

    let transfer_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_account);
    transfer(transfer_ctx, token_a_offered_amount)?;

    ctx.accounts.offer.set_inner(Offer { 
        id, maker: ctx.accounts.signer.key(), 
        token_mint_a: ctx.accounts.token_mint_a.key(), 
        token_mint_b: ctx.accounts.token_mint_b.key(), 
        token_b_wanted_amount, 
        bump: ctx.accounts.offer.bump 
    });

    Ok(())
}