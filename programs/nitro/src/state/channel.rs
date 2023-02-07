use crate::*;
use bytemuck::{Pod, Zeroable};
use ed25519_dalek::Verifier;

pub const NUM_USERS: usize = 2;
pub const SIGNATURE_LEN: u64 = 64;

#[account(zero_copy)]
pub struct Channel {
    pub channel: Pubkey,
    pub channel_authority: Pubkey,
    pub payer: Pubkey,
    pub liquidity_mint: Pubkey,
    pub liquidity_account: Pubkey,
    pub user_a: Pubkey,
    pub user_b: Pubkey,
    pub share_mint: Pubkey,
    pub user_a_share_account: Pubkey,
    pub user_b_share_account: Pubkey,
    pub token_program: Pubkey,
    pub share_token_program: Pubkey,
    pub challenge_duration: u64,
    pub challenge_period: u64,
    pub channel_authority_bump: [u8; 1],
    pub outcome: ChannelOutcome,
    pub user_a_finalizing: bool,
    pub user_b_finalizing: bool,
    pub _reserved: [u8; 4],
    pub state: ChannelState,
    pub reserved: [u8; 64],
}

impl Channel {
    pub fn signer_seeds(&self) -> [&[u8]; 3] {
        [
            seeds::AUTHORITY.as_ref(),
            self.channel.as_ref(),
            self.channel_authority_bump.as_ref(),
        ]
    }

    /// Verifies that `user_a` and `user_b` are on curve. The public keys must
    /// be on a curve for the private keys to have the ability to produce
    /// channel signatures.
    pub fn verify_user_pubkeys_on_curve(&self) -> Result<()> {
        require!(self.user_a.is_on_curve(), Errors::UserAPubKeyOffCurve);
        require!(self.user_b.is_on_curve(), Errors::UserBPubKeyOffCurve);
        Ok(())
    }

    pub fn verify_ability_join(&self) -> Result<()> {
        require!(
            self.outcome == ChannelOutcome::New,
            Errors::ChannelNotActive
        );
        Ok(())
    }

    pub fn verify_ability_update(&self) -> Result<()> {
        require!(
            self.outcome == ChannelOutcome::Active || self.outcome == ChannelOutcome::Finalizing,
            Errors::ChannelNotActive
        );
        Ok(())
    }

    pub fn verify_ability_distribute(&self) -> Result<()> {
        require!(
            self.outcome == ChannelOutcome::Finalizing,
            Errors::ChannelNotFinalizing
        );
        require!(
            Clock::get().unwrap().slot >= self.challenge_period,
            Errors::ChallengePeriodIncomplete
        );
        Ok(())
    }

    pub fn verify_nonce(&self, nonce: u64) -> Result<()> {
        require!(self.state.nonce == nonce, Errors::InvalidNonce);
        Ok(())
    }

    pub fn verify_user(&self, signer: Pubkey) -> Result<()> {
        require!(
            self.user_a == signer || self.user_b == signer,
            Errors::InvalidUser
        );
        Ok(())
    }

    pub fn state_transition(
        &self,
        state: ChannelState,
        signatures: [Signature; NUM_USERS],
    ) -> StateTransition {
        StateTransition {
            users: [self.user_a, self.user_b],
            states: [self.state, state],
            signatures,
        }
    }

    pub fn update(&mut self, state: &ChannelState) {
        if state.nonce > self.state.nonce {
            self.challenge_period = 0;
            self.outcome = ChannelOutcome::Active;
            self.user_a_finalizing = false;
            self.user_b_finalizing = false;
        }
        self.state = state.clone();
    }

    pub fn finalize_for_user(&mut self, user: Pubkey) {
        if self.outcome == ChannelOutcome::Active {
            self.outcome = ChannelOutcome::Finalizing;
            self.challenge_period = Clock::get().unwrap().slot + self.challenge_duration;
        }
        if self.user_a == user {
            self.user_a_finalizing = true;
        }
        if self.user_b == user {
            self.user_b_finalizing = true;
        }
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ChannelState {
    pub nonce: u64,
    pub balances: [u64; NUM_USERS],
}

pub struct StateTransition {
    pub users: [Pubkey; NUM_USERS],
    pub states: [ChannelState; 2],
    pub signatures: [Signature; NUM_USERS],
}

impl StateTransition {
    pub fn launch_distribute(&self) {}
    pub fn distribute(&self) {}

    pub fn launch_challenge(&self) -> Result<()> {
        Ok(())
    }

    pub fn counter_challenge(&self) -> Result<()> {
        Ok(())
    }

    pub fn valid_transition(&self) -> Result<()> {
        require!(
            self.states[1].nonce >= self.states[0].nonce,
            Errors::InvalidNonce
        );

        let before_liquidity: u64 = self.states[0].balances.iter().sum();
        let after_liquidity: u64 = self.states[1].balances.iter().sum();
        require!(
            before_liquidity == after_liquidity,
            Errors::TotalLiquidityChanged
        );

        let sign_data = bytemuck::bytes_of(&self.states[1]);

        for (i, user) in self.users.iter().enumerate() {
            let signature = self.signatures[i];
            verify_signature(&signature, sign_data, user)?;
        }
        Ok(())
    }

    pub fn balance_changes(&self) -> [[u64; 2]; NUM_USERS] {
        let mut changes: [[u64; 2]; NUM_USERS] = [[0; 2]; NUM_USERS];
        for i in 0..NUM_USERS {
            let before = self.states[0].balances[i];
            let after = self.states[1].balances[i];

            let mint = after.saturating_sub(before);
            let burn = before.saturating_sub(after);

            changes[i] = [mint, burn];
        }
        changes
    }
}

#[cfg(not(target_os = "solana"))]
fn verify_signature(signature: &Signature, sign_date: &[u8], public_key: &Pubkey) -> Result<()> {
    let signature = ed25519_dalek::Signature::from_bytes(&signature.0)
        .map_err(|_| Errors::ChannelSignatureInvalid)?;
    let public_key = ed25519_dalek::PublicKey::from_bytes(public_key.as_ref())
        .map_err(|_| Errors::ChannelSignatureInvalid)?;
    public_key
        .verify(sign_date, &signature)
        .map_err(|_| Errors::ChannelSignatureDoesNotMatchPublicKey)?;
    Ok(())
}

#[cfg(target_os = "solana")]
const CURVE_ID_EDWARDS: u64 = 0;

#[cfg(target_os = "solana")]
unsafe fn verify_signature(
    signature: &Signature,
    sign_date: &[u8],
    public_key: &Pubkey,
) -> Result<()> {
    let mut result = [0u8; 32];
    require!(
        syscalls::sol_curve_group_op(
            0,
            2,
            public_key.as_ref().as_ptr(),
            signature.0.as_ptr(),
            result.as_mut_ptr()
        ) == 0,
        Errors::ChannelSignatureDoesNotMatchPublicKey
    );
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct Signature([u8; 64]);

impl Default for Signature {
    fn default() -> Self {
        Self([0; 64])
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum ChannelOutcome {
    New = 0,
    Active = 1,
    Finalizing = 2,
}

unsafe impl Zeroable for ChannelOutcome {
    fn zeroed() -> Self {
        ChannelOutcome::New
    }
}

unsafe impl Pod for ChannelOutcome {}
