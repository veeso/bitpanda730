//! # Types
//!
//! Bitpanda API types

use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Bitpanda year is returned by the Asset GET.
/// The response has the following syntax
/// ```json
/// {
///  "data": {
///    "BTC": [
///      {
///        "type": "candle",
///        "attributes": {
///          "open": "19993.5",
///          "high": "20230.4",
///          "close": "19665.7",
///          "low": "19386.2",
///          "time": {
///            "date_iso8601": "2022-09-29T02:00:00+02:00",
///            "unix": "1664409600"
///          }
///        }
///      }
///    ],
///    "ETH": [
///      {
///        "type": "candle",
///        "attributes": {
///          "open": "19993.5",
///          "high": "20230.4",
///          "close": "19665.7",
///          "low": "19386.2",
///          "time": {
///            "date_iso8601": "2022-09-29T02:00:00+02:00",
///            "unix": "1664409600"
///          }
///        }
///      }
///    ]
///  }
///}
/// ```
#[derive(Debug, Deserialize)]
pub struct BitpandaYear {
    pub data: HashMap<String, Vec<BitpandaPrice>>,
}

#[derive(Debug, Deserialize)]
pub struct BitpandaPrice {
    #[serde(rename = "type")]
    pub type_: String,
    pub attributes: BitpandaPriceAttributes,
}

#[derive(Debug, Deserialize)]
pub struct BitpandaPriceAttributes {
    pub close: Decimal,
    pub time: BitpandaPriceAttributesTime,
}

#[derive(Debug, Deserialize)]
pub struct BitpandaPriceAttributesTime {
    pub date_iso8601: DateTime<FixedOffset>,
}
