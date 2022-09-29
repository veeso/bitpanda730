//! # Tax
//!
//! This module expose the tax calculators for Italian taxation ruleset

use crate::database::TradeDatabase;

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
    since: DateTime<FixedOffset>,
    to: DateTime<FixedOffset>,
}

impl<'a> Taxes<'a> {
    pub fn new(
        trades: &'a TradeDatabase,
        since: DateTime<FixedOffset>,
        to: DateTime<FixedOffset>,
    ) -> Self {
        Self { trades, since, to }
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
            todo!("calc price for the day; asset price must be calculated using the latest year quotation");
            let fiat_balance = self.trades.fiat_balance_at(date);
            debug!(
                "FIAT balance at {} ({}): {}",
                date,
                date.ordinal(),
                fiat_balance
            );
            total_balance += fiat_balance;
            // calculate balance at date for each asset
            todo!("calc balance at date for each asset");
            // incr total balance, days and date
            date += chrono::Duration::days(1);
        }
        (total_balance / Decimal::from(self.to.ordinal())).round_dp(2)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::mock::database::DatabaseTradeMock;

    use chrono::prelude::*;

    #[test]
    fn should_init_taxes() {
        let db = DatabaseTradeMock::mock();
        let _ = mocked(&db);
    }

    #[test]
    fn should_calc_ivafe() {
        let db = DatabaseTradeMock::mock();
        let tax = mocked(&db);
        assert_eq!(tax.ivafe(), dec!(18.88));
    }

    #[test]
    fn should_calc_average_balance() {
        let db = DatabaseTradeMock::mock();
        let tax = mocked(&db);
        assert_eq!(tax.average_balance(), dec!(9441.02));
    }

    fn mocked(db: &TradeDatabase) -> Taxes {
        let since = FixedOffset::east(3600).ymd(2022, 1, 1).and_hms(0, 0, 0);
        let to = FixedOffset::east(3600)
            .ymd(2022, 12, 31)
            .and_hms(23, 59, 59);
        Taxes::new(db, since, to)
    }
}
