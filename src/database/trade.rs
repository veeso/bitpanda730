//! # Trade
//!
//! This module defines the trade database

use crate::bitpanda::Trade;

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
    use crate::bitpanda::trade::{Asset, Fiat};
    use crate::mock::database::DatabaseTradeMock;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_trades() {
        let db = DatabaseTradeMock::mock();
        assert_eq!(db.all().trades().len(), 15);
    }

    #[test]
    fn should_group_by_asset() {
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
        let db = DatabaseTradeMock::mock();
        assert_eq!(db.all().collect_assets().len(), 9)
    }

    #[test]
    fn should_calc_balance() {
        let db = DatabaseTradeMock::mock();
        assert_eq!(db.all().fiat_balance(Fiat::Eur), dec!(7377.54));
        assert_eq!(db.all().fiat_balance(Fiat::Usd), dec!(1000.0));
    }

    #[test]
    fn should_calc_balance_at() {
        use chrono::prelude::*;
        let db = DatabaseTradeMock::mock();
        let date = FixedOffset::east(3600).ymd(2021, 08, 15).and_hms(0, 0, 0);
        assert_eq!(
            db.select(TradeQuery::default().before(date))
                .fiat_balance(Fiat::Eur),
            dec!(7934.88)
        );
    }
}
