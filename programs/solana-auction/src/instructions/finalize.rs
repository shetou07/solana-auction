use anchor_lang::prelude::*;

use crate::errors::AuctionError;
use crate::state::auction::Auction;

#[derive(Accounts)]
pub struct FinalizeAuction<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut, has_one = seller)]
    pub auction: Account<'info, Auction>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<FinalizeAuction>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let auction = &mut ctx.accounts.auction;

    require!(!auction.is_finalized, AuctionError::AlreadyFinalized);
    require!(now > auction.end_time, AuctionError::AuctionNotEnded);

    auction.is_finalized = true;
    Ok(())
}
