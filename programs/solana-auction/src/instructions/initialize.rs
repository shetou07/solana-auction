use anchor_lang::prelude::*;

#[account(
    init,
    payer = seller,
    space = Auction::LEN,
    seeds = [
        b"auction",
        seller.key().as_ref(),
        &auction_id.to_le_bytes()
    ],
    bump
)]
pub auction: Account<'info, Auction>,