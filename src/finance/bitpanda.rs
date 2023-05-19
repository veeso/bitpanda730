//! # Bitpanda
//!
//! This module exposes the bitpanda api client

use bitpanda_api::model::ohlc::Period;
use bitpanda_api::model::{Asset, AssetClass};
use bitpanda_api::Client;
use bitpanda_csv::Asset as CsvAsset;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::{Quote, Quotes};

const CURRENCY: &str = "EUR";

/// The Bitpanda client is used to fetch prices for provided assets
/// NOTE: should only be used for ETFs/Commodities/Metals, while for other assets, please refer to Yahoo
pub struct BitpandaClient {
    assets: Vec<Asset>,
    client: Client,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl BitpandaClient {
    pub async fn init(from: DateTime<Utc>, to: DateTime<Utc>) -> anyhow::Result<Self> {
        let assets = Self::get_assets().await?;

        Ok(Self {
            assets,
            client: Client::default(),
            from,
            to,
        })
    }

    /// Get symbols quotes
    pub async fn get_symbols_quotes(
        &self,
        assets: &[CsvAsset],
    ) -> anyhow::Result<HashMap<CsvAsset, Quotes>> {
        let mut quotes = HashMap::with_capacity(assets.len());
        for asset in assets {
            debug!("getting PID for {asset}");
            let pid = &self.select_asset_from_db(&asset.to_string())?.pid;
            debug!("querying OHLC for {pid}");
            let ohlc = self.client.get_ohlc(Period::Year, pid, CURRENCY).await?;
            quotes.insert(
                asset.clone(),
                Quotes::from(
                    ohlc.chart
                        .into_iter()
                        .filter(|entry| entry.time >= self.from && entry.time < self.to)
                        .map(Quote::from)
                        .collect::<Vec<Quote>>(),
                ),
            );
        }

        Ok(quotes)
    }

    /// get all assets from bitpanda
    async fn get_assets() -> anyhow::Result<Vec<Asset>> {
        let client = Client::default();
        debug!("fetching all assets from bitpanda...");
        let mut assets = Vec::new();
        debug!("fetching commodities...");
        assets.extend(client.get_assets(AssetClass::Commodity).await?);
        debug!("fetching cryptos...");
        assets.extend(client.get_assets(AssetClass::Cryptocurrency).await?);
        debug!("fetching etfs...");
        assets.extend(client.get_assets(AssetClass::Etf).await?);
        debug!("fetching metals...");
        assets.extend(client.get_assets(AssetClass::Metal).await?);
        debug!("fetching stocks...");
        assets.extend(client.get_assets(AssetClass::Stock).await?);
        debug!("found a total amount of {} assets", assets.len());

        Ok(assets)
    }

    /// select asset from database. If not found return error
    fn select_asset_from_db(&self, symbol: &String) -> anyhow::Result<&Asset> {
        match self.assets.iter().find(|asset| &asset.symbol == symbol) {
            Some(asset) => Ok(asset),
            None => anyhow::bail!("asset {symbol} not found"),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use bitpanda_csv::{CryptoCurrency, Currency, Metal};

    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn should_get_quotes_for_symbols() {
        let bitpanda = client().await;
        let assets = vec![
            CsvAsset::Ticker(String::from("NASDAQ100")), // ETF
            CsvAsset::Ticker(String::from("S&P500")),    // ETF with strange name
            CsvAsset::Ticker(String::from("NATGAS")),    // Commodity
            CsvAsset::Metal(Metal::Gold),                // Metal
            CsvAsset::Ticker(String::from("AMZN")),      // Stock
            CsvAsset::HongKong(1177),                    // Stock hong kong
            CsvAsset::Currency(Currency::Crypto(CryptoCurrency::OneInch)), // Crypto
            CsvAsset::Currency(Currency::Crypto(CryptoCurrency::Sushi)), // Crypto
        ];
        // fetch
        let quotes = bitpanda.get_symbols_quotes(&assets).await.unwrap();
        assert_eq!(quotes.len(), 8);
        assert!(quotes
            .get(&CsvAsset::Ticker(String::from("NASDAQ100")))
            .is_some());
        assert!(quotes
            .get(&CsvAsset::Ticker(String::from("S&P500")))
            .is_some());
        assert!(quotes
            .get(&CsvAsset::Ticker(String::from("NATGAS")))
            .is_some());
        assert!(quotes.get(&CsvAsset::Metal(Metal::Gold)).is_some());
        assert!(quotes
            .get(&CsvAsset::Ticker(String::from("AMZN")))
            .is_some());
        assert!(quotes.get(&CsvAsset::HongKong(1177)).is_some());
        assert!(quotes
            .get(&CsvAsset::Currency(Currency::Crypto(
                CryptoCurrency::OneInch
            )))
            .is_some());
        assert!(quotes
            .get(&CsvAsset::Currency(Currency::Crypto(CryptoCurrency::Sushi)))
            .is_some());
    }

    #[tokio::test]
    async fn should_fail_get_quotes_for_unexisting_symbol() {
        let bitpanda = client().await;
        let assets = vec![CsvAsset::Ticker(String::from("SOLARIUDINE"))];
        assert!(bitpanda.get_symbols_quotes(&assets).await.is_err());
    }

    async fn client() -> BitpandaClient {
        use chrono::prelude::*;
        use chrono::NaiveDate;
        BitpandaClient::init(
            Utc.from_utc_datetime(
                &NaiveDate::from_ymd_opt(2021, 1, 1)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
            ),
            Utc.from_utc_datetime(
                &NaiveDate::from_ymd_opt(2050, 12, 31)
                    .unwrap()
                    .and_hms_opt(23, 59, 59)
                    .unwrap(),
            ),
        )
        .await
        .unwrap()
    }
}
