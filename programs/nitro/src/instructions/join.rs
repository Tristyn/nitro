use anchor_spl::token::{
    freeze_account, mint_to, thaw_account, transfer, FreezeAccount, Mint, MintTo, ThawAccount,
    Token, TokenAccount, Transfer,
};

use crate::*;

#[derive(Accounts)]
pub struct JoinChannel<'info> {
    /// The channel containing the users and balance sheet
    #[account(mut,
        has_one = channel,
        has_one = channel_authority,
        has_one = user_a,
        has_one = user_b,
        has_one = share_mint,
        has_one = user_b_share_account,
        has_one = liquidity_mint,
        has_one = liquidity_account,
        has_one = token_program,
    )]
    pub channel: AccountLoader<'info, Channel>,
    pub channel_authority: AccountInfo<'info>,

    pub user_a: AccountInfo<'info>,
    pub user_b: AccountInfo<'info>,

    #[account(mut)]
    pub share_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub user_b_share_account: Box<Account<'info, TokenAccount>>,

    /// The token mint which the channel will transact
    pub liquidity_mint: Box<Account<'info, Mint>>,

    /// The token account that will custody funds
    #[account(mut)]
    pub liquidity_account: Box<Account<'info, TokenAccount>>,

    pub funder: Signer<'info>,
    #[account(mut)]
    pub funder_token_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> JoinChannel<'info> {
    fn thaw_context(&self) -> CpiContext<'_, '_, '_, 'info, ThawAccount<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            ThawAccount {
                account: self.user_b_share_account.to_account_info(),
                mint: self.share_mint.to_account_info(),
                authority: self.channel_authority.to_account_info(),
            },
        )
    }

    fn mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.share_mint.to_account_info(),
                to: self.user_b_share_account.to_account_info(),
                authority: self.channel_authority.to_account_info(),
            },
        )
    }

    fn freeze_context(&self) -> CpiContext<'_, '_, '_, 'info, FreezeAccount<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            FreezeAccount {
                account: self.user_b_share_account.to_account_info(),
                mint: self.share_mint.to_account_info(),
                authority: self.channel_authority.to_account_info(),
            },
        )
    }

    fn fund_liquidity_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.funder_token_account.to_account_info(),
                to: self.liquidity_account.to_account_info(),
                authority: self.funder.to_account_info(),
            },
        )
    }

    pub fn process(
        ctx: Context<JoinChannel>,
        balances: [u64; NUM_USERS],
        challenge_duration: u64,
    ) -> Result<()> {
        let mut channel = ctx.accounts.channel.load_mut()?;

        channel.verify_ability_join()?;
        require!(
            channel.state.balances == balances,
            Errors::UnexpectedBalances
        );
        require!(
            channel.challenge_duration == challenge_duration,
            Errors::UnexpectedChallengeDuration
        );

        channel.outcome = ChannelOutcome::Active;

        let signer_seeds: &[&[&[u8]]] = &[&channel.signer_seeds()];

        // Transfer liquidity from user_b equal to his balance
        transfer(
            ctx.accounts.fund_liquidity_context(),
            channel.state.balances[1],
        )?;

        // Unthaw the share account for user_b
        thaw_account(ctx.accounts.thaw_context().with_signer(signer_seeds))?;

        // Mint share tokens for funded liquidity.
        mint_to(
            ctx.accounts.mint_to_context().with_signer(signer_seeds),
            channel.state.balances[1],
        )?;

        // Freeze the share accounts
        freeze_account(ctx.accounts.freeze_context().with_signer(signer_seeds))?;

        let user_a_balance = channel.state.balances[0];
        let user_b_balance = channel.state.balances[1];
        assert!(user_a_balance + user_b_balance == ctx.accounts.liquidity_account.amount);

        Ok(())
    }
}
