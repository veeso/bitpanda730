//! # Quote
//!
//! Quote type

use bitpanda_api::model::ohlc::Ohlc;
use chrono::prelude::*;
use chrono::Utc;
use rust_decimal::Decimal;

/// The collection of quotes ASC sorted by date
pub struct Quotes {
    quotes: Vec<Quote>,
}

/// A symbol quotation in on a specific date
pub struct Quote {
    pub date: DateTime<Utc>,
    /// EUR price
    pub price: Decimal,
}

impl From<Ohlc> for Quote {
    fn from(value: Ohlc) -> Self {
        Self {
            date: value.time.with_timezone(&Utc),
            price: value.close,
        }
    }
}

impl From<Vec<Quote>> for Quotes {
    fn from(mut quotes: Vec<Quote>) -> Self {
        // sort ASC by date
        quotes.sort_by_key(|x| x.date);
        Self { quotes }
    }
}

impl Quotes {
    /// Get valid price for date
    pub fn price_at(&self, date: DateTime<Utc>) -> Decimal {
        // get last
        self.quotes
            .iter()
            .filter(|x| x.date <= date)
            .last()
            .map(|x| x.price)
            .unwrap_or_else(|| {
                self.quotes
                    .iter()
                    .map(|x| x.price)
                    .next()
                    .expect("quotes are empty")
            })
    }
}
