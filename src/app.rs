//! # App
//!
//! This module exposes the main application workflow

use crate::{args::Args, bitpanda::Trade, database::TradeDatabase, parser::BitpandaTradeParser};

use std::convert::TryFrom;

/// Application container
pub struct App {
    trades: TradeDatabase,
}

impl TryFrom<Args> for App {
    type Error = anyhow::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        // open file
        info!("parsing CSV file {}", args.csv_file.display());
        let trades = BitpandaTradeParser::parse(&args.csv_file)?;
        // filter by date
        let trades: Vec<Trade> = trades
            .into_iter()
            .filter(|trade| args.since <= trade.timestamp() && args.to >= trade.timestamp())
            .collect();
        info!("working on a total amount of {} trades", trades.len());
        Ok(App {
            trades: TradeDatabase::from(trades),
        })
    }
}

impl App {
    /// Run application
    pub fn run(mut self) -> anyhow::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use chrono::DateTime;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn should_init_app_from_args() {
        let args = Args {
            since: DateTime::from_str("2022-08-01T00:00:00Z").unwrap(),
            to: DateTime::from_str("2022-08-15T00:00:00Z").unwrap(),
            debug: false,
            verbose: false,
            csv_file: PathBuf::from("./test/bitpanda.csv"),
            version: false,
        };
        let app = App::try_from(args).unwrap();
        assert_eq!(app.trades.len(), 4);
    }
}
