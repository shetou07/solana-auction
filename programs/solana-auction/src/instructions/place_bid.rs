use anchor_lang::prelude::*;

use crate::errors::AuctionError;
use crate::state::auction::Auction;

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(mut)]
    pub auction: Account<'info, Auction>,

    /// CHECK: placeholder wiring; transfer flow will be added with vault integration.
    #[account(mut)]
    pub previous_highest_bidder: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PlaceBid>, bid_amount: u64) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let auction = &mut ctx.accounts.auction;

    require!(!auction.is_finalized, AuctionError::AlreadyFinalized);
    require!(now >= auction.start_time, AuctionError::AuctionNotStarted);
    require!(now <= auction.end_time, AuctionError::AuctionEnded);
    require!(bid_amount > auction.highest_bid, AuctionError::BidTooLow);
    require!(
        bid_amount >= auction.reserve_price || auction.highest_bid >= auction.reserve_price,
        AuctionError::BidTooLow
    );

    if auction.highest_bidder != Pubkey::default() {
        require_keys_eq!(
            ctx.accounts.previous_highest_bidder.key(),
            auction.highest_bidder,
            AuctionError::InvalidPreviousBidder
        );
    }

    auction.highest_bid = bid_amount;
    auction.highest_bidder = ctx.accounts.bidder.key();

    Ok(())
}
