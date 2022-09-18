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
        let mut trades = BitpandaTradeParser::parse(&args.csv_file)?;
        // filter
        todo!("filter dates");
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
