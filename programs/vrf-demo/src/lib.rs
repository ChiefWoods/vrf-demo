pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

#[allow(ambiguous_glob_reexports)]
pub use instructions::*;
pub use state::*;

declare_id!("E7pq92NEL96KLPSRowyr7Q54VZAUrS2pCZngiLPheSUe");

#[program]
pub mod vrf_demo {
    use super::*;

    pub fn request_magicblock(ctx: Context<RequestMagicblock>, seed: [u8; 32]) -> Result<()> {
        instructions::magicblock::request_magicblock(ctx, seed)
    }
    pub fn consume_magicblock(
        ctx: Context<ConsumeMagicblock>,
        randomness: [u8; 32],
        seed: [u8; 32],
    ) -> Result<()> {
        instructions::magicblock::consume_magicblock(ctx, randomness, seed)
    }
    pub fn consume_switchboard_on_demand(ctx: Context<ConsumeSwitchboardOnDemand>) -> Result<()> {
        instructions::switchboard_on_demand::consume_switchboard_on_demand(ctx)
    }
    pub fn request_orao(ctx: Context<RequestOrao>, seed: [u8; 32]) -> Result<()> {
        instructions::orao::request_orao(ctx, seed)
    }
    pub fn consume_orao(ctx: Context<ConsumeOrao>) -> Result<()> {
        instructions::orao::consume_orao(ctx)
    }
    pub fn verify_blueshift_ecvrf(
        ctx: Context<VerifyBlueshiftEcvrf>,
        public_key: [u8; 32],
        proof: [u8; 80],
        alpha: Vec<u8>,
    ) -> Result<()> {
        instructions::blueshift_ecvrf::verify(ctx, public_key, proof, alpha)
    }
}
