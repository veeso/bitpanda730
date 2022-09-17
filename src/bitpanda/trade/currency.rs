//! # Currency
//!
//! This module provides the currency type on bitpanda

use super::Fiat;

/// Defines the currency on Bitanda
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    /// A fiat is a kind of currency
    Fiat(Fiat),
    Crypto(CryptoCurrency),
}

/// Defines the list of crypto currencies accepted for deposit/withdrawal on bitpanda
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum CryptoCurrency {
    Btc,
    Eth,
    Usdt,
    Usdc,
    Xrp,
    Ada,
    Sol,
    Doge,
    Dot,
    Shib,
    Avax,
    Atom,
    Uni,
    Ltc,
    Link,
    Xlm,
    Bch,
    Vet,
    Ape,
    Eos,
    Xtz,
    Mana,
    Sand,
    Chz,
    Aave,
    Axs,
    Zec,
    Miota,
    Grt,
    Mkr,
    Neo,
    Snx,
    Crv,
    Enj,
    Bat,
    Dash,
    Waves,
    Lrc,
    Comp,
    Xem,
    #[serde(rename = "1INCH")]
    OneInch,
    Knc,
    Gala,
    Yfi,
    Omg,
    Rsr,
    Zrx,
    Srm,
    Ont,
    Uma,
    Sushi,
    Lsk,
    Best,
    Ren,
    Ocean,
    Rep,
    Euroc,
    Ant,
    Band,
    Kmd,
    Pan,
}
