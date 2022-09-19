//! # Currency
//!
//! This module provides the currency type on bitpanda

use super::Fiat;

/// Defines the currency on Bitanda
#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "UPPERCASE", untagged)]
pub enum Currency {
    /// A fiat is a kind of currency
    Fiat(Fiat),
    Crypto(CryptoCurrency),
}

/// Defines the list of crypto currencies accepted for deposit/withdrawal on bitpanda
#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
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

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    #[test]
    fn should_decode_currency() {
        let csv = r#"id,currency
0,BTC
1,1INCH
2,ETH
3,USDT
4,EUR
5,USD
"#;
        let buffer = Cursor::new(csv);
        let mut reader = csv::Reader::from_reader(buffer);
        let mut fakes: Vec<Currency> = Vec::new();
        for result in reader.deserialize::<Fake>() {
            fakes.push(result.expect("failed to decode").currency);
        }
        assert_eq!(
            fakes,
            vec![
                Currency::Crypto(CryptoCurrency::Btc),
                Currency::Crypto(CryptoCurrency::OneInch),
                Currency::Crypto(CryptoCurrency::Eth),
                Currency::Crypto(CryptoCurrency::Usdt),
                Currency::Fiat(Fiat::Eur),
                Currency::Fiat(Fiat::Usd),
            ]
        );
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Fake {
        id: u64,
        currency: Currency,
    }
}
