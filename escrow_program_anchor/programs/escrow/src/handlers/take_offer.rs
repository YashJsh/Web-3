use anchor_lang::prelude::*;
use crate::{state::offer::Offer, error::ErrorCode};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, CloseAccount, close_account, Transfer, TransferChecked, transfer_checked}
};

#[derive(Accounts)]
pub struct TakeOffer<'info>{
    pub associated_token_program : Program<'info, AssociatedToken>,

    pub token_program : Interface<'info, TokenInterface>,

    pub system_program : Program<'info, System>,

    #[account(mut)]
    pub taker : Signer<'info>,

    #[account(mut)]
    pub maker : SystemAccount<'info>,

    pub token_mint_a : InterfaceAccount<'info, Mint>,
    pub token_mint_b : InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_a_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_b_account : InterfaceAccount<'info , TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_b,
        associated_token::authority = maker, 
        associated_token::token_program = token_program
    )]
    pub maker_token_account_b : InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_mint_b,
        seeds = [b"offer", offer.id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    offer : Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info, TokenAccount>,
}

//This instruction will perform two functions : 
//1. Sending the wanted token to the users;
//2. Widthdrawing the offered tokens to the vault to the taker and closing the vault.
pub fn take_offer(ctx : Context<TakeOffer>)-> Result<()>{
    //Check for sufficient baalnce;
    require!(ctx.accounts.offer.token_b_wanted_amount <= ctx.accounts.taker_token_b_account.amount, ErrorCode::InsufficientTokenBalance);

    let offer_account_seeds : &[&[u8]] = &[
        b"offer",
        ctx.accounts.offer.id.to_le_bytes().as_ref(),
        &[ctx.accounts.offer.bump]
    ];

     //Transfer the tokens from taker to the maker.
    let transfer_accounts = TransferChecked{
        authority : ctx.accounts.taker.to_account_info(),
        from : ctx.accounts.taker_token_b_account.to_account_info(),
        to : ctx.accounts.maker_token_account_b.to_account_info(),
        mint : ctx.accounts.token_mint_b.to_account_info(),
    };
    
    let cpi_transfer_token_ctx = CpiContext::new(
        tx.accounts.token_program.to_account_info(), 
        transfer_accounts
    );
    
    transfer_checked(
        cpi_transfer_token_ctx, 
        ctx.accounts.offer.token_b_wanted_amount, 
        ctx.accounts.token_mint_b.decimals
    )?;

    

    //withdraw the offered tokens from vault to the taker account
    let transfer_account = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        to: ctx.accounts.taker_token_a_account.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };

    let transfer_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), transfer_account, &[offer_account_seeds]);

    transfer_checked(transfer_ctx, ctx.accounts.vault.amount, ctx.accounts.token_mint_a.decimals)?;

    //Close account and return the rent to the user.
    let close_accounts = CloseAccount{
        account : ctx.accounts.vault.to_account_info(),
        authority : ctx.accounts.offer.to_account_info(),
        destination : ctx.accounts.maker.to_account_info()
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        close_accounts, 
        &[offer_seeds]
    );

    close_account(cpi_context)?;

    Ok(())
}