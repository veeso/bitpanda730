///! # Query
///
/// This module exposes the query which can be performed to select trades
use chrono::{DateTime, FixedOffset};

use super::{Trade, TradeSet};
use crate::bitpanda::trade::{Asset, TransactionType};

/// Query statement for trade
#[derive(Default, Debug)]
pub struct Query {
    filters: Vec<QueryFilter>,
}

impl Query {
    pub(super) fn select(self, trades: &[Trade]) -> TradeSet {
        debug!("selecting trades which satisfy query {:?}", self);
        TradeSet::from_iter(trades.iter().filter(|trade| self.filter(trade)))
    }

    /// Select only trades after `date`
    pub fn after(mut self, date: DateTime<FixedOffset>) -> Self {
        self.filters.push(QueryFilter::DateTimeAfter(date));
        self
    }

    /// Select only trades before `date`
    pub fn before(mut self, date: DateTime<FixedOffset>) -> Self {
        self.filters.push(QueryFilter::DateTimeBefore(date));
        self
    }

    /// Select only trades which asset is NOT equal to `asset`
    pub fn asset_neq(mut self, asset: Asset) -> Self {
        self.filters.push(QueryFilter::AssetNeq(asset));
        self
    }

    /// Select only trades which transaction type is included in `types`
    pub fn has_transaction_type(mut self, types: Vec<TransactionType>) -> Self {
        self.filters.push(QueryFilter::TransactionTypes(types));
        self
    }

    /// apply filters for trade
    fn filter(&self, trade: &Trade) -> bool {
        for filter in self.filters.iter() {
            if !filter.includes(trade) {
                return false;
            }
        }
        true
    }
}

/// A single filter to apply to trades
#[derive(Debug)]
pub enum QueryFilter {
    AssetNeq(Asset),
    DateTimeAfter(DateTime<FixedOffset>),
    DateTimeBefore(DateTime<FixedOffset>),
    TransactionTypes(Vec<TransactionType>),
}

impl QueryFilter {
    fn includes(&self, trade: &Trade) -> bool {
        match self {
            Self::DateTimeAfter(date) => trade.timestamp() >= *date,
            Self::DateTimeBefore(date) => trade.timestamp() <= *date,
            Self::AssetNeq(asset) => trade.asset() != *asset,
            Self::TransactionTypes(types) => types.contains(&trade.transaction_type()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::bitpanda::trade::{Currency, Fiat};
    use crate::mock::database::DatabaseTradeMock;

    use chrono::TimeZone;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_query_by_date_range() {
        let db = DatabaseTradeMock::mock();

        let query = Query::default()
            .after(FixedOffset::east(3600).ymd(2021, 7, 5).and_hms(0, 0, 0))
            .before(FixedOffset::east(3600).ymd(2021, 9, 1).and_hms(0, 0, 0));
        assert_eq!(query.select(&db.trades).trades().len(), 5);
    }

    #[test]
    fn should_query_by_asset_neq() {
        let db = DatabaseTradeMock::mock();

        let query = Query::default().asset_neq(Asset::Currency(Currency::Fiat(Fiat::Eur)));
        assert_eq!(query.select(&db.trades).trades().len(), 13);
    }

    #[test]
    fn should_query_by_transaction_type() {
        let db = DatabaseTradeMock::mock();

        let query = Query::default()
            .has_transaction_type(vec![TransactionType::Buy, TransactionType::Sell]);
        assert_eq!(query.select(&db.trades).trades().len(), 11);
    }
}
