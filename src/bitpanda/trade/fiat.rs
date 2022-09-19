//! # Fiat
//!
//! FIAT currency definition

/// Defines the FIAT currency on Bitanda
#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
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
