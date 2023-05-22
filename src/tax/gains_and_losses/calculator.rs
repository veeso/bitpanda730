//! # Calculator
//!
//! Gains and losses calculator

mod ticker_whitelist;
mod wallet;

use rust_decimal::Decimal;
use std::collections::HashMap;

use super::{CapitalDiff, GainsAndLosses};
use crate::database::TradeDatabase;
use bitpanda_csv::Trade;
use bitpanda_csv::{Asset, InOut, TransactionType};

use ticker_whitelist::TickerWhitelist;
use wallet::Wallet;

/// Gains and losses calculator from trades
#[derive(Debug, Default)]
pub struct Calculator {
    balance: HashMap<Asset, Wallet>,
}

impl Calculator {
    /// Calculate gains and losses from trade database
    pub fn calculate(&mut self, trades: &TradeDatabase) -> anyhow::Result<GainsAndLosses> {
        let mut stonks = vec![];
        debug!(
            "calculating gains and losses for {} trades",
            trades.all().trades().len()
        );
        // iter trades (only BUY, SELL, DEPOSIT, WITHDRAWAL)
        for trade in trades.all().trades() {
            // if the wallet update, produces a capital-diff, push it to gains and losses
            if let Some(capital_diff) = self.update_wallet(trade)? {
                stonks.push(capital_diff);
            }
        }

        Ok(GainsAndLosses::from(stonks).flatten())
    }

    /// Update wallet using trade.
    /// Optionally produce a capital diff (after a sell)
    fn update_wallet(&mut self, trade: &Trade) -> anyhow::Result<Option<CapitalDiff>> {
        debug!(
            "processing trade {} with asset {}",
            trade.transaction_id(),
            trade.asset()
        );
        // if buy, buy block in wallet, otherwise sell
        if trade.transaction_type() == TransactionType::Deposit
            || trade.transaction_type() == TransactionType::Buy
        {
            self.buy_asset(trade)
        } else if trade.transaction_type() == TransactionType::Transfer
            && trade.in_out() == InOut::Incoming
        {
            // NOTE: this is a stock split
            self.stock_split(trade)
        } else {
            self.sell_asset(trade)
        }
    }

    /// Buy asset
    fn buy_asset(&mut self, trade: &Trade) -> anyhow::Result<Option<CapitalDiff>> {
        let wallet = self.get_wallet(trade.asset());
        wallet.buy(
            trade.amount_asset().unwrap_or_default(),
            trade.amount_fiat(),
        );
        info!(
            "bought {} units of {} at € {}",
            trade.amount_asset().unwrap_or_default(),
            trade.asset(),
            trade.amount_fiat()
        );
        Ok(None)
    }

    /// Sell asset
    fn sell_asset(&mut self, trade: &Trade) -> anyhow::Result<Option<CapitalDiff>> {
        let wallet = self.get_wallet(trade.asset());
        // sell block
        let buy_amount_fiat = wallet.sell(trade.amount_asset().unwrap_or_default())?;
        if trade.transaction_type() == TransactionType::Sell {
            // Calc loss/gain
            let capital_diff = trade.amount_fiat() - buy_amount_fiat;
            info!(
                "sold {} units of {} at € {} (difference with buy price: € {})",
                trade.amount_asset().unwrap_or_default(),
                trade.asset(),
                trade.amount_fiat(),
                capital_diff
            );
            Ok(self.calc_capital_diff(capital_diff, trade))
        } else {
            info!("ignoring capital diff for withdrawal ({})", trade.asset());
            Ok(None)
        }
    }

    /// Perform a stock split on the trade asset
    fn stock_split(&mut self, trade: &Trade) -> anyhow::Result<Option<CapitalDiff>> {
        let wallet = self.get_wallet(trade.asset());
        info!(
            "stock split for {}; new amount: {}",
            trade.asset(),
            trade.amount_asset().unwrap_or_default()
        );
        wallet.stock_split(trade.amount_asset().unwrap_or_default());

        Ok(None)
    }

    /// Calculate capital diff from trade and diff amount
    fn calc_capital_diff(&self, diff: Decimal, trade: &Trade) -> Option<CapitalDiff> {
        if diff.is_zero() {
            None
        } else if diff.is_sign_negative() {
            Some(CapitalDiff::loss(
                trade.asset(),
                trade.asset_class(),
                self.tax_percentage(trade.asset()),
                diff,
            ))
        } else {
            Some(CapitalDiff::gain(
                trade.asset(),
                trade.asset_class(),
                self.tax_percentage(trade.asset()),
                diff,
            ))
        }
    }

    /// Return the tax percentage to apply to trade asset
    fn tax_percentage(&self, asset: Asset) -> Decimal {
        match asset {
            Asset::Currency(_) | Asset::Metal(_) | Asset::HongKong(_) => dec!(26.0),
            Asset::Ticker(ticker) if TickerWhitelist::is_whitelisted(&ticker) => dec!(12.50),
            Asset::Ticker(_) => dec!(26.0),
        }
    }

    /// Get wallet for asset.
    /// If wallet doesn't exist yet, create it.
    fn get_wallet(&mut self, asset: Asset) -> &mut Wallet {
        if !self.balance.contains_key(&asset) {
            debug!("initializing new wallet for {}", asset);
            self.balance.insert(asset.clone(), Wallet::default());
        }

        self.balance.get_mut(&asset).unwrap()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::database::DatabaseTradeMock;

    use bitpanda_csv::{CryptoCurrency, Currency, Metal};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_calculate_gains_and_losses() {
        crate::mock::log();
        let db = DatabaseTradeMock::mock();
        let mut calculator = Calculator::default();
        let gains_and_losses = calculator.calculate(&db).unwrap();
        assert_eq!(gains_and_losses.gains_value().round_dp(2), dec!(159.21));
        assert_eq!(gains_and_losses.losses_value().round_dp(2), dec!(-308.21));
        assert_eq!(gains_and_losses.tax_to_pay().round_dp(2), dec!(41.40));
        assert_eq!(gains_and_losses.iter().len(), 4);
    }

    #[test]
    fn should_calculate_gains_and_losses_correctly_when_a_stock_split_occurs() {
        crate::mock::log();
        let db = DatabaseTradeMock::google_stock_split_mock();
        let mut calculator = Calculator::default();
        let gains_and_losses = calculator.calculate(&db).unwrap();
        assert_eq!(gains_and_losses.losses_value(), Decimal::ZERO);
        assert_eq!(gains_and_losses.gains_value().round_dp(2), dec!(17.16));
    }

    #[test]
    fn should_tell_tax_percentage() {
        crate::mock::log();
        let calculator = Calculator::default();
        assert_eq!(
            calculator.tax_percentage(Asset::Metal(Metal::Gold)),
            dec!(26.0)
        );
        assert_eq!(
            calculator.tax_percentage(Asset::Currency(Currency::Crypto(CryptoCurrency::Btc))),
            dec!(26.0)
        );
        assert_eq!(calculator.tax_percentage(Asset::HongKong(1177)), dec!(26.0));
        assert_eq!(
            calculator.tax_percentage(Asset::Ticker(String::from("USGOVIES"))),
            dec!(12.50)
        );
        assert_eq!(
            calculator.tax_percentage(Asset::Ticker(String::from("AMZN"))),
            dec!(26.0)
        );
    }
}
