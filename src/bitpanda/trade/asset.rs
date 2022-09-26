//! # Asset
//!
//! asset defintion

use super::Currency;

/// Defines the asset name. The asset can be a currency or an asset name (stock code)
#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Asset {
    Currency(Currency),
    Name(String),
    HongKong(i64),
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::bitpanda::trade::{CryptoCurrency, Fiat};

    use pretty_assertions::assert_eq;
    use std::io::Cursor;

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
                Asset::Name("TSLA".to_string()),
                Asset::Name("Gold".to_string()),
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
