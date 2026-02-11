use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MasterKey{
    pub highest_bid : u64,
    pub highest_bidder : Pubkey,
    pub auction_end_time : i64,
    pub bump : u8,
    pub treasury_bump : u8
}

#[account]
pub struct Treasury {}