//! # Trade
//!
//! This module defines the trade database

use bitpanda_csv::Trade;

mod query;
mod set;
pub use query::Query as TradeQuery;
pub use set::Set as TradeSet;

/// The trade database contains all the trades parsed from the CSV
/// and exposes methods to query the trade datas
#[derive(Debug, Clone)]
pub struct TradeDatabase {
    trades: Vec<Trade>,
}

impl From<Vec<Trade>> for TradeDatabase {
    fn from(trades: Vec<Trade>) -> Self {
        Self { trades }
    }
}

impl TradeDatabase {
    /// select all trades.
    /// Shorthand for `select(TradeQuery::default())`
    pub fn all(&self) -> TradeSet {
        self.select(TradeQuery::default())
    }

    /// Select only trades which satisfies the query
    pub fn select(&self, query: TradeQuery) -> TradeSet {
        query.select(&self.trades)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::database::DatabaseTradeMock;
    use bitpanda_csv::{Asset, Fiat};

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_trades() {
        crate::mock::log();
        let db = DatabaseTradeMock::mock();
        assert_eq!(db.all().trades().len(), 15);
    }

    #[test]
    fn should_group_by_asset() {
        crate::mock::log();
        let db = DatabaseTradeMock::mock();
        let set = db.all();
        let groups = set.group_by_asset();
        assert_eq!(groups.len(), 9);
        assert_eq!(
            groups
                .get(&Asset::Ticker(String::from("AMZN")))
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn should_collect_assets() {
        crate::mock::log();
        let db = DatabaseTradeMock::mock();
        assert_eq!(db.all().collect_assets().len(), 9)
    }

    #[test]
    fn should_calc_balance() {
        crate::mock::log();
        let db = DatabaseTradeMock::mock();
        assert_eq!(db.all().fiat_balance(Fiat::Eur), dec!(7377.54));
        assert_eq!(db.all().fiat_balance(Fiat::Usd), dec!(1000.0));
    }

    #[test]
    fn should_calc_balance_at() {
        crate::mock::log();
        use chrono::prelude::*;
        let db = DatabaseTradeMock::mock();
        let date = FixedOffset::east_opt(3600)
            .unwrap()
            .with_ymd_and_hms(2022, 08, 15, 0, 0, 0)
            .unwrap();
        assert_eq!(
            db.select(TradeQuery::default().before(date))
                .fiat_balance(Fiat::Eur),
            dec!(7934.88)
        );
    }
}
