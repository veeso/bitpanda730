//! # AssetClass
//!
//! asset class types

/// Defines the asset class type, which is the asset group on Bitanda
#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
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

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    #[test]
    fn should_decode_asset_class() {
        let csv = r#"id,asset_class
0,Fiat
1,Stock (derivative)
2,Cryptocurrency
3,ETF (derivative)
4,Commodity
5,Metal
"#;
        let buffer = Cursor::new(csv);
        let mut reader = csv::Reader::from_reader(buffer);
        let mut fakes: Vec<AssetClass> = Vec::new();
        for result in reader.deserialize::<Fake>() {
            fakes.push(result.expect("failed to decode").asset_class);
        }
        assert_eq!(
            fakes,
            vec![
                AssetClass::Fiat,
                AssetClass::Stock,
                AssetClass::Cryptocurrency,
                AssetClass::Etf,
                AssetClass::Commodity,
                AssetClass::Metal
            ]
        );
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Fake {
        id: u64,
        asset_class: AssetClass,
    }
}
