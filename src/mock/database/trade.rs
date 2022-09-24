use crate::bitpanda::trade::{Asset, AssetClass, CryptoCurrency, Currency, Fiat};
use crate::database::TradeDatabase;
use crate::mock::bitpanda::TradeGenerator;

use chrono::DateTime;
use rust_decimal_macros::dec;
use std::str::FromStr;

pub struct DatabaseTradeMock;

impl DatabaseTradeMock {
    pub fn mock() -> TradeDatabase {
        TradeDatabase::from(vec![
            TradeGenerator::deposit(
                DateTime::from_str("2022-06-01T12:32:24Z").unwrap(),
                dec!(2036.00),
                Fiat::Eur,
            ),
            TradeGenerator::buy(
                DateTime::from_str("2022-07-01T16:32:24Z").unwrap(),
                dec!(100.00),
                Fiat::Eur,
                Asset::Name(String::from("TSLA")),
                AssetClass::Stock,
                dec!(227.26),
            ),
            TradeGenerator::buy(
                DateTime::from_str("2022-07-01T16:32:24Z").unwrap(),
                dec!(300.00),
                Fiat::Eur,
                Asset::Name(String::from("AMZN")),
                AssetClass::Stock,
                dec!(109.56),
            ),
            TradeGenerator::buy(
                DateTime::from_str("2022-07-01T16:32:24Z").unwrap(),
                dec!(400.00),
                Fiat::Eur,
                Asset::Name(String::from("ADBE")),
                AssetClass::Stock,
                dec!(368.48),
            ),
            TradeGenerator::buy(
                DateTime::from_str("2022-07-01T16:32:24Z").unwrap(),
                dec!(250.00),
                Fiat::Eur,
                Asset::Currency(Currency::Crypto(CryptoCurrency::Btc)),
                AssetClass::Cryptocurrency,
                dec!(18456.54),
            ),
            TradeGenerator::sell(
                DateTime::from_str("2022-08-13T16:32:24Z").unwrap(),
                dec!(645.47),
                Fiat::Eur,
                Asset::Currency(Currency::Crypto(CryptoCurrency::Btc)),
                AssetClass::Cryptocurrency,
                dec!(23833.75),
            ),
            TradeGenerator::sell(
                DateTime::from_str("2022-08-16T16:32:24Z").unwrap(),
                dec!(396.44),
                Fiat::Eur,
                Asset::Name(String::from("AMZN")),
                AssetClass::Stock,
                dec!(144.78),
            ),
            TradeGenerator::sell(
                DateTime::from_str("2022-09-22T16:32:24Z").unwrap(),
                dec!(311.61),
                Fiat::Eur,
                Asset::Name(String::from("ADBE")),
                AssetClass::Stock,
                dec!(287.06),
            ),
            TradeGenerator::withdrawal(
                DateTime::from_str("2022-09-24T12:32:24Z").unwrap(),
                dec!(500.00),
                Fiat::Eur,
            ),
        ])
    }
}
