//! # Tax
//!
//! This module expose the tax calculators for Italian taxation ruleset

mod gains_and_losses;
pub use gains_and_losses::{Calculator as GainsAndLossesCalculator, CapitalDiff, GainsAndLosses};

use crate::database::{QuoteDatabase, TradeDatabase, TradeQuery, WalletDatabase};
use bitpanda_csv::{Asset, Currency, Fiat};

use chrono::{DateTime, Datelike, FixedOffset, LocalResult, TimeZone};
use rust_decimal::Decimal;

/// Italian fiscal taxes calculator
///
/// References:
///
/// - <https://taxfix.it/guide-e-consigli/guide-al-730/tasse/tasse-trading-online/>
/// - <https://www.youtube.com/watch?v=APWKZgimiR8>
///
pub struct Taxes<'a> {
    trades: &'a TradeDatabase,
    quotes: &'a QuoteDatabase,
    since: DateTime<FixedOffset>,
    to: DateTime<FixedOffset>,
}

impl<'a> Taxes<'a> {
    pub fn new(
        trades: &'a TradeDatabase,
        quotes: &'a QuoteDatabase,

        since: DateTime<FixedOffset>,
        to: DateTime<FixedOffset>,
    ) -> Self {
        Self {
            trades,
            quotes,
            since,
            to,
        }
    }

    /// Calculate the tax on the foreign bank account (Bitpanda is located in Austria)
    ///
    /// > Le persone fisiche residenti in Italia che hanno prodotti finanziari,
    /// > libretti di risparmio o conti correnti presso intermediari esteri,
    /// > Sono tenuti a versare anche l’IVAFE, ossia l’Imposta sul Valore delle Attività Finanziarie all’Estero.
    /// > Tale imposta è applicata in modo
    /// > proporzionale al 2 per mille annuo del valore delle attività finanziarie.
    pub fn ivafe(&self, average_balance: Decimal) -> Decimal {
        debug!("average balance for this year is {}", average_balance);
        if average_balance < dec!(5000.0) {
            info!("average balance is under € 5000, so IVAFE is not required");
            Decimal::ZERO
        } else {
            // avg_balance : 100 = ivafe : 0.2
            let ivafe = average_balance * dec!(0.002);
            info!("IVAFE: {}", ivafe);
            ivafe.round_dp(2)
        }
    }

    /// Calculate the capital gains and losses. Taxes are already calculated.
    ///
    /// > plusvalenze: reddito dovuto alla vendita a un prezzo superiore di quello di acquisto, ossia un guadagno
    /// > minusvalenze: controvalore derivante dalla vendita di uno strumento finanziario a un prezzo inferiore rispetto a quello d’acquisto, ossia una perdita
    pub fn capital_gains_and_losses(&self) -> anyhow::Result<GainsAndLosses> {
        let mut calculator = GainsAndLossesCalculator::default();
        calculator.calculate(self.trades)
    }

    /// Calculate the average balance along the year
    /// From Agenzia delle entrate: (<https://www.agenziaentrate.gov.it/portale/web/guest/schede/comunicazioni/integrativa-archivio-dei-rapporti-con-operatori-finanziari/giacenza-media-annua#:~:text=Il%20calcolo%20della%20giacenza%20media,il%20deposito%2Fconto%20risulta%20attivo.>)
    ///
    /// > Per giacenza media annua si intende l’importo medio delle somme
    /// > a credito del cliente in un dato periodo ragguagliato ad un anno.
    /// > Il calcolo della giacenza media annua si determina dividendo la somma delle giacenze giornaliere per 365,
    /// > indipendentemente dal numero di giorni in cui il deposito/conto risulta attivo.
    /// > Per giacenze giornaliere si intendono i saldi giornalieri per valuta.
    pub fn average_balance(&self) -> anyhow::Result<Decimal> {
        let mut date = match (*self.since.offset()).with_ymd_and_hms(
            self.since.year(),
            self.since.month(),
            self.since.day(),
            23,
            59,
            59,
        ) {
            LocalResult::Single(date) => date,
            _ => anyhow::bail!("invalid date"),
        };
        let mut total_balance = Decimal::ZERO;
        // Iterate over the days in the time range
        while date <= self.to {
            let fiat_balance = self
                .trades
                .select(TradeQuery::default().before(date))
                .fiat_balance(Fiat::Eur);
            info!(
                "FIAT balance at {} ({}): {}",
                date,
                date.ordinal(),
                fiat_balance
            );
            total_balance += fiat_balance;
            // calculate balance at date for each asset; get wallet at date first
            let trades_wno_eur = self.trades.select(
                TradeQuery::default().asset_neq(Asset::Currency(Currency::Fiat(Fiat::Eur))),
            );
            let wallet = WalletDatabase::load(&trades_wno_eur);
            let wallet_balance = self.wallet_balance(wallet)?;
            info!(
                "wallet balance at {} ({}): {}",
                date,
                date.ordinal(),
                wallet_balance
            );
            total_balance += wallet_balance;
            // incr total balance, days and date
            date += chrono::Duration::days(1);
        }
        Ok(total_balance / Decimal::from(self.to.ordinal()))
    }

    /// Get wallet balance from wallet
    fn wallet_balance(&self, wallet: WalletDatabase) -> anyhow::Result<Decimal> {
        let mut wallet_balance = Decimal::ZERO;
        for (asset, quantity) in wallet.iter() {
            let asset_price = match self.quotes.price(asset) {
                Some(price) => price,
                None => anyhow::bail!("could not find any price for asset {}", asset),
            };
            let asset_balance = *quantity * asset_price;
            debug!("asset balance for {}: € {}", asset, asset_balance);
            wallet_balance += asset_balance;
        }
        Ok(wallet_balance)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::mock::database::{DatabaseQuoteMock, DatabaseTradeMock};

    #[tokio::test]
    async fn should_init_taxes() {
        crate::mock::log();
        let trades = DatabaseTradeMock::mock();
        let quotes = DatabaseQuoteMock::mock().await;
        let _ = mocked(&trades, &quotes);
    }

    #[tokio::test]
    async fn should_calc_ivafe() {
        crate::mock::log();
        let trades = DatabaseTradeMock::mock();
        let quotes = DatabaseQuoteMock::mock().await;
        let tax = mocked(&trades, &quotes);
        let avg_balance = tax.average_balance().unwrap();
        assert_eq!(tax.ivafe(avg_balance), dec!(20.16));
    }

    #[tokio::test]
    async fn should_return_ivafe_0_if_below_5000() {
        crate::mock::log();
        let trades = TradeDatabase::from(vec![]);
        let quotes = DatabaseQuoteMock::mock().await;
        let tax = mocked(&trades, &quotes);
        let avg_balance = tax.average_balance().unwrap();
        assert_eq!(tax.ivafe(avg_balance), Decimal::ZERO);
    }

    #[tokio::test]
    async fn should_calc_average_balance() {
        crate::mock::log();
        let trades = DatabaseTradeMock::mock();
        let quotes = DatabaseQuoteMock::mock().await;
        let tax = mocked(&trades, &quotes);
        assert_eq!(tax.average_balance().unwrap().round_dp(2), dec!(10077.96));
    }

    fn mocked<'a>(trades: &'a TradeDatabase, quotes: &'a QuoteDatabase) -> Taxes<'a> {
        let since = FixedOffset::east_opt(3600)
            .unwrap()
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .unwrap();
        let to = FixedOffset::east_opt(3600)
            .unwrap()
            .with_ymd_and_hms(2022, 12, 31, 23, 59, 59)
            .unwrap();
        Taxes::new(trades, quotes, since, to)
    }
}
