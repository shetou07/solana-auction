use anchor_lang::prelude::*;

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
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        require!(end_time > start_time, AuctionError::InvalidTime);
        require!(end_time > current_time, AuctionError::InvalidTime);
        require!(reserve_price > 0, AuctionError::InvalidReserve);

        let auction = &mut ctx.accounts.auction;

        auction.seller = ctx.accounts.seller.key();
        auction.start_time = start_time;
        auction.end_time = end_time;
        auction.reserve_price = reserve_price;
        auction.highest_bid = 0;
        auction.highest_bidder = Pubkey::default();
        auction.is_finalized = false;
        auction.bump = ctx.bumps.auction;

        Ok(())
    }

    pub fn place_bid(
        ctx: Context<PlaceBid>,
        bid_amount: u64,
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;

        require!(now >= auction.start_time, AuctionError::AuctionNotStarted);
        require!(now < auction.end_time, AuctionError::AuctionEnded);
        require!(!auction.is_finalized, AuctionError::AlreadyFinalized);
        require!(bid_amount > auction.highest_bid, AuctionError::BidTooLow);
        require!(bid_amount >= auction.reserve_price, AuctionError::BidTooLow);

        // Transfer SOL from bidder to vault
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.bidder.key(),
            &ctx.accounts.vault.key(),
            bid_amount,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[
                ctx.accounts.bidder.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        )?;

        // Refund previous bidder if exists
        if auction.highest_bid > 0 {
            **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= auction.highest_bid;
            **ctx.accounts.bidder.to_account_info().try_borrow_mut_lamports()? += auction.highest_bid;
        }

        auction.highest_bid = bid_amount;
        auction.highest_bidder = ctx.accounts.bidder.key();

        Ok(())
    }

    pub fn finalize_auction(ctx: Context<FinalizeAuction>) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;

        require!(now >= auction.end_time, AuctionError::AuctionNotEnded);
        require!(!auction.is_finalized, AuctionError::AlreadyFinalized);

        if auction.highest_bid > 0 {
            **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= auction.highest_bid;
            **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += auction.highest_bid;
        }

        auction.is_finalized = true;

        Ok(())
    }
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
