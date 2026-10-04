use anchor_lang::prelude::*;
use solana_ecvrf::{Proof, PublicKey};

#[derive(Accounts)]
pub struct VerifyBlueshiftEcvrf<'info> {
    pub system_program: Program<'info, System>,
}

pub fn verify(
    _ctx: Context<VerifyBlueshiftEcvrf>,
    public_key: [u8; 32],
    proof: [u8; 80],
    alpha: Vec<u8>,
) -> Result<()> {
    let randomness = Proof(proof)
        .verify(&PublicKey(public_key), &alpha)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    msg!("Blueshift ECVRF randomness: {:?}", randomness);
    Ok(())
}
