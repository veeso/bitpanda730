//! # Fiat
//!
//! FIAT currency definition

/// Defines the FIAT currency on Bitanda
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Fiat {
    Chf,
    Czk,
    Dkk,
    Eur,
    Gbp,
    Huf,
    Pln,
    Sek,
    Try,
    Usd,
}
