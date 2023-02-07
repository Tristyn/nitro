use crate::*;

#[derive(Accounts)]
pub struct FinalizeChannel<'info> {
    /// The channel containing the users and balance sheet
    #[account(mut,
      has_one = channel,
    )]
    pub channel: AccountLoader<'info, Channel>,

    pub signer: Signer<'info>,
}

impl<'info> FinalizeChannel<'info> {
    pub fn process(ctx: Context<FinalizeChannel>, nonce: u64) -> Result<()> {
        let mut channel = ctx.accounts.channel.load_mut()?;

        channel.verify_ability_update()?;
        channel.verify_nonce(nonce)?;
        channel.verify_user(ctx.accounts.signer.key())?;

        channel.outcome = ChannelOutcome::Finalizing;

        channel.finalize_for_user(ctx.accounts.signer.key());

        Ok(())
    }
}
