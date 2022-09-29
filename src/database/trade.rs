//! # Trade
//!
//! This module defines the trade database

use crate::bitpanda::{
    trade::{Asset, AssetClass, Currency, InOut, TransactionType},
    Trade,
};

use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};

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
    /// get trades
    pub fn trades(&self) -> &[Trade] {
        &self.trades
    }

    /// Group trades by asset
    pub fn group_by_asset(&self) -> HashMap<Asset, Vec<&Trade>> {
        let mut grouped: HashMap<Asset, Vec<&Trade>> = HashMap::new();
        for trade in self.trades.iter() {
            if let Some(trades) = grouped.get_mut(&trade.asset()) {
                trades.push(trade)
            } else {
                grouped.insert(trade.asset(), vec![trade]);
            }
        }
        grouped
    }

    /// Collect assets and their class from database (unique)
    pub fn collect_assets(&self) -> Vec<(Asset, AssetClass)> {
        self.trades
            .iter()
            .map(|x| (x.asset(), x.asset_class()))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Get current FIAT balance in the bitpanda wallet
    pub fn fiat_balance(&self) -> Decimal {
        let trades: Vec<&Trade> = self.trades().iter().collect();
        self.calc_fiat_balance_from_trades(&trades)
    }

    /// Get the FIAT balance at the provided date
    pub fn fiat_balance_at(&self, date: DateTime<FixedOffset>) -> Decimal {
        let trades_at: Vec<&Trade> = self
            .trades()
            .iter()
            .filter(|t| t.timestamp() <= date)
            .collect();
        self.calc_fiat_balance_from_trades(&trades_at)
    }

    // -- private

    /// Calculate the FIAT balance from provided trades
    fn calc_fiat_balance_from_trades(&self, trades: &[&Trade]) -> Decimal {
        let incoming_fiat = trades
            .iter()
            .filter(|t| Self::is_fiat_incoming(t))
            .map(|t| t.amount_fiat() - t.fee().unwrap_or_default()) // NOTE: for incoming operations fee must be subtracted, since is kept by Bitpanda
            .sum::<Decimal>();
        debug!("total incoming fiat amount: {}", incoming_fiat);
        let outgoing_fiat = trades
            .iter()
            .filter(|t| Self::is_fiat_outgoing(t))
            .map(|t| t.amount_fiat())
            .sum::<Decimal>();
        debug!("total outgoing fiat amount: {}", outgoing_fiat);
        (incoming_fiat - outgoing_fiat).round_dp(2)
    }

    /// Returns whether trade is FIAT incoming
    fn is_fiat_incoming(trade: &Trade) -> bool {
        if trade.transaction_type() == TransactionType::Transfer
            && (trade.asset_class() == AssetClass::Stock
                || matches!(trade.asset(), Asset::Currency(Currency::Crypto(_))))
        {
            // NOTE: is stock split or staking
            false
        } else {
            trade.in_out() == InOut::Incoming
        }
    }

    /// Returns whether trade is FIAT outgoing
    fn is_fiat_outgoing(trade: &Trade) -> bool {
        if trade.transaction_type() == TransactionType::Transfer
            && matches!(trade.asset(), Asset::Currency(Currency::Crypto(_)))
        {
            // NOTE: is staking
            false
        } else {
            trade.in_out() == InOut::Outgoing
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::database::DatabaseTradeMock;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_trades() {
        let db = DatabaseTradeMock::mock();
        assert_eq!(db.trades().len(), 14);
    }

    #[test]
    fn should_group_by_asset() {
        let db = DatabaseTradeMock::mock();
        let groups = db.group_by_asset();
        assert_eq!(groups.len(), 8);
        assert_eq!(
            groups
                .get(&Asset::Name(String::from("AMZN")))
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn should_collect_assets() {
        let db = DatabaseTradeMock::mock();
        assert_eq!(db.collect_assets().len(), 8)
    }

    #[test]
    fn should_calc_balance() {
        let db = DatabaseTradeMock::mock();
        assert_eq!(db.fiat_balance(), dec!(7377.54));
    }

    #[test]
    fn should_calc_balance_at() {
        use chrono::prelude::*;
        let db = DatabaseTradeMock::mock();
        let date = FixedOffset::east(3600).ymd(2021, 08, 15).and_hms(0, 0, 0);
        assert_eq!(db.fiat_balance_at(date), dec!(7934.88));
    }
}
