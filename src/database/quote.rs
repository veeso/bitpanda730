//! # Quote
//!
//! This module exposes the database for quotes based on assets

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::bitpanda::trade::Asset;
use crate::database::TradeDatabase;
use crate::finance::Exchange;

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
        let exchange = Self::load_exchange(from, to)?;
        let assets = trades.collect_assets();
        debug!("collected {} assets from trades", assets.len());
        let mut quotes = HashMap::with_capacity(assets.len());
        for asset in assets.into_iter() {
            quotes.insert(asset.clone(), Self::asset_price(&exchange, asset, to)?);
        }
        Ok(Self { quotes })
    }

    /// Get price for asset
    pub fn price(&self, asset: Asset) -> Option<Decimal> {
        self.quotes.get(&asset).cloned()
    }

    // -- loaders

    fn load_exchange(from: DateTime<Utc>, to: DateTime<Utc>) -> anyhow::Result<Exchange> {
        debug!("loading exchange in time range {} - {}", from, to);
        Exchange::new(from, to)
    }

    /// Get asset price
    fn asset_price(
        exchange: &Exchange,
        asset: Asset,
        price_at: DateTime<Utc>,
    ) -> anyhow::Result<Decimal> {
        debug!("looking up asset {:?}", asset);
        let symbol = Symbols::lookup(asset);
        debug!("got symbol {}", symbol);
        let quotation = exchange.get_symbol_quotes(&symbol)?;
        let price = quotation.price_at(price_at);
        debug!(
            "got quotation for {}; price at {}: {}",
            symbol, price_at, price
        );
        Ok(price)
    }
}
