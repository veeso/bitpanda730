//! # Option
//!
//! This module defines the CsvOption type

/// A CSV option
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum CsvOption<T> {
    Some(T),
    #[serde(rename = "-")]
    None,
}

impl<T> CsvOption<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Some(v) => v,
            Self::None => panic!("unwrap on a None value"),
        }
    }

    pub fn unwrap_or(self, alt: T) -> T {
        match self {
            Self::Some(v) => v,
            Self::None => alt,
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
            CsvOption::None => None,
        }
    }
}
