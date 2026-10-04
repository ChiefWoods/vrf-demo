use anchor_lang::prelude::*;

pub const MAGICBLOCK_REQUEST_SEED: &[u8] = b"magicblock-request";

#[account]
#[derive(InitSpace)]
pub struct MagicblockRequest {
    pub payer: Pubkey,
    pub seed: [u8; 32],
    pub randomness: Option<[u8; 32]>,
}
