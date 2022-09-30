//! # Metal
//!
//! Defines metal asset types

/// Defines the metal asset.
#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum Metal {
    Gold,
    Palladium,
    Platinum,
    Silver,
}

impl ToString for Metal {
    fn to_string(&self) -> String {
        match self {
            Self::Gold => "XAU",
            Self::Palladium => "XPD",
            Self::Platinum => "XPT",
            Self::Silver => "XAG",
        }
        .to_string()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    #[test]
    fn should_decode_asset() {
        let csv = r#"id,metal
0,Gold
1,Palladium
2,Platinum
3,Silver
"#;
        let buffer = Cursor::new(csv);
        let mut reader = csv::Reader::from_reader(buffer);
        let mut fakes: Vec<Metal> = Vec::new();
        for result in reader.deserialize::<Fake>() {
            fakes.push(result.expect("failed to decode").metal);
        }
        assert_eq!(
            fakes,
            vec![
                Metal::Gold,
                Metal::Palladium,
                Metal::Platinum,
                Metal::Silver
            ]
        );
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Fake {
        id: u64,
        metal: Metal,
    }
}
