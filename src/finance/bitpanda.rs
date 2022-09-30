//! # Bitpanda
//!
//! This module exposes the bitpanda api client

const API_BITPANDA_URL: &str = "https://api.bitpanda.com";

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use urlencoding;

use super::{Quote, Quotes};
use crate::bitpanda::trade::Asset;

mod types;
use types::{BitpandaPrice, BitpandaYear};

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
    pub fn get_symbols_quotes(&self, assets: &[Asset]) -> anyhow::Result<HashMap<Asset, Quotes>> {
        let symbols: Vec<String> = assets
            .iter()
            .map(|x| urlencoding::encode(&x.to_string()).to_string())
            .collect();
        let api_data = self.fetch_bitpanda_api(&symbols)?;
        let mut quotes = HashMap::with_capacity(symbols.len());
        // iter symbols to check whether all symbols are covered
        for asset in assets.iter() {
            let prices = match api_data.data.get(&asset.to_string()) {
                Some(prices) => prices,
                None => anyhow::bail!("{}: not found in bitpanda response", asset.to_string()),
            };
            quotes.insert(asset.clone(), self.convert_prices_to_quote(prices));
        }
        Ok(quotes)
    }

    /// fetch assets from bitpanda API
    fn fetch_bitpanda_api(&self, symbols: &[String]) -> anyhow::Result<BitpandaYear> {
        let query = symbols.join("%2C");
        let url = format!("{}/v2/ohlc/eur/year?assets={}", API_BITPANDA_URL, query);
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
        Quotes::from(
            prices
                .iter()
                .filter(|x| {
                    x.attributes.time.date_iso8601 >= self.from
                        && x.attributes.time.date_iso8601 <= self.to
                })
                .map(|x| {
                    Quote::eur(
                        DateTime::from(x.attributes.time.date_iso8601),
                        x.attributes.close,
                    )
                })
                .collect::<Vec<Quote>>(),
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::bitpanda::trade::{CryptoCurrency, Currency, Metal};

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_quotes_for_symbols() {
        let bitpanda = client();
        let assets = vec![
            Asset::Name(String::from("NASDAQ100")), // ETF
            Asset::Name(String::from("S&P500")),    // ETF with strange name
            Asset::Name(String::from("NATGAS")),    // Commodity
            Asset::Metal(Metal::Gold),              // Metal
            Asset::Name(String::from("AMZN")),      // Stock
            Asset::HongKong(1177),                  // Stock hong kong
            Asset::Currency(Currency::Crypto(CryptoCurrency::OneInch)), // Crypto
            Asset::Currency(Currency::Crypto(CryptoCurrency::Sushi)), // Crypto
        ];
        // fetch
        let quotes = bitpanda.get_symbols_quotes(&assets).unwrap();
        assert_eq!(quotes.len(), 8);
        assert!(quotes
            .get(&Asset::Name(String::from("NASDAQ100")))
            .is_some());
        assert!(quotes.get(&Asset::Name(String::from("S&P500"))).is_some());
        assert!(quotes.get(&Asset::Name(String::from("NATGAS"))).is_some());
        assert!(quotes.get(&Asset::Metal(Metal::Gold)).is_some());
        assert!(quotes.get(&Asset::Name(String::from("AMZN"))).is_some());
        assert!(quotes.get(&Asset::HongKong(1177)).is_some());
        assert!(quotes
            .get(&Asset::Currency(Currency::Crypto(CryptoCurrency::OneInch)))
            .is_some());
        assert!(quotes
            .get(&Asset::Currency(Currency::Crypto(CryptoCurrency::Sushi)))
            .is_some());
    }

    #[test]
    fn should_fail_get_quotes_for_unexisting_symbol() {
        let bitpanda = client();
        let assets = vec![Asset::Name(String::from("SOLARIUDINE"))];
        assert!(bitpanda.get_symbols_quotes(&assets).is_err());
    }

    fn client() -> BitpandaClient {
        use chrono::prelude::*;
        use chrono::NaiveDate;
        BitpandaClient::new(
            Utc.from_utc_datetime(&NaiveDate::from_ymd(2021, 1, 1).and_hms(12, 0, 0)),
            Utc.from_utc_datetime(&NaiveDate::from_ymd(2050, 12, 31).and_hms(23, 59, 59)),
        )
    }
}
