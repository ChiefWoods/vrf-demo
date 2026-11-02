use anchor_lang::prelude::*;
use orao_solana_vrf::{
    cpi::accounts::RequestV2,
    program::OraoVrf,
    state::{NetworkState, RandomnessV2},
    CONFIG_ACCOUNT_SEED, RANDOMNESS_ACCOUNT_SEED,
};

#[derive(Accounts)]
#[instruction(seed: [u8; 32])]
pub struct RequestOrao<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [CONFIG_ACCOUNT_SEED],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub network_state: Account<'info, NetworkState>,
    /// CHECK: constrained to the SOL treasury stored on the Orao network state
    #[account(
        mut,
        constraint = network_state.config.treasury == treasury.key() @ ProgramError::InvalidArgument
    )]
    pub treasury: UncheckedAccount<'info>,
    /// CHECK: created by the Orao VRF program during the request CPI
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED, &seed],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub request: UncheckedAccount<'info>,
    pub vrf: Program<'info, OraoVrf>,
    pub system_program: Program<'info, System>,
}

pub fn request_orao(ctx: Context<RequestOrao>, seed: [u8; 32]) -> Result<()> {
    if seed == [0; 32] {
        return Err(ProgramError::InvalidArgument.into());
    }

    let cpi_accounts = RequestV2 {
        payer: ctx.accounts.payer.to_account_info(),
        network_state: ctx.accounts.network_state.to_account_info(),
        treasury: ctx.accounts.treasury.to_account_info(),
        request: ctx.accounts.request.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.vrf.key(), cpi_accounts);
    orao_solana_vrf::cpi::request_v2(cpi_context, seed)
}

#[derive(Accounts)]
pub struct ConsumeOrao<'info> {
    #[account(
        seeds = [RANDOMNESS_ACCOUNT_SEED, request.seed()],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub request: Account<'info, RandomnessV2>,
}

pub fn consume_orao(ctx: Context<ConsumeOrao>) -> Result<()> {
    let randomness = ctx
        .accounts
        .request
        .fulfilled()
        .ok_or(ProgramError::InvalidAccountData)?
        .randomness;

    msg!("Orao VRF randomness: {:?}", randomness);
    Ok(())
}
