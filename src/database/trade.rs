//! # Trade
//!
//! This module defines the trade database

use crate::bitpanda::{
    trade::{Asset, InOut, TransactionType},
    Trade,
};

use rust_decimal::Decimal;
use std::collections::HashMap;

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

    /// Get current FIAT balance in the bitpanda wallet
    pub fn fiat_balance(&self) -> Decimal {
        let mut balance = Decimal::ZERO;
        todo!("something must be excluded here in the calc (some transfers are glitched?)")
        balance += self
            .trades
            .iter()
            .filter(|t| t.in_out() == InOut::Incoming)
            .map(|t| t.amount_fiat() - t.fee().unwrap_or_default()) // NOTE: for incoming operations fee must be subtracted, since is kept by Bitpanda
            .sum::<Decimal>();
        balance -= self
            .trades
            .iter()
            .filter(|t| t.in_out() == InOut::Outgoing)
            .map(|t| t.amount_fiat())
            .sum::<Decimal>();
        balance
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
        assert_eq!(db.trades().len(), 9);
    }

    #[test]
    fn should_group_by_asset() {
        let db = DatabaseTradeMock::mock();
        let groups = db.group_by_asset();
        assert_eq!(groups.len(), 5);
        assert_eq!(
            groups
                .get(&Asset::Name(String::from("AMZN")))
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn should_calc_balance() {
        let db = DatabaseTradeMock::mock();
        assert_eq!(db.fiat_balance(), dec!(9219.90));
    }
}
