use crate::bitpanda::trade::{Asset, AssetClass, CryptoCurrency, Currency, Fiat};
use crate::database::TradeDatabase;
use crate::mock::bitpanda::TradeGenerator;

use chrono::DateTime;
use std::str::FromStr;

pub struct DatabaseTradeMock;

impl DatabaseTradeMock {
    pub fn mock() -> TradeDatabase {
        todo!("change to 2021");
        TradeDatabase::from(vec![
            TradeGenerator::deposit(
                DateTime::from_str("2022-01-06T12:32:24Z").unwrap(),
                dec!(10180.00),
                Fiat::Eur,
            ),
            TradeGenerator::buy(
                DateTime::from_str("2022-07-01T16:32:24Z").unwrap(),
                dec!(227.26),
                Fiat::Eur,
                dec!(1.0),
                Asset::Name(String::from("TSLA")),
                AssetClass::Stock,
                dec!(227.26),
            ),
            TradeGenerator::buy(
                DateTime::from_str("2022-07-01T16:32:24Z").unwrap(),
                dec!(328.68),
                Fiat::Eur,
                dec!(3.0),
                Asset::Name(String::from("AMZN")),
                AssetClass::Stock,
                dec!(109.56),
            ),
            TradeGenerator::buy(
                DateTime::from_str("2022-07-01T16:32:24Z").unwrap(),
                dec!(552.72),
                Fiat::Eur,
                dec!(1.5),
                Asset::Name(String::from("ADBE")),
                AssetClass::Stock,
                dec!(368.48),
            ),
            TradeGenerator::buy(
                DateTime::from_str("2022-07-01T16:32:24Z").unwrap(),
                dec!(250.00),
                Fiat::Eur,
                dec!(0.01354533),
                Asset::Currency(Currency::Crypto(CryptoCurrency::Btc)),
                AssetClass::Cryptocurrency,
                dec!(18456.54),
            ),
            TradeGenerator::buy(
                DateTime::from_str("2022-07-05T12:32:24Z").unwrap(),
                dec!(51.93),
                Fiat::Eur,
                dec!(80.0),
                Asset::HongKong(1177),
                AssetClass::Stock,
                dec!(0.65),
            ),
            TradeGenerator::sell(
                DateTime::from_str("2022-08-13T16:32:24Z").unwrap(),
                dec!(645.47),
                Fiat::Eur,
                dec!(0.01354533),
                Asset::Currency(Currency::Crypto(CryptoCurrency::Btc)),
                AssetClass::Cryptocurrency,
                dec!(23833.75),
            ),
            TradeGenerator::sell(
                DateTime::from_str("2022-08-16T16:32:24Z").unwrap(),
                dec!(289.56),
                Fiat::Eur,
                dec!(2.0),
                Asset::Name(String::from("AMZN")),
                AssetClass::Stock,
                dec!(144.78),
            ),
            TradeGenerator::sell(
                DateTime::from_str("2022-09-22T16:32:24Z").unwrap(),
                dec!(143.53),
                Fiat::Eur,
                dec!(0.5),
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
