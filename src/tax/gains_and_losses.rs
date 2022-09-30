//! # Gains And Losses
//!
//! This module exposes the gains and losees type

use std::slice::Iter;

use rust_decimal::Decimal;

use crate::bitpanda::trade::Asset;

/// Gains and losses contains the different capital gains and losees calculated.
/// Taxes, assets and original amounts are stored
#[derive(Debug)]
pub struct GainsAndLosses {
    capitals: Vec<CapitalDiff>,
}

/// Capital diff defines a gain or a loss in the investor's capital
#[derive(Debug)]
pub struct CapitalDiff {
    /// Defines whether the capital diff is a gain or a loss
    diff: Diff,
    /// The asset the capital diff is referred to
    asset: Asset,
    /// The tax value applied
    tax: Decimal,
    /// The percentage applied to `value` to calculate the tax
    tax_percentage: Decimal,
    /// The value of the capital difference (if positive is a gain, if negative is a loss)
    value: Decimal,
}

#[derive(Debug, Eq, PartialEq)]
enum Diff {
    Gain,
    Loss,
}

impl GainsAndLosses {
    /// Returns an iterator over gains and losses
    pub fn iter(&self) -> Iter<'_, CapitalDiff> {
        self.capitals.iter()
    }

    /// Get the amount (value) of gains
    pub fn gains_value(&self) -> Decimal {
        self.capitals
            .iter()
            .filter(|x| x.is_gain())
            .map(|x| x.value())
            .sum()
    }

    /// Get the amount (value) of losses
    pub fn losses_value(&self) -> Decimal {
        self.capitals
            .iter()
            .filter(|x| x.is_loss())
            .map(|x| x.value())
            .sum()
    }

    /// Get the total amount of tax to pay
    pub fn tax_to_pay(&self) -> Decimal {
        self.capitals.iter().map(|x| x.tax()).sum()
    }
}

impl CapitalDiff {
    /// Construct a Gain capital diff
    pub fn gain(asset: Asset, tax_percentage: Decimal, value: Decimal) -> Self {
        assert!(tax_percentage >= Decimal::ZERO && tax_percentage <= dec!(100.0));
        let tax = value * (tax_percentage / dec!(100.0)).round_dp(2);
        Self {
            diff: Diff::Gain,
            asset,
            tax,
            tax_percentage,
            value,
        }
    }

    /// Construct a Loss capital diff
    pub fn loss(asset: Asset, value: Decimal) -> Self {
        Self {
            diff: Diff::Loss,
            asset,
            tax: Decimal::ZERO,
            tax_percentage: Decimal::ZERO,
            value,
        }
    }

    /// Returns whether this capital diff is a gain
    pub fn is_gain(&self) -> bool {
        self.diff == Diff::Gain
    }

    /// Returns whether this capital diff is a loss
    pub fn is_loss(&self) -> bool {
        self.diff == Diff::Loss
    }

    /// The asset associated to the gain/loss
    pub fn asset(&self) -> &Asset {
        &self.asset
    }

    /// The tax which must be paid on capital difference
    pub fn tax(&self) -> Decimal {
        self.tax
    }

    /// The tax percentage applied
    pub fn tax_percentage(&self) -> Decimal {
        self.tax_percentage
    }

    /// Returns the value of the gain or of the loss
    pub fn value(&self) -> Decimal {
        self.value
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_init_gains_and_losses() {
        todo!()
    }
}
