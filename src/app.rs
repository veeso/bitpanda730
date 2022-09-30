//! # App
//!
//! This module exposes the main application workflow

use crate::{
    args::Args,
    bitpanda::Trade,
    database::{QuoteDatabase, TradeDatabase, WalletDatabase},
    parser::BitpandaTradeParser,
    tax::Taxes,
};

use chrono::prelude::*;
use chrono::{DateTime, FixedOffset};
use spinners::{Spinner, Spinners};
use std::convert::TryFrom;

/// Application container
pub struct App {
    trades: TradeDatabase,
    wallet: WalletDatabase,
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
        Ok(App {
            wallet: WalletDatabase::load(trades.all()),
            trades,
            since,
            to,
        })
    }
}

impl App {
    /// Run application
    pub fn run(mut self) -> anyhow::Result<()> {
        let quotes = self.load_quotes_database()?;
        debug!("quotes loaded");
        info!("current FIAT balance: {}", self.trades.all().fiat_balance());
        debug!("taxes setup");
        let taxes = Taxes::new(&self.trades, &quotes, &self.wallet, self.since, self.to);
        debug!("calculating IVAFE");
        let ivafe = taxes.ivafe();
        info!("IVAFE is: {}", ivafe);
        todo!()
    }

    /// Load quotes database from trades
    fn load_quotes_database(&self) -> anyhow::Result<QuoteDatabase> {
        debug!("loading quotes from {} to {}...", self.since, self.to);
        let mut sp = Spinner::new(Spinners::Dots, "loading asset prices...".to_string());
        let quotes = QuoteDatabase::load(&self.trades, self.since, self.to)?;
        sp.stop();
        Ok(quotes)
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
