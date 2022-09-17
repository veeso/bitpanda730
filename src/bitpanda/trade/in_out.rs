//! # InOut
//!
//! The direction of a transaction

/// Defines the direction of a trade on bitpanda
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InOut {
    Incoming,
    Outgoing,
}
