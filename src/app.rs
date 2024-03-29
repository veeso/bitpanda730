//! # App
//!
//! This module exposes the main application workflow

use crate::{
    database::{QuoteDatabase, TradeDatabase},
    module730::{Module730, Stdout as StdoutPaginate},
    tax::{GainsAndLosses, Taxes},
};

use bitpanda_csv::{AsyncBitpandaTradeParser, Fiat, Trade};
use chrono::prelude::*;
use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;
use spinners::{Spinner, Spinners};
use std::path::Path;
use tokio::fs::File;
use tokio::io::BufReader;

/// Application container
pub struct App {
    trades: TradeDatabase,
    since: DateTime<FixedOffset>,
    to: DateTime<FixedOffset>,
}

impl App {
    /// Setup a new application
    pub async fn setup(year: i32, csv_file: &Path) -> anyhow::Result<Self> {
        // open file
        info!("parsing CSV file {}", csv_file.display());
        let csv_file = File::open(csv_file).await?;
        let reader = BufReader::new(csv_file);
        let trades = AsyncBitpandaTradeParser::parse(reader).await?;
        // calc date range according to Italian timezone
        let since = FixedOffset::east_opt(3600)
            .unwrap()
            .with_ymd_and_hms(year, 1, 1, 0, 0, 0)
            .unwrap();
        let to = FixedOffset::east_opt(3600)
            .unwrap()
            .with_ymd_and_hms(year, 12, 31, 23, 59, 59)
            .unwrap();
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

    /// Run application
    pub async fn run(self) -> anyhow::Result<()> {
        let quotes = self.load_quotes_database().await?;
        debug!("quotes loaded");
        info!(
            "current FIAT balance: € {}",
            self.trades.all().fiat_balance(Fiat::Eur)
        );
        debug!("taxes setup");
        let taxes = Taxes::new(&self.trades, &quotes, self.since, self.to);
        let average_balance = self.calc_average_balance(&taxes)?;
        info!("Average balance is: € {}", average_balance);
        let ivafe = self.calc_ivafe(&taxes, average_balance);
        info!("IVAFE is: € {}", ivafe);
        let capitals_diff = self.calc_gains_and_losses(&taxes)?;
        info!(
            "gains: € {}; losses: € {}; diff: € {}; total taxes to pay: € {}",
            capitals_diff.gains_value(),
            capitals_diff.losses_value(),
            capitals_diff.gains_value() + capitals_diff.losses_value(),
            capitals_diff.tax_to_pay()
        );
        // repr output
        debug!("preparing 730...");
        let m730 = Module730::prepare(average_balance, ivafe, &capitals_diff)?;
        debug!("730 ready; writing data to output...");
        m730.output(StdoutPaginate::default(), &capitals_diff)?;

        Ok(())
    }

    /// Load quotes database from trades
    async fn load_quotes_database(&self) -> anyhow::Result<QuoteDatabase> {
        debug!("loading quotes from {} to {}...", self.since, self.to);
        let mut sp = Spinner::new(Spinners::Dots, "loading asset prices...".to_string());
        let quotes = QuoteDatabase::load(&self.trades, self.since, self.to).await?;
        sp.stop();
        Ok(quotes)
    }

    fn calc_average_balance(&self, taxes: &Taxes) -> anyhow::Result<Decimal> {
        debug!("calculating IVAFE");
        let mut sp = Spinner::new(Spinners::Dots, "Calculating IVAFE...".to_string());
        let avg_balance = taxes.average_balance();
        sp.stop();
        avg_balance
    }

    fn calc_ivafe(&self, taxes: &Taxes, avg_balance: Decimal) -> Decimal {
        debug!("calculating IVAFE");
        let mut sp = Spinner::new(Spinners::Dots, "Calculating IVAFE...".to_string());
        let ivafe = taxes.ivafe(avg_balance);
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

    #[tokio::test]
    async fn should_init_app_from_args() {
        crate::mock::log();
        let app = App::setup(2022, Path::new("./test/bitpanda.csv"))
            .await
            .unwrap();
        assert_eq!(app.trades.all().trades().len(), 12);
    }
}
