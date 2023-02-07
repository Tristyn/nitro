use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Invalid nonce")]
    InvalidNonce,
    #[msg("Invalid channel user")]
    InvalidUser,
    #[msg("Unexpected balances")]
    UnexpectedBalances,
    #[msg("Unexpected challenge duration")]
    UnexpectedChallengeDuration,
    #[msg("Channel is active")]
    ChannelActive,
    #[msg("Channel is not active")]
    ChannelNotActive,
    #[msg("Channel signature invalid")]
    ChannelSignatureInvalid,
    #[msg("Channel public key invalid")]
    ChannelPublicKeyInvalid,
    #[msg("Channel signature does not match public key")]
    ChannelSignatureDoesNotMatchPublicKey,
    #[msg("The update would cause total liquidity would change")]
    TotalLiquidityChanged,
    #[msg("User A pubkey is off curve and can not create channel signatures. PDA accounts are not supported.")]
    UserAPubKeyOffCurve,
    #[msg("User B pubkey is off curve and can not create channel signatures. PDA accounts are not supported.")]
    UserBPubKeyOffCurve,
    #[msg("Channel not finalizing")]
    ChannelNotFinalizing,
    #[msg("The challenge period has not elapsed")]
    ChallengePeriodIncomplete,
}
