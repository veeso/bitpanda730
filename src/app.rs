//! # App
//!
//! This module exposes the main application workflow

use crate::{
    args::Args,
    bitpanda::{trade::Fiat, Trade},
    database::{QuoteDatabase, TradeDatabase},
    parser::BitpandaTradeParser,
    tax::{GainsAndLosses, Taxes},
};

use chrono::prelude::*;
use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;
use spinners::{Spinner, Spinners};
use std::convert::TryFrom;

/// Application container
pub struct App {
    trades: TradeDatabase,
    since: DateTime<FixedOffset>,
    to: DateTime<FixedOffset>,
}

impl TryFrom<Args> for App {
    type Error = anyhow::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        // open file
        info!("parsing CSV file {}", args.csv_file.display());
        let trades = BitpandaTradeParser::parse(&args.csv_file)?;
        // calc date range according to Italian timezone
        let since = FixedOffset::east(3600)
            .ymd(args.year, 1, 1)
            .and_hms(0, 0, 0);
        let to = FixedOffset::east(3600)
            .ymd(args.year, 12, 31)
            .and_hms(23, 59, 59);
        info!("working on time range {} => {}", since, to);
        // filter by date
        let trades: Vec<Trade> = trades
            .into_iter()
            .filter(|trade| since <= trade.timestamp() && to >= trade.timestamp())
            .collect();
        info!("working on a total amount of {} trades", trades.len());
        let trades = TradeDatabase::from(trades);
        Ok(App { trades, since, to })
    }
}

impl App {
    /// Run application
    pub fn run(self) -> anyhow::Result<()> {
        let quotes = self.load_quotes_database()?;
        debug!("quotes loaded");
        info!(
            "current FIAT balance: € {}",
            self.trades.all().fiat_balance(Fiat::Eur)
        );
        debug!("taxes setup");
        let taxes = Taxes::new(&self.trades, &quotes, self.since, self.to);
        let ivafe = self.calc_ivafe(&taxes)?;
        info!("IVAFE is: € {}", ivafe);
        let capitals_diff = self.calc_gains_and_losses(&taxes)?;
        info!(
            "gains: € {}; losses: € {}; diff: € {}; total taxes to pay: € {}",
            capitals_diff.gains_value(),
            capitals_diff.losses_value(),
            capitals_diff.gains_value() - capitals_diff.losses_value(),
            capitals_diff.tax_to_pay()
        );
        todo!("repr output")
    }

    /// Load quotes database from trades
    fn load_quotes_database(&self) -> anyhow::Result<QuoteDatabase> {
        debug!("loading quotes from {} to {}...", self.since, self.to);
        let mut sp = Spinner::new(Spinners::Dots, "loading asset prices...".to_string());
        let quotes = QuoteDatabase::load(&self.trades, self.since, self.to)?;
        sp.stop();
        Ok(quotes)
    }

    fn calc_ivafe(&self, taxes: &Taxes) -> anyhow::Result<Decimal> {
        debug!("calculating IVAFE");
        let mut sp = Spinner::new(Spinners::Dots, "Calculating IVAFE...".to_string());
        let ivafe = taxes.ivafe();
        sp.stop();
        ivafe
    }

    fn calc_gains_and_losses(&self, taxes: &Taxes) -> anyhow::Result<GainsAndLosses> {
        debug!("calculating gains and losses");
        let mut sp = Spinner::new(
            Spinners::Dots,
            "Calculating capital gains and losses...".to_string(),
        );
        let capital_diff = taxes.capital_gains_and_losses();
        sp.stop();
        capital_diff
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn should_init_app_from_args() {
        let args = Args {
            year: 2022,
            debug: false,
            verbose: false,
            csv_file: PathBuf::from("./test/bitpanda.csv"),
            version: false,
        };
        let app = App::try_from(args).unwrap();
        assert_eq!(app.trades.all().trades().len(), 12);
    }
}
