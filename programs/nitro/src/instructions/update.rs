use anchor_spl::token::{
    freeze_account, mint_to, thaw_account, Burn, FreezeAccount, Mint, MintTo, ThawAccount, Token,
    TokenAccount,
};

use crate::*;

#[derive(Accounts)]
pub struct UpdateChannel<'info> {
    /// The channel containing the users and balance sheet
    #[account(mut,
        has_one = channel,
        has_one = channel_authority,
        has_one = share_mint,
        has_one = user_a_share_account,
        has_one = user_b_share_account,
        has_one = token_program,
    )]
    pub channel: AccountLoader<'info, Channel>,
    pub channel_authority: AccountInfo<'info>,

    #[account(mut)]
    pub share_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub user_a_share_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_b_share_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

impl<'info> UpdateChannel<'info> {
    fn thaw_context(
        &self,
        share_account: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, ThawAccount<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            ThawAccount {
                account: share_account.to_account_info(),
                mint: self.share_mint.to_account_info(),
                authority: self.channel_authority.to_account_info(),
            },
        )
    }

    fn mint_to_context(
        &self,
        share_account: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.share_mint.to_account_info(),
                to: share_account.to_account_info(),
                authority: self.channel_authority.to_account_info(),
            },
        )
    }

    fn burn_context(
        &self,
        share_account: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.share_mint.to_account_info(),
                authority: self.channel_authority.to_account_info(),
                from: share_account.to_account_info(),
            },
        )
    }

    fn freeze_context(
        &self,
        share_account: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, FreezeAccount<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            FreezeAccount {
                account: share_account.to_account_info(),
                mint: self.share_mint.to_account_info(),
                authority: self.channel_authority.to_account_info(),
            },
        )
    }

    pub fn update_shares(
        &self,
        signer_seeds: &[&[&[u8]]],
        share_account: &Account<'info, TokenAccount>,
        mint: u64,
        burn: u64,
    ) -> Result<()> {
        if mint == 0 && burn == 0 {
            return Ok(());
        }

        thaw_account(self.thaw_context(share_account).with_signer(signer_seeds))?;

        if mint > 0 {
            assert!(burn == 0);
            mint_to(self.mint_to_context(share_account), mint)?;
        } else if burn > 0 {
            anchor_spl::token::burn(self.burn_context(share_account), burn)?;
        }

        freeze_account(self.freeze_context(share_account))?;

        Ok(())
    }

    pub fn process(
        ctx: Context<UpdateChannel>,
        state: ChannelState,
        signatures: [Signature; NUM_USERS],
    ) -> Result<()> {
        let mut channel = ctx.accounts.channel.load_mut()?;

        channel.verify_ability_update()?;

        let transition = channel.state_transition(state, signatures);

        transition.valid_transition()?;

        channel.update(&state);

        let mint_amounts = transition.balance_changes();
        let signer_seeds: &[&[&[u8]]] = &[&channel.signer_seeds()];

        // Mint or burn share accounts for user_a
        ctx.accounts.update_shares(
            signer_seeds,
            &ctx.accounts.user_a_share_account,
            mint_amounts[0][0],
            mint_amounts[0][1],
        )?;

        // Mint or burn share accounts for user_b
        ctx.accounts.update_shares(
            signer_seeds,
            &ctx.accounts.user_b_share_account,
            mint_amounts[1][0],
            mint_amounts[1][1],
        )?;

        Ok(())
    }
}
