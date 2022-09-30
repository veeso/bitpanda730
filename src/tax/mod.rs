//! # Tax
//!
//! This module expose the tax calculators for Italian taxation ruleset

use crate::bitpanda::trade::Fiat;
use crate::database::{QuoteDatabase, TradeDatabase, TradeQuery, WalletDatabase};

use chrono::{DateTime, Datelike, FixedOffset};
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
    wallet: &'a WalletDatabase,
    since: DateTime<FixedOffset>,
    to: DateTime<FixedOffset>,
}

impl<'a> Taxes<'a> {
    pub fn new(
        trades: &'a TradeDatabase,
        quotes: &'a QuoteDatabase,
        wallet: &'a WalletDatabase,
        since: DateTime<FixedOffset>,
        to: DateTime<FixedOffset>,
    ) -> Self {
        Self {
            trades,
            quotes,
            wallet,
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
    pub fn ivafe(&self) -> Decimal {
        let avg_balance = self.average_balance();
        debug!("average balance for this year is {}", avg_balance);
        if avg_balance < dec!(5000.0) {
            info!("average balance is under 5000€, so IVAFE is not required");
            Decimal::ZERO
        } else {
            // avg_balance : 100 = ivafe : 0.2
            let ivafe = avg_balance * dec!(0.002);
            info!("IVAFE: {}", ivafe);
            ivafe.round_dp(2)
        }
    }

    /// Calculate the average balance along the year
    /// From Agenzia delle entrate: (<https://www.agenziaentrate.gov.it/portale/web/guest/schede/comunicazioni/integrativa-archivio-dei-rapporti-con-operatori-finanziari/giacenza-media-annua#:~:text=Il%20calcolo%20della%20giacenza%20media,il%20deposito%2Fconto%20risulta%20attivo.>)
    ///
    /// > Per giacenza media annua si intende l’importo medio delle somme
    /// > a credito del cliente in un dato periodo ragguagliato ad un anno.
    /// > Il calcolo della giacenza media annua si determina dividendo la somma delle giacenze giornaliere per 365,
    /// > indipendentemente dal numero di giorni in cui il deposito/conto risulta attivo.
    /// > Per giacenze giornaliere si intendono i saldi giornalieri per valuta.
    fn average_balance(&self) -> Decimal {
        let mut date = self.since;
        let mut total_balance = Decimal::ZERO;
        // Iterate over the days in the time range
        while date < self.to {
            let trades_at_date = self.trades.select(TradeQuery::default().before(date));
            let fiat_balance = trades_at_date.fiat_balance(Fiat::Eur);
            debug!(
                "FIAT balance at {} ({}): {}",
                date,
                date.ordinal(),
                fiat_balance
            );
            total_balance += fiat_balance;
            // calculate balance at date for each asset; get wallet at date first
            todo!("filter FIAT not EUR");
            let wallet = WalletDatabase::load(&trades_at_date);
            let wallet_balance = self.wallet_balance(wallet);
            debug!(
                "wallet balance at {} ({}): {}",
                date,
                date.ordinal(),
                wallet_balance
            );
            total_balance += wallet_balance;
            // incr total balance, days and date
            date += chrono::Duration::days(1);
        }
        (total_balance / Decimal::from(self.to.ordinal())).round_dp(2)
    }

    /// Get wallet balance from wallet
    fn wallet_balance(&self, wallet: WalletDatabase) -> Decimal {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::mock::database::{DatabaseQuoteMock, DatabaseTradeMock, DatabaseWalletMock};

    use chrono::prelude::*;

    #[test]
    fn should_init_taxes() {
        let trades = DatabaseTradeMock::mock();
        let quotes = DatabaseQuoteMock::mock();
        let wallet = DatabaseWalletMock::mock();
        let _ = mocked(&trades, &quotes, &wallet);
    }

    #[test]
    fn should_calc_ivafe() {
        let trades = DatabaseTradeMock::mock();
        let quotes = DatabaseQuoteMock::mock();
        let wallet = DatabaseWalletMock::mock();
        let tax = mocked(&trades, &quotes, &wallet);
        assert_eq!(tax.ivafe(), dec!(18.88));
    }

    #[test]
    fn should_calc_average_balance() {
        let trades = DatabaseTradeMock::mock();
        let quotes = DatabaseQuoteMock::mock();
        let wallet = DatabaseWalletMock::mock();
        let tax = mocked(&trades, &quotes, &wallet);
        assert_eq!(tax.average_balance(), dec!(9441.02));
    }

    fn mocked<'a>(
        trades: &'a TradeDatabase,
        quotes: &'a QuoteDatabase,
        wallet: &'a WalletDatabase,
    ) -> Taxes<'a> {
        let since = FixedOffset::east(3600).ymd(2022, 1, 1).and_hms(0, 0, 0);
        let to = FixedOffset::east(3600)
            .ymd(2022, 12, 31)
            .and_hms(23, 59, 59);
        Taxes::new(trades, quotes, wallet, since, to)
    }
}
