//! # Transaction type
//!
//! Transaction type definition

/// Defines the `TransactionType` in the bitpanda trade
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    /// Currency deposited to bitpanda
    Deposit,
    /// The buy operation of an asset
    Buy,
    /// An asset transferred from Bitpanda to your wallet (e.g. BEST rewards, staking rewards, ...)
    Transfer,
    /// A sell operation of an asset
    Sell,
    /// A withdrawal of a currency (NOTE: can be FIAT or Crypto).
    /// A bitpanda Card transaction is a Withdrawal too.
    Withdrawal,
}
