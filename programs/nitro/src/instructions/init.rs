use anchor_spl::token::{
    freeze_account, mint_to, set_authority, spl_token::instruction::AuthorityType, transfer,
    FreezeAccount, Mint, MintTo, SetAuthority, Token, TokenAccount, Transfer,
};

use crate::*;

#[derive(Accounts)]
pub struct InitChannel<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The channel containing the users and balance sheet
    #[account(init,
      payer = payer,
      space = 8 + std::mem::size_of::<Channel>(),
    )]
    pub channel: AccountLoader<'info, Channel>,
    #[account(
      seeds = [
        seeds::AUTHORITY.as_ref(),
        channel.key().as_ref(),
      ],
      bump
    )]
    pub channel_authority: AccountInfo<'info>,

    pub user_a: AccountInfo<'info>,
    pub user_b: AccountInfo<'info>,

    #[account(init,
      payer = payer,
      seeds = [
        seeds::SHARE_MINT.as_ref(),
        channel.key().as_ref()
      ],
      bump,
      mint::authority = channel_authority,
      mint::decimals = liquidity_mint.decimals,
    )]
    pub share_mint: Box<Account<'info, Mint>>,
    #[account(init,
      payer = payer,
      seeds = [
        seeds::SHARE_ACCOUNT.as_ref(),
        channel.key().as_ref(),
        [0u8].as_ref(),
      ],
      bump,
      token::mint = share_mint,
      token::authority = channel_authority,
    )]
    pub user_a_share_account: Box<Account<'info, TokenAccount>>,
    #[account(init,
      payer = payer,
      seeds = [
        seeds::SHARE_ACCOUNT.as_ref(),
        channel.key().as_ref(),
        [0u8].as_ref(),
      ],
      bump,
      token::mint = share_mint,
      token::authority = channel_authority,
    )]
    pub user_b_share_account: Box<Account<'info, TokenAccount>>,

    /// The token mint which the channel will transact
    pub liquidity_mint: Box<Account<'info, Mint>>,

    /// The token account that will custody funds
    #[account(init,
      payer = payer,
      token::mint = liquidity_mint,
      token::authority = channel_authority
    )]
    pub liquidity_account: Box<Account<'info, TokenAccount>>,

    pub funder: Signer<'info>,
    #[account(mut)]
    pub funder_token_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitChannel<'info> {
    fn set_authority_context(
        &self,
        account: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            SetAuthority {
                current_authority: self.channel_authority.to_account_info(),
                account_or_mint: account.to_account_info(),
            },
        )
    }

    fn freeze_context(
        &self,
        account: &Account<'info, TokenAccount>,
        mint: &Account<'info, Mint>,
    ) -> CpiContext<'_, '_, '_, 'info, FreezeAccount<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            FreezeAccount {
                account: account.to_account_info(),
                mint: mint.to_account_info(),
                authority: self.channel_authority.to_account_info(),
            },
        )
    }

    fn mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.share_mint.to_account_info(),
                to: self.user_a_share_account.to_account_info(),
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
        ctx: Context<InitChannel>,
        balances: [u64; NUM_USERS],
        challenge_duration: u64,
    ) -> Result<()> {
        let mut channel = ctx.accounts.channel.load_init()?;

        *channel = Channel {
            channel: ctx.accounts.channel.key(),
            channel_authority: ctx.accounts.channel_authority.key(),
            payer: ctx.accounts.payer.key(),
            liquidity_mint: ctx.accounts.liquidity_mint.key(),
            liquidity_account: ctx.accounts.liquidity_account.key(),
            user_a: ctx.accounts.user_a.key(),
            user_b: ctx.accounts.user_b.key(),
            share_mint: ctx.accounts.share_mint.key(),
            user_a_share_account: ctx.accounts.user_a_share_account.key(),
            user_b_share_account: ctx.accounts.user_b_share_account.key(),
            token_program: ctx.accounts.token_program.key(),
            share_token_program: ctx.accounts.token_program.key(),
            challenge_duration,
            challenge_period: 0,
            channel_authority_bump: [ctx.bumps["channel_authority"]],
            outcome: ChannelOutcome::New,
            user_a_finalizing: false,
            user_b_finalizing: false,
            _reserved: [0; 4],
            state: ChannelState { nonce: 0, balances },
            reserved: [0; 64],
        };

        channel.verify_user_pubkeys_on_curve()?;

        let signer_seeds: &[&[&[u8]]] = &[&channel.signer_seeds()];

        // Transfer liquidity from user_a equal to his balance
        transfer(ctx.accounts.fund_liquidity_context(), balances[0])?;

        // Mint share tokens for funded liquidity.
        // user_b has not funded the channel,
        // so they do not get shares yet.
        mint_to(
            ctx.accounts.mint_to_context().with_signer(signer_seeds),
            balances[0],
        )?;

        // Grant close account authority over share accounts to the channel
        set_authority(
            ctx.accounts
                .set_authority_context(&ctx.accounts.user_a_share_account)
                .with_signer(signer_seeds),
            AuthorityType::CloseAccount,
            Some(ctx.accounts.channel_authority.key()),
        )?;
        set_authority(
            ctx.accounts
                .set_authority_context(&ctx.accounts.user_b_share_account)
                .with_signer(signer_seeds),
            AuthorityType::CloseAccount,
            Some(ctx.accounts.channel_authority.key()),
        )?;

        // Freeze the share accounts
        freeze_account(
            ctx.accounts
                .freeze_context(&ctx.accounts.user_a_share_account, &ctx.accounts.share_mint)
                .with_signer(signer_seeds),
        )?;
        freeze_account(
            ctx.accounts
                .freeze_context(&ctx.accounts.user_b_share_account, &ctx.accounts.share_mint)
                .with_signer(signer_seeds),
        )?;

        // Give the users ownership of the share accounts
        set_authority(
            ctx.accounts
                .set_authority_context(&ctx.accounts.user_a_share_account)
                .with_signer(signer_seeds),
            AuthorityType::AccountOwner,
            Some(ctx.accounts.user_a.key()),
        )?;
        set_authority(
            ctx.accounts
                .set_authority_context(&ctx.accounts.user_b_share_account)
                .with_signer(signer_seeds),
            AuthorityType::AccountOwner,
            Some(ctx.accounts.user_a.key()),
        )?;

        let user_a_balance = channel.state.balances[0];
        assert!(user_a_balance == ctx.accounts.liquidity_account.amount);

        Ok(())
    }
}
