use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod seeds;
pub mod state;
pub mod syscalls;

pub use errors::*;
pub use instructions::*;
pub use seeds::*;
pub use state::*;
pub use syscalls::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nitro {
    use super::*;

    pub fn init(
        ctx: Context<InitChannel>,
        balances: [u64; NUM_USERS],
        challenge_duration: u64,
    ) -> Result<()> {
        InitChannel::process(ctx, balances, challenge_duration)
    }

    pub fn join(
        ctx: Context<JoinChannel>,
        balances: [u64; NUM_USERS],
        challenge_duration: u64,
    ) -> Result<()> {
        JoinChannel::process(ctx, balances, challenge_duration)
    }

    pub fn update(
        ctx: Context<UpdateChannel>,
        state: ChannelState,
        signatures: [Signature; NUM_USERS],
    ) -> Result<()> {
        UpdateChannel::process(ctx, state, signatures)
    }

    pub fn finalize(ctx: Context<FinalizeChannel>, nonce: u64) -> Result<()> {
        FinalizeChannel::process(ctx, nonce)
    }

    pub fn distribute(ctx: Context<DistributeChannel>) -> Result<()> {
        DistributeChannel::process(ctx)
    }
}
