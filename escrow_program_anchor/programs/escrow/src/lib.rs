pub mod state;
pub mod handlers;
pub mod error;

use anchor_lang::prelude::*;
use crate::handlers::make_offer::{MakeOffer, make_offer as make_offer_handler};

declare_id!("EzGAFZDUdtWKXhYkXu8P2LgMno95gBqzACFook4kAgcg");

#[program]
pub mod escrow {
    use super::*;
    pub fn offer(ctx : Context<MakeOffer>, id : u64, token_a_offered_amount : u64, token_b_wanted_amount : u64)-> Result<()>{
        make_offer_handler(ctx, id, token_a_offered_amount, token_b_wanted_amount)
    }
}

