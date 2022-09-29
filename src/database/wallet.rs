//! # Wallet
//!
//! The wallet database contains all the assets detained by your wallet

use super::TradeDatabase;
use crate::bitpanda::trade::{Asset, AssetClass, InOut, Trade, TransactionType};

use rust_decimal::Decimal;
use std::collections::HashMap;

/// Contains all the assets detained by the user
pub struct WalletDatabase {
    assets: HashMap<Asset, Decimal>,
}

impl WalletDatabase {
    /// Load wallet from user's trades
    pub fn load(trades: &TradeDatabase) -> Self {
        debug!("loading wallet database");
        let grouped_trades = trades.group_by_asset();
        debug!("loading {} assets", grouped_trades.len());
        let mut assets = HashMap::with_capacity(grouped_trades.len());
        for (asset, trades) in grouped_trades.into_iter() {
            debug!("counting assets amount for {:?}", asset);
            assets.insert(asset, Self::count(&trades));
        }

        Self { assets }
    }

    /// Get balance for provided asset
    pub fn balance(&self, asset: &Asset) -> Option<Decimal> {
        self.assets.get(asset).cloned()
    }

    /// Get the amount of assets detained from these trades
    fn count(trades: &[&Trade]) -> Decimal {
        let mut amount = Decimal::ZERO;
        amount += trades
            .iter()
            .filter(|t| Self::has_asset_increased(t))
            .map(|t| Self::asset_amount(t)) // NOTE: for incoming operations fee must be subtracted, since is kept by Bitpanda
            .sum::<Decimal>();
        amount -= trades
            .iter()
            .filter(|t| Self::has_asset_decreased(t))
            .map(|t| Self::asset_amount(t))
            .sum::<Decimal>();
        debug!("found {} assets", amount);
        amount
    }

    /// Check whether asset in trade has increased in quantity, according to these rules:
    ///
    /// - the trade is FIAT and the direction is IN
    /// - the transaction is BUY OR is INCOMING TRANSFER
    fn has_asset_increased(trade: &Trade) -> bool {
        if trade.asset_class() == AssetClass::Fiat && trade.in_out() == InOut::Incoming {
            true
        } else {
            trade.transaction_type() == TransactionType::Buy
                || (trade.transaction_type() == TransactionType::Transfer
                    && trade.in_out() == InOut::Incoming)
        }
    }

    /// Check whether asset in trade has decreased in quantity, according to these rules:
    ///
    /// - the trade is FIAT and the direction is OUT
    /// - the transaction is SELL OR is OUTGOING TRANSFER
    fn has_asset_decreased(trade: &Trade) -> bool {
        if trade.asset_class() == AssetClass::Fiat && trade.in_out() == InOut::Outgoing {
            true
        } else {
            trade.transaction_type() == TransactionType::Sell
                || (trade.transaction_type() == TransactionType::Transfer
                    && trade.in_out() == InOut::Outgoing)
        }
    }

    /// Get asset amount
    /// If asset is FIAT, get FIAT amount, otherwise amount asset
    fn asset_amount(trade: &Trade) -> Decimal {
        if trade.asset_class() == AssetClass::Fiat {
            trade.amount_fiat()
        } else {
            trade.amount_asset().unwrap_or_default()
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::bitpanda::trade::{CryptoCurrency, Currency, Fiat};
    use crate::mock::database::DatabaseTradeMock;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_load_wallet_database() {
        let trades = DatabaseTradeMock::mock();
        let db = WalletDatabase::load(&trades);
        assert_eq!(db.assets.len(), 8);
    }

    #[test]
    fn should_get_asset_balance_for_stock() {
        let trades = DatabaseTradeMock::mock();
        let db = WalletDatabase::load(&trades);
        assert_eq!(
            db.balance(&Asset::Name(String::from("AMZN"))).unwrap(),
            dec!(1.0)
        );
    }

    #[test]
    fn should_get_asset_balance_for_fiat() {
        let trades = DatabaseTradeMock::mock();
        let db = WalletDatabase::load(&trades);
        assert_eq!(
            db.balance(&Asset::Currency(Currency::Fiat(Fiat::Eur)))
                .unwrap(),
            dec!(9680.0)
        );
    }

    #[test]
    fn should_get_asset_balance_for_transfer() {
        let trades = DatabaseTradeMock::mock();
        let db = WalletDatabase::load(&trades);
        assert_eq!(
            db.balance(&Asset::Currency(Currency::Crypto(CryptoCurrency::Ada)))
                .unwrap(),
            dec!(100.0)
        );
    }
}
