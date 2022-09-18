//! # Option
//!
//! This module defines the CsvOption type

/// A CSV option
#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum CsvOption<T> {
    Some(T),
    None(NoneValue),
}

/// A dummy value to allow deserialization of `-`
#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum NoneValue {
    #[serde(rename = "-")]
    Null,
}

impl<T> CsvOption<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Some(v) => v,
            Self::None(_) => panic!("unwrap on a None value"),
        }
    }

    pub fn unwrap_or(self, alt: T) -> T {
        match self {
            Self::Some(v) => v,
            Self::None(_) => alt,
        }
    }

    /// Convert `CsvOption<T>` into a std `Option<T>`
    pub fn option(self) -> Option<T> {
        self.into()
    }
}

impl<T> From<CsvOption<T>> for Option<T> {
    fn from(opt: CsvOption<T>) -> Self {
        match opt {
            CsvOption::Some(v) => Some(v),
            CsvOption::None(_) => None,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::bitpanda::trade::Fiat;

    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    #[test]
    fn should_unwrap_csv_option() {
        assert_eq!(CsvOption::Some(Fiat::Eur).unwrap(), Fiat::Eur);
    }

    #[test]
    #[should_panic]
    fn should_panic_on_unwrap_csv_option() {
        CsvOption::None(NoneValue::Null).unwrap()
    }

    #[test]
    fn should_unwrap_csv_option_with_fallback() {
        assert_eq!(CsvOption::Some(Fiat::Eur).unwrap_or(Fiat::Usd), Fiat::Eur);
        assert_eq!(
            CsvOption::None(NoneValue::Null).unwrap_or(Fiat::Usd),
            Fiat::Usd
        );
    }

    #[test]
    fn should_convert_csv_option_to_option() {
        let opt: Option<Fiat> = CsvOption::Some(Fiat::Eur).into();
        assert_eq!(opt, Some(Fiat::Eur));
        let opt: Option<Fiat> = CsvOption::None(NoneValue::Null).into();
        assert_eq!(opt, None);
    }

    #[test]
    fn should_decode_csv_option() {
        let csv = r#"id,fiat
0,EUR
1,-
"#;
        let buffer = Cursor::new(csv);
        let mut reader = csv::Reader::from_reader(buffer);
        let mut fakes: Vec<CsvOption<Fiat>> = Vec::new();
        for result in reader.deserialize::<Fake>() {
            fakes.push(result.expect("failed to decode").fiat);
        }
        assert_eq!(
            fakes,
            vec![CsvOption::Some(Fiat::Eur), CsvOption::None(NoneValue::Null)]
        );
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Fake {
        id: u64,
        fiat: CsvOption<Fiat>,
    }
}
