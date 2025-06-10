use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account(discriminator = 1)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey, // Wallet that created the escrow
    pub mint_a: Pubkey, // SPL mint addresses for the give side of the swap
    pub mint_b: Pubkey, // SPL mint addresses for the get side of the swap
    pub receive: u64, // How much of token b the maker wants
    pub bump: u8,
}

