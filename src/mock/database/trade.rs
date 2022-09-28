use crate::bitpanda::trade::{Asset, AssetClass, CryptoCurrency, Currency, Fiat};
use crate::database::TradeDatabase;
use crate::mock::bitpanda::TradeGenerator;

use chrono::DateTime;
use std::str::FromStr;

pub struct DatabaseTradeMock;

impl DatabaseTradeMock {
    pub fn mock() -> TradeDatabase {
        TradeDatabase::from(vec![
            TradeGenerator::deposit(
                DateTime::from_str("2021-01-06T12:32:24Z").unwrap(),
                dec!(10180.00),
                Fiat::Eur,
            ),
            TradeGenerator::buy(
                DateTime::from_str("2021-07-01T16:32:24Z").unwrap(),
                dec!(225.97),
                Fiat::Eur,
                dec!(1.0),
                Asset::Name(String::from("TSLA")),
                AssetClass::Stock,
                dec!(225.97),
            ),
            TradeGenerator::buy(
                DateTime::from_str("2021-07-01T16:32:24Z").unwrap(),
                dec!(514.25),
                Fiat::Eur,
                dec!(3.0),
                Asset::Name(String::from("AMZN")),
                AssetClass::Stock,
                dec!(171.65),
            ),
            TradeGenerator::buy(
                DateTime::from_str("2021-07-01T16:32:24Z").unwrap(),
                dec!(877.01),
                Fiat::Eur,
                dec!(1.5),
                Asset::Name(String::from("ADBE")),
                AssetClass::Stock,
                dec!(584.73),
            ),
            TradeGenerator::buy(
                DateTime::from_str("2021-07-01T16:32:24Z").unwrap(),
                dec!(454.75),
                Fiat::Eur,
                dec!(0.01354533),
                Asset::Currency(Currency::Crypto(CryptoCurrency::Btc)),
                AssetClass::Cryptocurrency,
                dec!(33572.12),
            ),
            TradeGenerator::buy(
                DateTime::from_str("2021-07-05T12:32:24Z").unwrap(),
                dec!(586.40),
                Fiat::Eur,
                dec!(80.0),
                Asset::HongKong(1177),
                AssetClass::Stock,
                dec!(7.33),
            ),
            TradeGenerator::sell(
                DateTime::from_str("2021-08-13T16:32:24Z").unwrap(),
                dec!(593.26),
                Fiat::Eur,
                dec!(0.01354533),
                Asset::Currency(Currency::Crypto(CryptoCurrency::Btc)),
                AssetClass::Cryptocurrency,
                dec!(43798.12),
            ),
            TradeGenerator::sell(
                DateTime::from_str("2021-08-16T16:32:24Z").unwrap(),
                dec!(334.50),
                Fiat::Eur,
                dec!(2.0),
                Asset::Name(String::from("AMZN")),
                AssetClass::Stock,
                dec!(167.25),
            ),
            TradeGenerator::sell(
                DateTime::from_str("2021-09-22T16:32:24Z").unwrap(),
                dec!(313.04),
                Fiat::Eur,
                dec!(0.5),
                Asset::Name(String::from("ADBE")),
                AssetClass::Stock,
                dec!(626.08),
            ),
            TradeGenerator::withdrawal(
                DateTime::from_str("2021-09-24T12:32:24Z").unwrap(),
                dec!(500.00),
                Fiat::Eur,
            ),
        ])
    }
}
