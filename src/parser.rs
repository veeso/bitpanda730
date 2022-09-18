//! # Parser
//!
//! Bitpanda CSV parser

use crate::bitpanda::Trade;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct BitpandaTradeParser;

impl BitpandaTradeParser {
    pub fn parse(path: &Path) -> anyhow::Result<Vec<Trade>> {
        debug!("opening file {}", path.display());
        let mut file = File::open(path)?;
        debug!("file opened");
        todo!("skip rows");
        debug!("parsing CSV data");
        let mut reader = csv::Reader::from_reader(file);
        let mut trades: Vec<Trade> = Vec::new();
        for result in reader.deserialize::<Trade>() {
            trades.push(result?);
        }
        Ok(trades)
    }
}
