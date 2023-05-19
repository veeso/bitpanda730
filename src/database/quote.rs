//! # Quote
//!
//! This module exposes the database for quotes based on assets

use chrono::{DateTime, FixedOffset, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::database::{TradeDatabase, TradeQuery};
use crate::finance::BitpandaClient;
use bitpanda_csv::Asset;

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
        let bitpanda = BitpandaClient::init(from, to).await?;
        debug!("collected {} assets from trades", assets.len());
        let mut quotes = HashMap::with_capacity(assets.len());
        // get prices
        Self::assets_price_from_bitpanda(&mut quotes, &bitpanda, &assets, to).await?;
        Ok(Self { quotes })
    }

    /// Get price for asset
    pub fn price(&self, asset: &Asset) -> Option<Decimal> {
        self.quotes.get(asset).cloned()
    }

    // -- loaders

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

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::database::DatabaseTradeMock;

    use chrono::prelude::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn should_load_quote_database() {
        let trades = DatabaseTradeMock::mock();
        assert!(
            QuoteDatabase::load(&trades, date(2021, 1, 1), date(2021, 12, 31))
                .await
                .is_ok()
        );
    }

    #[test]
    fn should_get_price() {
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
