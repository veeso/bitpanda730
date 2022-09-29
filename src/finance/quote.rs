//! # Quote
//!
//! Quote type

use chrono::prelude::*;
use chrono::Utc;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Currency {
    Usd,
    Eur,
}

/// The collection of quotes ASC sorted by date
pub struct Quotes {
    quotes: Vec<Quote>,
}

/// A symbol quotation in on a specific date
pub struct Quote {
    pub date: DateTime<Utc>,
    /// EUR price
    pub price: Decimal,
    /// Tracks current price currency
    currency: Currency,
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
            .unwrap_or_else(|| {
                self.quotes
                    .iter()
                    .map(|x| x.price)
                    .next()
                    .expect("quotes are empty")
            })
    }

    /// Convert quotes prices to EUR from USD
    pub fn usd_to_eur(&mut self, conversion: &Quotes) -> anyhow::Result<()> {
        debug!("converting quotes to EUR");
        for quote in self.quotes.iter_mut() {
            quote.usd_to_eur(conversion)?;
        }
        Ok(())
    }
}

impl Quote {
    /// Create a new Quote with USD price
    pub fn usd(date: DateTime<Utc>, price: Decimal) -> Self {
        Self::new(date, price, Currency::Usd)
    }

    /// Create a new quote with EUR price
    pub fn eur(date: DateTime<Utc>, price: Decimal) -> Self {
        Self::new(date, price, Currency::Eur)
    }

    fn new(date: DateTime<Utc>, price: Decimal, currency: Currency) -> Self {
        Self {
            date,
            price,
            currency,
        }
    }

    /// Convert self price from USD to EUR
    fn usd_to_eur(&mut self, conversion: &Quotes) -> anyhow::Result<()> {
        if self.currency != Currency::Usd {
            anyhow::bail!("current currency is not USD, but {:?}", self.currency);
        }
        let eur_change = conversion.price_at(self.date);
        // convert => self.price : eur_change = x : 1.0
        debug!(
            "applying USD to EUR conversion; 1$ = {}€ at {}",
            eur_change, self.date
        );
        self.price /= eur_change;
        // set currency to eur
        self.currency = Currency::Eur;
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_make_quote() {
        let date = Utc.from_utc_datetime(&NaiveDate::from_ymd(2022, 9, 23).and_hms(12, 0, 0));
        let quote = Quote::usd(date, dec!(120.32));
        assert_eq!(quote.date, date);
        assert_eq!(quote.price, dec!(120.32));
        assert_eq!(quote.currency, Currency::Usd);
    }

    #[test]
    fn should_convert_price_to_eur() {
        let date = Utc.from_utc_datetime(&NaiveDate::from_ymd(2022, 9, 23).and_hms(12, 0, 0));
        let mut quote = Quote::usd(date, dec!(120.32));
        assert!(quote.usd_to_eur(&usd_to_eur_table()).is_ok());
        assert_eq!(quote.price.round_dp(2), dec!(122.78));
        assert_eq!(quote.currency, Currency::Eur);
    }

    #[test]
    fn should_not_convert_price_to_eur_twice() {
        let date = Utc.from_utc_datetime(&NaiveDate::from_ymd(2022, 9, 23).and_hms(12, 0, 0));
        let mut quote = Quote::usd(date, dec!(120.32));
        assert!(quote.usd_to_eur(&usd_to_eur_table()).is_ok());
        // error on second conversion
        assert!(quote.usd_to_eur(&usd_to_eur_table()).is_err());
        assert_eq!(quote.price.round_dp(2), dec!(122.78));
        assert_eq!(quote.currency, Currency::Eur);
    }

    fn usd_to_eur_table() -> Quotes {
        Quotes::from(vec![
            Quote::usd(quote_date(2022, 1, 1), dec!(1.14)),
            Quote::usd(quote_date(2022, 9, 1), dec!(1.00)),
            Quote::usd(quote_date(2022, 9, 22), dec!(0.98)),
            Quote::usd(quote_date(2022, 9, 26), dec!(0.97)),
        ])
    }

    fn quote_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.from_utc_datetime(&NaiveDate::from_ymd(year, month, day).and_hms(12, 0, 0))
    }
}
