//! # Symbols
//!
//! This module provides lookup for the yahoo api queries

use crate::bitpanda::trade::{Asset, CryptoCurrency, Currency, Fiat};

/// Symbols lookup resolver
pub struct Symbols;

impl Symbols {
    /// Get yahoo finance name for an asset
    pub fn lookup(asset: Asset) -> String {
        match asset {
            Asset::Currency(Currency::Crypto(CryptoCurrency::Aave)) => "AAVE-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Ada)) => "ADA-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Ant)) => "ANT-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Ape)) => "APE-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Atom)) => "ATOM-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Avax)) => "AVAX-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Axs)) => "AXS-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Band)) => "BAND-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Bat)) => "BAT-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Bch)) => "BCH-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Best)) => "BEST-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Btc)) => "BTC-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Chz)) => "CHZ-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Comp)) => "COMP-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Crv)) => "CRV-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Dash)) => "DASH-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Doge)) => "DOGE-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Dot)) => "DOT-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Enj)) => "ENJ-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Eos)) => "EOS-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Eth)) => "ETH-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Euroc)) => "EUROC-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Gala)) => "GALA-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Grt)) => "GRT-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Kmd)) => "KMD-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Knc)) => "KNC-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Link)) => "LINK-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Lrc)) => "LRC-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Lsk)) => "LSK-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Ltc)) => "LTC-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Mana)) => "MANA-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Miota)) => "MIOTA-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Mkr)) => "MKR-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Neo)) => "NEO-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Ocean)) => "OCEAN-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Omg)) => "OMG-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::OneInch)) => "1INCH-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Ont)) => "ONT-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Pan)) => "PAN-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Ren)) => "REN-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Rep)) => "REP-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Rsr)) => "RSR-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Sand)) => "SAND-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Shib)) => "SHIB-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Snx)) => "SNX-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Sol)) => "SOL-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Srm)) => "SRM-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Sushi)) => "SUSHI-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Uma)) => "UMA-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Uni)) => "UNI-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Usdc)) => "USDC-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Usdt)) => "USDT-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Vet)) => "VET-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Waves)) => "WAVES-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Xem)) => "XEM-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Xlm)) => "XLM-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Xrp)) => "XRP-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Xtz)) => "XTZ-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Yfi)) => "YFI-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Zec)) => "ZEC-USD".to_string(),
            Asset::Currency(Currency::Crypto(CryptoCurrency::Zrx)) => "ZRX-USD".to_string(),
            Asset::Currency(Currency::Fiat(Fiat::Chf)) => "USDCHF=x".to_string(),
            Asset::Currency(Currency::Fiat(Fiat::Czk)) => "USDCZK=x".to_string(),
            Asset::Currency(Currency::Fiat(Fiat::Dkk)) => "USDDKK=x".to_string(),
            Asset::Currency(Currency::Fiat(Fiat::Eur)) => "USDEUR=x".to_string(),
            Asset::Currency(Currency::Fiat(Fiat::Gbp)) => "USDGBP=x".to_string(),
            Asset::Currency(Currency::Fiat(Fiat::Huf)) => "USDHUF=x".to_string(),
            Asset::Currency(Currency::Fiat(Fiat::Pln)) => "USDPLN=x".to_string(),
            Asset::Currency(Currency::Fiat(Fiat::Sek)) => "USDSEK=x".to_string(),
            Asset::Currency(Currency::Fiat(Fiat::Try)) => "USDTRY=x".to_string(),
            Asset::Currency(Currency::Fiat(Fiat::Usd)) => "USDT-USD".to_string(),
            Asset::Name(name) => name,
            Asset::HongKong(id) => format!("{}.HK", id),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_resolve_asset_to_symbol() {
        assert_eq!(
            Symbols::lookup(Asset::Currency(Currency::Crypto(CryptoCurrency::Btc))).as_str(),
            "BTC-USD"
        );
        assert_eq!(
            Symbols::lookup(Asset::Name(String::from("AMZN"))).as_str(),
            "AMZN"
        );
        assert_eq!(Symbols::lookup(Asset::HongKong(1177)).as_str(), "1177.HK");
    }
}
