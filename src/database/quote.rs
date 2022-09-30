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
use symbols::YahooFinanceSymbols;

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
        debug!("sorting assets by exchange...");
        let assets = AssetsSortedByExchange::from(assets);
        debug!(
            "assets sorted by exchange; bitpanda: {}; yahoo: {}",
            assets.bitpanda.len(),
            assets.yahoo.len()
        );
        // get prices
        if !assets.bitpanda.is_empty() {
            Self::assets_price_from_bitpanda(&mut quotes, &bitpanda, &assets.bitpanda, to)?;
        }
        Self::assets_price_from_yahoo(&mut quotes, &yahoo_finance, &assets.yahoo, to)?;
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

    fn assets_price_from_yahoo(
        quotes: &mut HashMap<Asset, Decimal>,
        yahoo_finance: &YahooFinanceClient,
        assets: &[Asset],
        price_at: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        debug!("getting assets price from Yahoo");
        for asset in assets.iter() {
            let symbol = YahooFinanceSymbols::lookup(asset);
            debug!("got symbol {} for {:?}", symbol, asset);
            let quotation = yahoo_finance.get_symbol_quotes(&symbol)?;
            let price = quotation.price_at(price_at);
            debug!(
                "got quotation for {}; price at {}: {}",
                symbol, price_at, price
            );
            quotes.insert(asset.clone(), price);
        }

        Ok(())
    }

    fn assets_price_from_bitpanda(
        quotes: &mut HashMap<Asset, Decimal>,
        bitpanda: &BitpandaClient,
        assets: &[Asset],
        price_at: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        debug!("getting asset price from Bitpanda");
        let quotations = bitpanda.get_symbols_quotes(assets)?;
        for (asset, quotation) in quotations.into_iter() {
            let price = quotation.price_at(price_at);
            debug!(
                "got quotation for {}; price at {}: {}",
                asset.to_string(),
                price_at,
                price
            );
            quotes.insert(asset, price);
        }
        Ok(())
    }
}

/// A struct which contains the assets sorted by the exchange to query to get prices
#[derive(Default)]
struct AssetsSortedByExchange {
    bitpanda: Vec<Asset>,
    yahoo: Vec<Asset>,
}

impl From<Vec<(Asset, AssetClass)>> for AssetsSortedByExchange {
    fn from(assets: Vec<(Asset, AssetClass)>) -> Self {
        let mut sorted_assets = Self::default();
        for (asset, class) in assets.into_iter() {
            match class {
                AssetClass::Commodity | AssetClass::Etf | AssetClass::Metal => {
                    sorted_assets.bitpanda.push(asset);
                }
                AssetClass::Fiat | AssetClass::Stock | AssetClass::Cryptocurrency => {
                    sorted_assets.yahoo.push(asset);
                }
            }
        }
        sorted_assets
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
