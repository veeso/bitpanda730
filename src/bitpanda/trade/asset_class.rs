//! # AssetClass
//!
//! asset class types

/// Defines the asset class type, which is the asset group on Bitanda
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum AssetClass {
    Fiat,
    #[serde(rename = "Stock (derivative)")]
    Stock,
    Cryptocurrency,
    #[serde(rename = "ETF (derivative)")]
    Etf,
    Commodity,
    Metal,
}
