use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use crate::errors::AuctionError;
use crate::state::auction::Auction;

#[derive(Accounts)]
pub struct FinalizeAuction<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        has_one = seller,
        seeds = [b"auction", seller.key().as_ref()],
        bump = auction.bump
    )]
    pub auction: Account<'info, Auction>,

    #[account(
        mut,
        seeds = [b"vault", auction.key().as_ref()],
        bump = auction.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(mut)]
    pub highest_bidder: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<FinalizeAuction>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let auction = &mut ctx.accounts.auction;

    require!(!auction.is_finalized, AuctionError::AlreadyFinalized);
    require!(now > auction.end_time, AuctionError::AuctionNotEnded);

    let auction_key = auction.key();
    let vault_bump = auction.vault_bump;
    let vault_seeds: &[&[u8]] = &[b"vault", auction_key.as_ref(), &[vault_bump]];
    let signer_seeds: &[&[&[u8]]] = &[vault_seeds];

    if auction.highest_bidder == Pubkey::default() || auction.highest_bid == 0 {
        auction.is_finalized = true;
        return Ok(());
    }

    require_keys_eq!(
        ctx.accounts.highest_bidder.key(),
        auction.highest_bidder,
        AuctionError::InvalidHighestBidder
    );

    let payout_target = if auction.highest_bid >= auction.reserve_price {
        ctx.accounts.seller.to_account_info()
    } else {
        ctx.accounts.highest_bidder.to_account_info()
    };

    let payout_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: payout_target,
        },
        signer_seeds,
    );
    system_program::transfer(payout_ctx, auction.highest_bid)?;

    auction.is_finalized = true;
    Ok(())
}
