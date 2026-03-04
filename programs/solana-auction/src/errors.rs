use anchor_lang::prelude::*;

#[error_code]
pub enum AuctionError {
    #[msg("Invalid time settings")]
    InvalidTime,
    #[msg("Invalid reserve price")]
    InvalidReserve,
    #[msg("Auction has not started")]
    AuctionNotStarted,
    #[msg("Auction has ended")]
    AuctionEnded,
    #[msg("Auction not ended")]
    AuctionNotEnded,
    #[msg("Bid too low")]
    BidTooLow,
    #[msg("Already finalized")]
    AlreadyFinalized,
    #[msg("Invalid previous bidder account")]
    InvalidPreviousBidder,
}