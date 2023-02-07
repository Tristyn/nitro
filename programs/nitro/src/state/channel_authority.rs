use crate::*;

/// Interface for the authority program
pub trait ChannelAuthority {
    /// The validTransition function accepts two states, and returns true if
    /// the transition is valid between them. As well as defining the valid
    /// transitions, the function above implicitly defines the valid states and
    /// their format: in order to assess whether a transition is valid, the
    /// function will first need to decode the data passed in; incorrectly
    /// formatted states or states with invalid data will lead to failure. The
    /// set of valid states is therefore defined to be the set of states which
    /// form part of some valid transition.
    fn valid_transition(channel: &AccountLoader<Channel>, a: &[u8], b: &[u8]) -> Result<()>;
}
