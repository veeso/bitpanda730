//! # Asset
//!
//! asset defintion

use super::{CryptoCurrency, Currency};

use std::fmt;

mod metal;

pub use metal::Metal;

/// Defines the asset name. The asset can be a currency or an asset name (stock code)
#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Asset {
    Currency(Currency),
    Metal(Metal),
    /// A symbol name
    Ticker(String),
    /// Hong kong stock identifier
    HongKong(i64),
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = (match self {
            Self::Currency(Currency::Crypto(CryptoCurrency::OneInch)) => "1Inch".to_string(),
            Self::Currency(Currency::Crypto(x)) => format!("{:?}", x),
            Self::Currency(Currency::Fiat(x)) => format!("{:?}", x),
            Self::Metal(metal) => metal.to_string(),
            Self::Ticker(name) => name.to_string(),
            Self::HongKong(num) => num.to_string(),
        })
        .to_uppercase();
        write!(f, "{}", repr)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::bitpanda::trade::{CryptoCurrency, Fiat};

    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    #[test]
    fn should_convert_asset_to_string() {
        assert_eq!(
            Asset::Ticker(String::from("AMZN")).to_string().as_str(),
            "AMZN"
        );
        assert_eq!(Asset::HongKong(1197).to_string().as_str(), "1197");
        assert_eq!(
            Asset::Currency(Currency::Fiat(Fiat::Eur))
                .to_string()
                .as_str(),
            "EUR"
        );
        assert_eq!(
            Asset::Currency(Currency::Crypto(CryptoCurrency::Btc))
                .to_string()
                .as_str(),
            "BTC"
        );
        assert_eq!(
            Asset::Currency(Currency::Crypto(CryptoCurrency::OneInch))
                .to_string()
                .as_str(),
            "1INCH"
        );
        assert_eq!(Asset::Metal(Metal::Gold).to_string().as_str(), "XAU");
        assert_eq!(Asset::Metal(Metal::Silver).to_string().as_str(), "XAG");
        assert_eq!(Asset::Metal(Metal::Palladium).to_string().as_str(), "XPD");
        assert_eq!(Asset::Metal(Metal::Platinum).to_string().as_str(), "XPT");
    }

    #[test]
    fn should_decode_asset() {
        let csv = r#"id,asset
0,EUR
1,BTC
2,TSLA
3,Gold
4,1177
"#;
        let buffer = Cursor::new(csv);
        let mut reader = csv::Reader::from_reader(buffer);
        let mut fakes: Vec<Asset> = Vec::new();
        for result in reader.deserialize::<Fake>() {
            fakes.push(result.expect("failed to decode").asset);
        }
        assert_eq!(
            fakes,
            vec![
                Asset::Currency(Currency::Fiat(Fiat::Eur)),
                Asset::Currency(Currency::Crypto(CryptoCurrency::Btc)),
                Asset::Ticker("TSLA".to_string()),
                Asset::Metal(Metal::Gold),
                Asset::HongKong(1177),
            ]
        );
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Fake {
        id: u64,
        asset: Asset,
    }
}
