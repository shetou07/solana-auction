use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use crate::errors::AuctionError;
use crate::state::auction::Auction;

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        mut,
        seeds = [b"auction", auction.seller.as_ref()],
        bump = auction.bump
    )]
    pub auction: Account<'info, Auction>,

    #[account(mut)]
    pub previous_highest_bidder: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", auction.key().as_ref()],
        bump = auction.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PlaceBid>, bid_amount: u64) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let auction = &mut ctx.accounts.auction;

    require!(!auction.is_finalized, AuctionError::AlreadyFinalized);
    require!(now >= auction.start_time, AuctionError::AuctionNotStarted);
    require!(now <= auction.end_time, AuctionError::AuctionEnded);
    require!(bid_amount > auction.highest_bid, AuctionError::BidTooLow);

    let bid_transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.bidder.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    );
    system_program::transfer(bid_transfer_ctx, bid_amount)?;

    if auction.highest_bidder != Pubkey::default() {
        require_keys_eq!(
            ctx.accounts.previous_highest_bidder.key(),
            auction.highest_bidder,
            AuctionError::InvalidPreviousBidder
        );

        let auction_key = auction.key();
        let vault_bump = auction.vault_bump;
        let vault_seeds: &[&[u8]] = &[b"vault", auction_key.as_ref(), &[vault_bump]];
        let signer_seeds: &[&[&[u8]]] = &[vault_seeds];
        let refund_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.previous_highest_bidder.to_account_info(),
            },
            signer_seeds,
        );
        system_program::transfer(refund_ctx, auction.highest_bid)?;
    }

    auction.highest_bid = bid_amount;
    auction.highest_bidder = ctx.accounts.bidder.key();

    Ok(())
}
