///! # Query
///
/// This module exposes the query which can be performed to select trades
use bitpanda_csv::Asset;
use chrono::{DateTime, FixedOffset};

use super::{Trade, TradeSet};

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
}

impl QueryFilter {
    fn includes(&self, trade: &Trade) -> bool {
        match self {
            Self::DateTimeAfter(date) => trade.timestamp() >= *date,
            Self::DateTimeBefore(date) => trade.timestamp() <= *date,
            Self::AssetNeq(asset) => trade.asset() != *asset,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::database::DatabaseTradeMock;
    use bitpanda_csv::{Currency, Fiat};

    use chrono::TimeZone;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_query_by_date_range() {
        crate::mock::log();
        let db = DatabaseTradeMock::mock();

        let query = Query::default()
            .after(
                FixedOffset::east_opt(3600)
                    .unwrap()
                    .with_ymd_and_hms(2022, 7, 5, 0, 0, 0)
                    .unwrap(),
            )
            .before(
                FixedOffset::east_opt(3600)
                    .unwrap()
                    .with_ymd_and_hms(2022, 9, 1, 0, 0, 0)
                    .unwrap(),
            );
        assert_eq!(query.select(&db.trades).trades().len(), 5);
    }

    #[test]
    fn should_query_by_asset_neq() {
        crate::mock::log();
        let db = DatabaseTradeMock::mock();

        let query = Query::default().asset_neq(Asset::Currency(Currency::Fiat(Fiat::Eur)));
        assert_eq!(query.select(&db.trades).trades().len(), 13);
    }
}
