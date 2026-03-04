use anchor_lang::prelude::*;

#[account]
pub struct Auction {
    pub seller: Pubkey,
    pub auction_id: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub reserve_price: u64,
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
    pub is_finalized: bool,
    pub bump: u8,
    pub vault_bump: u8,
}

impl Auction {
    pub const LEN: usize = 8
        + 32
        + 8
        + 8
        + 8
        + 8
        + 8
        + 32
        + 1
        + 1
        + 1;
}