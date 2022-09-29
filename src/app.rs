//! # App
//!
//! This module exposes the main application workflow

use crate::{
    args::Args, bitpanda::Trade, database::TradeDatabase, parser::BitpandaTradeParser, tax::Taxes,
};

use chrono::prelude::*;
use chrono::{DateTime, FixedOffset};
use std::convert::TryFrom;

/// Application container
pub struct App {
    trades: TradeDatabase,
    since: DateTime<FixedOffset>,
    to: DateTime<FixedOffset>,
    year: i32,
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
        Ok(App {
            trades: TradeDatabase::from(trades),
            since,
            to,
            year: args.year,
        })
    }
}

impl App {
    /// Run application
    pub fn run(mut self) -> anyhow::Result<()> {
        info!("current FIAT balance: {}", self.trades.fiat_balance());
        debug!("taxes setup");
        let taxes = Taxes::new(&self.trades, self.since, self.to);
        debug!("calculating IVAFE");
        let ivafe = taxes.ivafe();
        todo!()
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
        assert_eq!(app.trades.trades().len(), 12);
    }
}
