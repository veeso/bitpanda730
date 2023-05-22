//! # Quote
//!
//! This module exposes the database for quotes based on assets

use bitpanda_csv::AssetClass;
use chrono::{DateTime, FixedOffset, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::database::{TradeDatabase, TradeQuery};
use crate::finance::{BitpandaClient, YahooFinanceClient};
use bitpanda_csv::Asset;

mod symbols;
use symbols::YahooFinanceSymbols;

/// The quote database stores the asset quotations for all the symbols provided
pub struct QuoteDatabase {
    quotes: HashMap<Asset, Decimal>,
}

impl QuoteDatabase {
    /// Load quote database
    pub async fn load(
        trades: &TradeDatabase,
        from: DateTime<FixedOffset>,
        to: DateTime<FixedOffset>,
    ) -> anyhow::Result<Self> {
        let assets = trades
            .select(TradeQuery::default().after(from).before(to))
            .collect_assets();
        let from = DateTime::from(from);
        let to = DateTime::from(to);
        let yahoo_finance = YahooFinanceClient::new(from, to)?;
        let bitpanda = BitpandaClient::init(from, to).await?;
        debug!("collected {} assets from trades", assets.len());
        let mut quotes = HashMap::with_capacity(assets.len());
        debug!("sorting assets by exchange...");
        let assets = AssetsSortedByExchange::from(assets);
        // get prices
        Self::assets_price_from_bitpanda(&mut quotes, &bitpanda, &assets.bitpanda, to).await?;
        Self::assets_price_from_yahoo(&mut quotes, &yahoo_finance, &assets.yahoo, to)?;
        Ok(Self { quotes })
    }

    /// Get price for asset
    pub fn price(&self, asset: &Asset) -> Option<Decimal> {
        self.quotes.get(asset).cloned()
    }

    // -- loaders

    fn assets_price_from_yahoo(
        quotes: &mut HashMap<Asset, Decimal>,
        yahoo_finance: &YahooFinanceClient,
        assets: &[Asset],
        price_at: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        debug!("getting assets price from Yahoo");
        for asset in assets.iter() {
            let symbol = YahooFinanceSymbols::lookup(asset);
            debug!("got symbol {} for {}", symbol, asset);
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

    async fn assets_price_from_bitpanda(
        quotes: &mut HashMap<Asset, Decimal>,
        bitpanda: &BitpandaClient,
        assets: &[Asset],
        price_at: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        debug!("getting asset price from Bitpanda");
        let quotations = bitpanda.get_symbols_quotes(assets).await?;
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
            match (asset, class) {
                // yahoo is just for fiat
                (asset, AssetClass::Fiat) => {
                    sorted_assets.yahoo.push(asset);
                }
                (asset, _) => {
                    sorted_assets.bitpanda.push(asset);
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

    #[tokio::test]
    async fn should_load_quote_database() {
        crate::mock::log();
        let trades = DatabaseTradeMock::mock();
        assert!(
            QuoteDatabase::load(&trades, date(2022, 1, 1), date(2022, 12, 31))
                .await
                .is_ok()
        );
    }

    #[test]
    fn should_get_price() {
        crate::mock::log();
        let mut quotes = HashMap::new();
        quotes.insert(Asset::Ticker(String::from("AMZN")), dec!(124.08));
        let db = QuoteDatabase { quotes };
        assert_eq!(
            db.price(&Asset::Ticker(String::from("AMZN"))).unwrap(),
            dec!(124.08)
        );
        assert!(db.price(&Asset::Ticker(String::from("ADBE"))).is_none());
    }

    fn date(year: i32, month: u32, day: u32) -> DateTime<FixedOffset> {
        FixedOffset::west_opt(3600)
            .unwrap()
            .with_ymd_and_hms(year, month, day, 12, 0, 0)
            .unwrap()
    }
}
