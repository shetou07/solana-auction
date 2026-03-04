use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

declare_id!("BZMTC2BweEXZQBTxJyNZGKhRtocmihezzyR6fEWVV92m");

#[program]
pub mod solana_auction {
    use super::*;

    pub fn initialize_auction(
        ctx: Context<InitializeAuction>,
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
}