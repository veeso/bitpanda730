//! # YahooFinanceClient
//!
//! the yahoo finance client provides functions to scrape the prices for symbols

use chrono::prelude::*;
use chrono::Utc;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use time::OffsetDateTime;
use yahoo_finance_api::YahooConnector;

use super::{Quote, Quotes};

const EUR_USD_SYMBOL: &str = "EURUSD=x";

pub struct YahooFinanceClient {
    eur_usd: Quotes,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl YahooFinanceClient {
    /// Create a new exchange instance. Working time range must be provided
    pub async fn new(from: DateTime<Utc>, to: DateTime<Utc>) -> anyhow::Result<Self> {
        Ok(Self {
            eur_usd: Self::fetch_symbol_history(EUR_USD_SYMBOL, from, to).await?,
            from,
            to,
        })
    }

    /// Get symbol quotes
    pub async fn get_symbol_quotes(&self, symbol: &str) -> anyhow::Result<Quotes> {
        let mut quotes = Self::fetch_symbol_history(symbol, self.from, self.to).await?;
        quotes.usd_to_eur(&self.eur_usd)?;
        Ok(quotes)
    }

    async fn fetch_symbol_history(
        symbol: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> anyhow::Result<Quotes> {
        debug!("getting history for symbol {}", symbol);
        let client = YahooConnector::new();
        let quotes = client
            .get_quote_history(
                symbol,
                Self::datetime_to_offset_date_time(from),
                Self::datetime_to_offset_date_time(to),
            )
            .await?;

        Ok(quotes
            .quotes()
            .unwrap()
            .into_iter()
            .filter(|x| {
                x.timestamp as i64 >= from.timestamp() && x.timestamp as i64 <= to.timestamp()
            })
            .map(|x| {
                Quote::usd(
                    Utc.timestamp_millis_opt((x.timestamp * 1000) as i64)
                        .unwrap(),
                    Decimal::from_f64(x.close).unwrap_or_default(),
                )
            })
            .collect::<Vec<Quote>>()
            .into())
    }

    fn datetime_to_offset_date_time(date: DateTime<Utc>) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(date.timestamp()).unwrap()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn should_init_exchange() {
        crate::mock::log();
        let from = Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
        let to = Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(2021, 12, 31)
                .unwrap()
                .and_hms_opt(23, 59, 59)
                .unwrap(),
        );
        let exchange = YahooFinanceClient::new(from, to).await.unwrap();
        let september23 = Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(2022, 9, 23)
                .unwrap()
                .and_hms_opt(23, 59, 59)
                .unwrap(),
        );
        assert_eq!(
            exchange.eur_usd.price_at(september23).round_dp(2),
            dec!(1.13)
        );
    }

    #[tokio::test]
    async fn should_fetch_symbol() {
        crate::mock::log();
        let from = Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
        let to = Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(2022, 12, 31)
                .unwrap()
                .and_hms_opt(23, 59, 59)
                .unwrap(),
        );
        let exchange = YahooFinanceClient::new(from, to).await.unwrap();
        let quotes = exchange.get_symbol_quotes("AMZN").await.unwrap();
        // check price for 23/09/2022
        let september23 = Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(2022, 9, 23)
                .unwrap()
                .and_hms_opt(23, 59, 59)
                .unwrap(),
        );
        // 113.78 $ =>
        assert_eq!(quotes.price_at(september23).round_dp(2), dec!(115.61));
    }
}
