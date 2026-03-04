use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

declare_id!("FJTcpRNajvRKCkRhi984gxifiPWhgtmP7jqFCsuFC1QU");

#[program]
pub mod solana_auction {
    use super::*;

    pub fn initialize_auction(
        ctx: Context<InitializeAuction>,
        auction_id: u64,
        start_time: i64,
        end_time: i64,
        reserve_price: u64,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, start_time, end_time, reserve_price)
    }

    pub fn place_bid(
        ctx: Context<PlaceBid>,
        bid_amount: u64,
    ) -> Result<()> {
        instructions::place_bid::handler(ctx, bid_amount)
    }

    pub fn finalize_auction(ctx: Context<FinalizeAuction>) -> Result<()> {
        instructions::finalize::handler(ctx)
    }
<<<<<<< HEAD
}

#[account]
pub struct Auction {
    pub seller: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub reserve_price: u64,
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
    pub is_finalized: bool,
    pub bump: u8,
}

impl Auction {
    pub const LEN: usize = 8
        + 32
        + 8
        + 8
        + 8
        + 8
        + 32
        + 1
        + 1;
}

#[derive(Accounts)]
pub struct InitializeAuction<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init,
        payer = seller,
        space = Auction::LEN,
        seeds = [b"auction", seller.key().as_ref()],&auction_id.to_le_bytes()
        bump
    )]
    pub auction: Account<'info, Auction>,

    /// CHECK: vault holds SOL only
    #[account(
        mut,
        seeds = [b"vault", auction.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

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

    /// CHECK:
    #[account(
        mut,
        seeds = [b"vault", auction.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

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

    /// CHECK:
    #[account(
        mut,
        seeds = [b"vault", auction.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
}

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
}
=======
}
>>>>>>> a5b1595cd3244003227e612ceb63d9389c9aa697
