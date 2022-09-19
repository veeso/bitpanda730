//! # Trade
//!
//! This module defines the trade database

use std::collections::HashMap;

use crate::bitpanda::{trade::Asset, Trade};

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
    /// Get database items count
    pub fn len(&self) -> usize {
        self.trades.len()
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
}
