//! # Quote
//!
//! This module exposes the database for quotes based on assets

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::bitpanda::trade::{Asset, AssetClass};
use crate::database::TradeDatabase;
use crate::finance::{BitpandaClient, YahooFinanceClient};

mod symbols;
use symbols::Symbols;

/// The quote database stores the asset quotations for all the symbols provided
pub struct QuoteDatabase {
    quotes: HashMap<Asset, Decimal>,
}

impl QuoteDatabase {
    /// Load quote database
    pub fn load(
        trades: &TradeDatabase,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> anyhow::Result<Self> {
        let yahoo_finance = Self::load_exchange(from, to)?;
        let bitpanda = BitpandaClient::new(from, to);
        let assets = trades.collect_assets();
        debug!("collected {} assets from trades", assets.len());
        let mut quotes = HashMap::with_capacity(assets.len());
        for asset in assets.into_iter() {
            quotes.insert(
                asset.0.clone(),
                Self::asset_price(&yahoo_finance, &bitpanda, asset.0, asset.1, to)?,
            );
        }
        Ok(Self { quotes })
    }

    /// Get price for asset
    pub fn price(&self, asset: Asset) -> Option<Decimal> {
        self.quotes.get(&asset).cloned()
    }

    // -- loaders

    fn load_exchange(from: DateTime<Utc>, to: DateTime<Utc>) -> anyhow::Result<YahooFinanceClient> {
        debug!("loading exchange in time range {} - {}", from, to);
        YahooFinanceClient::new(from, to)
    }

    /// Get asset price
    fn asset_price(
        yahoo_finance: &YahooFinanceClient,
        bitpanda: &BitpandaClient,
        asset: Asset,
        asset_class: AssetClass,
        price_at: DateTime<Utc>,
    ) -> anyhow::Result<Decimal> {
        debug!("looking up asset {:?}", asset);
        match asset_class {
            AssetClass::Commodity | AssetClass::Etf | AssetClass::Metal => {
                Self::asset_price_from_bitpanda(bitpanda, asset, price_at)
            }
            AssetClass::Fiat | AssetClass::Stock | AssetClass::Cryptocurrency => {
                Self::asset_price_from_yahoo(yahoo_finance, asset, price_at)
            }
        }
    }

    fn asset_price_from_yahoo(
        yahoo_finance: &YahooFinanceClient,
        asset: Asset,
        price_at: DateTime<Utc>,
    ) -> anyhow::Result<Decimal> {
        debug!("getting asset price from Yahoo");
        let symbol = Symbols::lookup(asset);
        debug!("got symbol {}", symbol);
        let quotation = yahoo_finance.get_symbol_quotes(&symbol)?;
        let price = quotation.price_at(price_at);
        debug!(
            "got quotation for {}; price at {}: {}",
            symbol, price_at, price
        );
        Ok(price)
    }

    fn asset_price_from_bitpanda(
        bitpanda: &BitpandaClient,
        asset: Asset,
        price_at: DateTime<Utc>,
    ) -> anyhow::Result<Decimal> {
        debug!("getting asset price from Bitpanda");
        let symbol = asset.to_string();
        let quotation = bitpanda.get_symbols_quotes(&symbol)?;
        let price = quotation.price_at(price_at);
        debug!(
            "got quotation for {}; price at {}: {}",
            symbol, price_at, price
        );
        Ok(price)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::database::DatabaseTradeMock;

    use chrono::prelude::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_load_quote_database() {
        let trades = DatabaseTradeMock::mock();
        assert!(QuoteDatabase::load(&trades, date(2021, 1, 1), date(2021, 12, 31)).is_ok());
    }

    #[test]
    fn should_get_price() {
        let mut quotes = HashMap::new();
        quotes.insert(Asset::Name(String::from("AMZN")), dec!(124.08));
        let db = QuoteDatabase { quotes };
        assert_eq!(
            db.price(Asset::Name(String::from("AMZN"))).unwrap(),
            dec!(124.08)
        );
        assert!(db.price(Asset::Name(String::from("ADBE"))).is_none());
    }

    fn date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.from_utc_datetime(&NaiveDate::from_ymd(year, month, day).and_hms(12, 0, 0))
    }
}
