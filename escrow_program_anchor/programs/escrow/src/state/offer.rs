use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Offer{
    pub id : u64,
    pub maker : Pubkey,
    pub token_mint_a : Pubkey,
    pub token_mint_b : Pubkey,
    pub token_b_wanted_amount : Pubkey,
    pub bump : u8,
}