//! # Currency
//!
//! This module provides the currency type on bitpanda

use super::{CryptoCurrency, Fiat};

/// Defines the currency on Bitanda
#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "UPPERCASE", untagged)]
pub enum Currency {
    /// A fiat is a kind of currency
    Fiat(Fiat),
    Crypto(CryptoCurrency),
}

impl Currency {
    /// Returns whether the currency is FIAT
    pub fn is_fiat(&self) -> bool {
        matches!(self, Currency::Fiat(_))
    }

    /// Returns whether the currency is a crypto
    pub fn is_crypto(&self) -> bool {
        matches!(self, Currency::Crypto(_))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    #[test]
    fn should_tell_whether_is_fiat() {
        assert_eq!(Currency::Fiat(Fiat::Eur).is_fiat(), true);
        assert_eq!(Currency::Crypto(CryptoCurrency::Best).is_fiat(), false);
    }

    #[test]
    fn should_tell_whether_is_crypto() {
        assert_eq!(Currency::Fiat(Fiat::Eur).is_crypto(), false);
        assert_eq!(Currency::Crypto(CryptoCurrency::Best).is_crypto(), true);
    }

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
