//! # YahooFinanceClient
//!
//! the yahoo finance client provides functions to scrape the prices for symbols

use chrono::prelude::*;
use chrono::Utc;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use yahoo_finance::history;

use super::{Quote, Quotes};

const EUR_USD_SYMBOL: &str = "EURUSD=x";

pub struct YahooFinanceClient {
    eur_usd: Quotes,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl YahooFinanceClient {
    /// Create a new exchange instance. Working time range must be provided
    pub fn new(from: DateTime<Utc>, to: DateTime<Utc>) -> anyhow::Result<Self> {
        Ok(Self {
            eur_usd: Self::fetch_symbol_history(EUR_USD_SYMBOL, from, to)?,
            from,
            to,
        })
    }

    /// Get symbol quotes
    pub fn get_symbol_quotes(&self, symbol: &str) -> anyhow::Result<Quotes> {
        let mut quotes = Self::fetch_symbol_history(symbol, self.from, self.to)?;
        quotes.usd_to_eur(&self.eur_usd)?;
        Ok(quotes)
    }

    fn fetch_symbol_history(
        symbol: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> anyhow::Result<Quotes> {
        debug!("getting history for symbol {}", symbol);
        let data = history::retrieve_range(symbol, from, Some(to))?;

        Ok(data
            .into_iter()
            .filter(|x| x.timestamp >= from && x.timestamp <= to)
            .map(|x| Quote::usd(x.timestamp, Decimal::from_f64(x.close).unwrap_or_default()))
            .collect::<Vec<Quote>>()
            .into())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_init_exchange() {
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
        let exchange = YahooFinanceClient::new(from, to).unwrap();
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

    #[test]
    fn should_fetch_symbol() {
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
        let exchange = YahooFinanceClient::new(from, to).unwrap();
        let quotes = exchange.get_symbol_quotes("AMZN").unwrap();
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
