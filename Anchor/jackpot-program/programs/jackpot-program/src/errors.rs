use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Auction has ended.")]
    AuctionEnded,
    #[msg("Bid too low.")]
    BidTooLow,
    #[msg("Auction still running.")]
    AuctionStillRunning,
    #[msg("No bids placed.")]
    NoBidsPlaced,
    #[msg("Invalid highest bidder.")]
    InvalidHighestBidder,
    #[msg("Invalid auctioneer.")]
    InvalidAuctioneer,
}