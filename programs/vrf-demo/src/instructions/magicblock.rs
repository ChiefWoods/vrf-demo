use crate::{MagicblockRequest, MAGICBLOCK_REQUEST_SEED};
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{
    anchor::{vrf, vrf_callback},
    vrf::{
        consts::DEFAULT_QUEUE,
        instructions::{create_request_randomness_ix, RequestRandomnessParams},
    },
};

#[vrf]
#[derive(Accounts)]
#[instruction(seed: [u8; 32])]
pub struct RequestMagicblock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: validated in constraint
    #[account(
        mut,
        constraint = oracle_queue.key().to_bytes() == DEFAULT_QUEUE.to_bytes() @ ProgramError::InvalidArgument
    )]
    pub oracle_queue: UncheckedAccount<'info>,
    #[account(init, payer = payer, space = 8 + MagicblockRequest::INIT_SPACE, seeds = [MAGICBLOCK_REQUEST_SEED, payer.key().as_ref(), seed.as_ref()], bump)]
    pub request: Account<'info, MagicblockRequest>,
    pub system_program: Program<'info, System>,
}

pub fn request_magicblock(ctx: Context<RequestMagicblock>, seed: [u8; 32]) -> Result<()> {
    let ix = create_request_randomness_ix(RequestRandomnessParams {
        payer: ctx.accounts.payer.key(),
        oracle_queue: ctx.accounts.oracle_queue.key(),
        callback_program_id: crate::ID,
        callback_discriminator: crate::instruction::ConsumeMagicblock::DISCRIMINATOR.to_vec(),
        caller_seed: seed,
        accounts_metas: Some(vec![
            ephemeral_rollups_sdk::vrf::types::SerializableAccountMeta {
                pubkey: ctx.accounts.request.key(),
                is_signer: false,
                is_writable: true,
            },
        ]),
        callback_args: Some(seed.to_vec()),
        ..Default::default()
    });

    ctx.accounts.request.set_inner(MagicblockRequest {
        payer: ctx.accounts.payer.key(),
        seed,
        randomness: None,
    });
    Ok(ctx
        .accounts
        .invoke_signed_vrf(&ctx.accounts.payer.to_account_info(), &ix)?)
}

#[vrf_callback]
#[derive(Accounts)]
pub struct ConsumeMagicblock<'info> {
    #[account(mut)]
    pub request: Account<'info, MagicblockRequest>,
}

pub fn consume_magicblock(
    ctx: Context<ConsumeMagicblock>,
    randomness: [u8; 32],
    seed: [u8; 32],
) -> Result<()> {
    if ctx.accounts.request.seed != seed {
        return Err(ProgramError::InvalidInstructionData.into());
    }
    ctx.accounts.request.randomness = Some(randomness);
    msg!("MagicBlock VRF randomness: {:?}", randomness);
    Ok(())
}
