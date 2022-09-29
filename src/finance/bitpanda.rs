//! # Bitpanda
//!
//! This module exposes the bitpanda api client

const API_BITPANDA_URL: &str = "https://api.bitpanda.com";

use chrono::{DateTime, Utc};
use std::collections::HashMap;

use self::types::BitpandaPrice;

use super::{Quote, Quotes};

mod types;
use types::BitpandaYear;

/// The Bitpanda client is used to fetch prices for provided assets
/// NOTE: should only be used for ETFs/Commodities/Metals, while for other assets, please refer to Yahoo
pub struct BitpandaClient {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl BitpandaClient {
    pub fn new(from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        Self { from, to }
    }

    /// Get symbols quotes
    pub fn get_symbols_quotes(&self, symbol: &str) -> anyhow::Result<Quotes> {
        let api_data = self.fetch_bitpanda_api(symbol)?;
        let prices = match api_data.data.get(symbol) {
            Some(prices) => prices,
            None => anyhow::bail!("{}: not found in bitpanda response", symbol),
        };
        Ok(self.convert_prices_to_quote(prices))
    }

    /// fetch assets from bitpanda API
    fn fetch_bitpanda_api(&self, symbol: &str) -> anyhow::Result<BitpandaYear> {
        let url = format!("{}/v2/ohlc/eur/year?assets={}", API_BITPANDA_URL, symbol);
        debug!("getting data for assets at {}", url);
        let response = ureq::get(&url)
            .call()
            .map_err(|e| anyhow::anyhow!("failed to get assets: {}", e))?;
        response
            .into_json()
            .map_err(|e| anyhow::anyhow!("failed to decode response body: {}", e))
    }

    /// Convert bitpanda prices to Quotes
    fn convert_prices_to_quote(&self, prices: &[BitpandaPrice]) -> Quotes {
        todo!("convert")
    }
}
