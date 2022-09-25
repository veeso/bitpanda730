//! # Exchange
//!
//! the exchange provides functions to scrape the prices for symbols

use chrono::prelude::*;
use chrono::Utc;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use yahoo_finance::history;

const EUR_USD_SYMBOL: &str = "EURUSD=x";

/// The collection of quotes ASC sorted by date
pub struct Quotes {
    quotes: Vec<Quote>,
}

/// A symbol quotation in on a specific date
pub struct Quote {
    pub date: DateTime<Utc>,
    /// EUR price
    pub price: Decimal,
}

pub struct Exchange {
    eur_usd: Quotes,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl Exchange {
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
            .map(|x| Quote {
                date: x.timestamp,
                price: Decimal::from_f64(x.close).unwrap_or_default(),
            })
            .collect::<Vec<Quote>>()
            .into())
    }
}

impl From<Vec<Quote>> for Quotes {
    fn from(mut quotes: Vec<Quote>) -> Self {
        // sort ASC by date
        quotes.sort_by_key(|x| x.date);
        Self { quotes }
    }
}

impl Quotes {
    /// Get valid price for date
    pub fn price_at(&self, date: DateTime<Utc>) -> Decimal {
        // get last
        self.quotes
            .iter()
            .filter(|x| x.date <= date)
            .last()
            .map(|x| x.price)
            .unwrap_or_default()
    }

    /// Convert quotes prices to EUR from USD
    fn usd_to_eur(&mut self, conversion: &Quotes) -> anyhow::Result<()> {
        debug!("converting quotes to EUR");
        for quote in self.quotes.iter_mut() {
            quote.usd_to_eur(conversion)?;
        }
        Ok(())
    }
}

impl Quote {
    /// Convert self price from USD to EUR
    fn usd_to_eur(&mut self, conversion: &Quotes) -> anyhow::Result<()> {
        let eur_change = match conversion.quotes.iter().find(|x| x.date == self.date) {
            Some(quote) => quote.price,
            None => anyhow::bail!(
                "could not find any USD-EUR conversion value for {}",
                self.date
            ),
        };
        // convert => self.price : eur_change = x : 1.0
        debug!(
            "applying USD to EUR conversion; 1$ = {}€ at {}",
            eur_change, self.date
        );
        self.price = self.price / eur_change;
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_init_exchange() {
        let from = Utc.from_utc_datetime(&NaiveDate::from_ymd(2021, 1, 1).and_hms(0, 0, 0));
        let to = Utc.from_utc_datetime(&NaiveDate::from_ymd(2021, 12, 31).and_hms(23, 59, 59));
        let exchange = Exchange::new(from, to).unwrap();
        assert_eq!(exchange.eur_usd.quotes.len(), 365);
    }

    #[test]
    fn should_fetch_symbol() {
        let from = Utc.from_utc_datetime(&NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0));
        let to = Utc.from_utc_datetime(&NaiveDate::from_ymd(2022, 12, 31).and_hms(23, 59, 59));
        let exchange = Exchange::new(from, to).unwrap();
        let quotes = exchange.get_symbol_quotes("AMZN").unwrap();
        // check price for 23/09/2022
    }
}
