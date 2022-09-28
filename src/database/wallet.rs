//! # Wallet
//!
//! The wallet database contains all the assets detained by your wallet

use super::TradeDatabase;
use crate::bitpanda::trade::{Asset, InOut, Trade};

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
        todo!("failed; check buy sell");
        let mut amount = Decimal::ZERO;
        amount += trades
            .iter()
            .filter(|t| t.in_out() == InOut::Incoming && t.amount_asset().is_some())
            .map(|t| t.amount_asset().unwrap_or_default()) // NOTE: for incoming operations fee must be subtracted, since is kept by Bitpanda
            .sum::<Decimal>();
        amount -= trades
            .iter()
            .filter(|t| t.in_out() == InOut::Outgoing && t.amount_asset().is_some())
            .map(|t| t.amount_asset().unwrap_or_default())
            .sum::<Decimal>();
        debug!("found {} assets", amount);
        amount
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::database::DatabaseTradeMock;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_load_wallet_database() {
        let trades = DatabaseTradeMock::mock();
        let db = WalletDatabase::load(&trades);
        assert_eq!(db.assets.len(), 6);
    }

    #[test]
    fn should_get_asset_balance() {
        let trades = DatabaseTradeMock::mock();
        let db = WalletDatabase::load(&trades);
        assert_eq!(
            db.balance(&Asset::Name(String::from("AMZN"))).unwrap(),
            dec!(1.0)
        );
    }
}
