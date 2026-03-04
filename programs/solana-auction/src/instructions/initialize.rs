use anchor_lang::prelude::*;

use crate::errors::AuctionError;
use crate::state::auction::Auction;

#[derive(Accounts)]
pub struct InitializeAuction<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init,
        payer = seller,
        space = Auction::LEN,
        seeds = [b"auction", seller.key().as_ref()],
        bump
    )]
    pub auction: Account<'info, Auction>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeAuction>,
    start_time: i64,
    end_time: i64,
    reserve_price: u64,
) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;

    require!(end_time > start_time && end_time > now, AuctionError::InvalidTime);
    require!(reserve_price > 0, AuctionError::InvalidReserve);

    let auction = &mut ctx.accounts.auction;
    auction.seller = ctx.accounts.seller.key();
    auction.auction_id = 0;
    auction.start_time = start_time;
    auction.end_time = end_time;
    auction.reserve_price = reserve_price;
    auction.highest_bid = 0;
    auction.highest_bidder = Pubkey::default();
    auction.is_finalized = false;
    auction.bump = ctx.bumps.auction;
    auction.vault_bump = 0;

    Ok(())
}
