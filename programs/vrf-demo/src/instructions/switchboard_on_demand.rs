use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;
#[cfg(feature = "devnet")]
use switchboard_on_demand::ON_DEMAND_DEVNET_PID;
#[cfg(not(feature = "devnet"))]
use switchboard_on_demand::ON_DEMAND_MAINNET_PID;

#[derive(Accounts)]
pub struct ConsumeSwitchboardOnDemand<'info> {
    /// CHECK: validated in handler
    pub randomness: UncheckedAccount<'info>,
}

pub fn consume_switchboard_on_demand(ctx: Context<ConsumeSwitchboardOnDemand>) -> Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "devnet")] {
            if ctx.accounts.randomness.owner.to_bytes() != ON_DEMAND_DEVNET_PID.to_bytes() {
                return Err(ProgramError::InvalidAccountOwner.into());
            }
        } else {
            if ctx.accounts.randomness.owner.to_bytes() != ON_DEMAND_MAINNET_PID.to_bytes() {
                return Err(ProgramError::InvalidAccountOwner.into());
            }
        }
    }

    let randomness = RandomnessAccountData::parse(ctx.accounts.randomness.try_borrow_data()?)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let value = randomness
        .get_value(Clock::get()?.slot)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    msg!("Switchboard On-Demand randomness: {:?}", value);
    Ok(())
}
