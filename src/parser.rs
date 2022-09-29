//! # Parser
//!
//! Bitpanda CSV parser

use crate::bitpanda::Trade;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use tempfile::NamedTempFile;

const BITPANDA_CSV_COL_HEADER: &str = r#""Transaction ID",Timestamp,"Transaction Type",In/Out,"Amount Fiat",Fiat,"Amount Asset",Asset,"Asset market price","Asset market price currency","Asset class","Product ID",Fee,"Fee asset",Spread,"Spread Currency""#;

pub struct BitpandaTradeParser;

impl BitpandaTradeParser {
    /// Parse CSV from file at path after sanitizing it
    pub fn parse(path: &Path) -> anyhow::Result<Vec<Trade>> {
        debug!("parsing file {}", path.display());
        let sanitized_csv = Self::sanitize_csv(path)?;
        debug!("parsing CSV data from {}", sanitized_csv.path().display());
        let file = File::open(sanitized_csv.path())?;
        debug!("tempfile opened");
        let mut reader = csv::Reader::from_reader(file);
        let mut trades: Vec<Trade> = Vec::new();
        for trade in reader.deserialize::<Trade>() {
            let trade = trade?;
            debug!("found trade {:?}", trade);
            trades.push(trade);
        }
        info!("found {} trades in CSV file", trades.len());
        Ok(trades)
    }

    /// Sanitize the trades csv keeping only the lines after the columns headers
    fn sanitize_csv(path: &Path) -> anyhow::Result<NamedTempFile> {
        let reader = File::open(path)?;
        debug!("file opened");
        // open tempfile
        debug!("opening tempfile");
        let work_file = NamedTempFile::new()?;
        let mut writer = File::create(work_file.path())?;
        debug!("tempfile opened at {}", work_file.path().display());
        // iter reader lines
        let mut keep = false;
        for line in BufReader::new(reader).lines() {
            let line = line?;
            if line == BITPANDA_CSV_COL_HEADER {
                debug!("found column header");
                keep = true;
            }
            if keep {
                writeln!(writer, "{}", line)?;
            }
        }
        debug!("csv file sanitized");
        Ok(work_file)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_csv_data() {
        let trades = BitpandaTradeParser::parse(Path::new("./test/bitpanda.csv")).unwrap();
        assert_eq!(trades.len(), 12);
    }
}
