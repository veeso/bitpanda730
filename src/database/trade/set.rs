//! # Set
//!
//! This module expose a select result on the trade database

use super::Trade;
use bitpanda_csv::{Asset, AssetClass, Currency, Fiat, InOut, TransactionType};

use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub struct Set<'a> {
    trades: Vec<&'a Trade>,
}

impl<'a> FromIterator<&'a Trade> for Set<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Trade>>(iter: T) -> Self {
        let mut trades = Vec::new();
        for t in iter {
            trades.push(t);
        }
        Self { trades }
    }
}

impl<'a> Set<'a> {
    pub fn trades(&self) -> &[&'a Trade] {
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
    pub fn fiat_balance(&self, fiat: Fiat) -> Decimal {
        let incoming_fiat = self
            .trades
            .iter()
            .filter(|t| t.fiat() == fiat)
            .filter(|t| Self::is_fiat_incoming(t))
            .map(|t| t.amount_fiat() - t.fee().unwrap_or_default()) // NOTE: for incoming operations fee must be subtracted, since is kept by Bitpanda
            .sum::<Decimal>();
        debug!("total incoming fiat amount: {}", incoming_fiat);
        let outgoing_fiat = self
            .trades
            .iter()
            .filter(|t| t.fiat() == fiat)
            .filter(|t| Self::is_fiat_outgoing(t))
            .map(|t| t.amount_fiat())
            .sum::<Decimal>();
        debug!("total outgoing fiat amount: {}", outgoing_fiat);
        (incoming_fiat - outgoing_fiat).round_dp(2)
    }

    // -- private

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
