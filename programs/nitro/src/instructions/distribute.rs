use anchor_spl::token::{
    burn, close_account, thaw_account, transfer, Burn, CloseAccount, Mint, ThawAccount, Token,
    TokenAccount, Transfer,
};

use crate::*;

#[derive(Accounts)]
pub struct DistributeChannel<'info> {
    /// The channel containing the users and balance sheet
    #[account(mut,
        has_one = channel,
        has_one = channel_authority,
        has_one = payer,
        has_one = user_a,
        has_one = user_b,
        has_one = share_mint,
        has_one = user_a_share_account,
        has_one = user_b_share_account,
        has_one = liquidity_account,
        has_one = token_program,
        close = payer,
    )]
    pub channel: AccountLoader<'info, Channel>,
    pub channel_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: AccountInfo<'info>,

    pub user_a: AccountInfo<'info>,
    pub user_b: AccountInfo<'info>,

    #[account(mut)]
    pub share_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub user_a_share_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_b_share_account: Box<Account<'info, TokenAccount>>,

    /// The token account that will custody funds
    #[account(mut)]
    pub liquidity_account: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        token::authority = user_a,
    )]
    pub user_a_liquidity_account: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        token::authority = user_b,
    )]
    pub user_b_liquidity_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> DistributeChannel<'info> {
    fn thaw_share_context(
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

    fn burn_share_account_context(
        &self,
        share_account: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.share_mint.to_account_info(),
                from: share_account.to_account_info(),
                authority: self.channel_authority.to_account_info(),
            },
        )
    }

    fn close_share_account_context(
        &self,
        share_account: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            CloseAccount {
                account: share_account.to_account_info(),
                destination: self.payer.to_account_info(),
                authority: self.channel_authority.to_account_info(),
            },
        )
    }

    fn transfer_liquidity_context(
        &self,
        to: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.liquidity_account.to_account_info(),
                to: to.to_account_info(),
                authority: self.channel_authority.to_account_info(),
            },
        )
    }

    fn close_liquidity_account_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.liquidity_account.to_account_info(),
                destination: self.payer.to_account_info(),
                authority: self.channel_authority.to_account_info(),
            },
        )
    }

    pub fn process(ctx: Context<DistributeChannel>) -> Result<()> {
        let channel = ctx.accounts.channel.load()?;

        channel.verify_ability_distribute()?;

        let signer_seeds: &[&[&[u8]]] = &[&channel.signer_seeds()];

        // Close user a share account
        thaw_account(
            ctx.accounts
                .thaw_share_context(&ctx.accounts.user_a_share_account)
                .with_signer(signer_seeds),
        )?;
        burn(
            ctx.accounts
                .burn_share_account_context(&ctx.accounts.user_a_share_account)
                .with_signer(signer_seeds),
            ctx.accounts.user_a_share_account.amount,
        )?;
        close_account(
            ctx.accounts
                .close_share_account_context(&ctx.accounts.user_a_share_account)
                .with_signer(signer_seeds),
        )?;

        // Close user b share account
        thaw_account(
            ctx.accounts
                .thaw_share_context(&ctx.accounts.user_b_share_account)
                .with_signer(signer_seeds),
        )?;
        burn(
            ctx.accounts
                .burn_share_account_context(&ctx.accounts.user_b_share_account)
                .with_signer(signer_seeds),
            ctx.accounts.user_b_share_account.amount,
        )?;
        close_account(
            ctx.accounts
                .close_share_account_context(&ctx.accounts.user_b_share_account)
                .with_signer(signer_seeds),
        )?;

        let user_a_balance = channel.state.balances[0];
        let user_b_balance = channel.state.balances[1];

        assert!(user_a_balance + user_b_balance == ctx.accounts.liquidity_account.amount);

        if user_a_balance > 0 {
            transfer(
                ctx.accounts
                    .transfer_liquidity_context(&ctx.accounts.user_a_liquidity_account)
                    .with_signer(signer_seeds),
                user_a_balance,
            )?;
        }

        if user_b_balance > 0 {
            transfer(
                ctx.accounts
                    .transfer_liquidity_context(&ctx.accounts.user_b_liquidity_account)
                    .with_signer(signer_seeds),
                user_b_balance,
            )?;
        }

        close_account(
            ctx.accounts
                .close_liquidity_account_context()
                .with_signer(signer_seeds),
        )?;

        Ok(())
    }
}
