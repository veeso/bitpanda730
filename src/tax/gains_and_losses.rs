//! # Gains And Losses
//!
//! This module exposes the gains and losees type

use rust_decimal::Decimal;
use std::slice::Iter;

mod calculator;
mod capital_diff;

pub use calculator::Calculator;
pub use capital_diff::CapitalDiff;

/// Gains and losses contains the different capital gains and losees calculated.
/// Taxes, assets and original amounts are stored
#[derive(Debug)]
pub struct GainsAndLosses {
    capitals: Vec<CapitalDiff>,
}

impl From<Vec<CapitalDiff>> for GainsAndLosses {
    fn from(capitals: Vec<CapitalDiff>) -> Self {
        Self { capitals }
    }
}

impl GainsAndLosses {
    /// Returns an iterator over gains and losses
    pub fn iter(&self) -> Iter<'_, CapitalDiff> {
        self.capitals.iter()
    }

    /// Group gains and losses by the same assets and create a unique capital diff for them
    pub fn flatten(&mut self) {
        todo!();
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

#[cfg(test)]
mod test {

    use super::*;
    use crate::bitpanda::trade::{Asset, Metal};

    use pretty_assertions::assert_eq;

    #[test]
    fn should_init_gains_and_losses() {
        let gain_and_losses = GainsAndLosses::from(vec![
            CapitalDiff::gain(Asset::Metal(Metal::Gold), dec!(26.0), dec!(500.0)),
            CapitalDiff::gain(Asset::Metal(Metal::Palladium), dec!(11.0), dec!(100.0)),
            CapitalDiff::gain(Asset::Metal(Metal::Silver), dec!(50.0), dec!(600.0)),
            CapitalDiff::loss(Asset::Name(String::from("TSLA")), dec!(32.0)),
            CapitalDiff::loss(Asset::Name(String::from("NASDAQ100")), dec!(400.0)),
        ]);
        assert_eq!(gain_and_losses.capitals.len(), 5);
    }

    #[test]
    fn should_calc_gains_and_losses() {
        let gain_and_losses = GainsAndLosses::from(vec![
            CapitalDiff::gain(Asset::Metal(Metal::Gold), dec!(26.0), dec!(500.0)),
            CapitalDiff::gain(Asset::Metal(Metal::Palladium), dec!(11.0), dec!(100.0)),
            CapitalDiff::gain(Asset::Metal(Metal::Silver), dec!(50.0), dec!(600.0)),
            CapitalDiff::loss(Asset::Name(String::from("TSLA")), dec!(32.0)),
            CapitalDiff::loss(Asset::Name(String::from("NASDAQ100")), dec!(400.0)),
        ]);
        assert_eq!(gain_and_losses.gains_value(), dec!(1200.0));
        assert_eq!(gain_and_losses.losses_value(), dec!(432.0));
        assert_eq!(gain_and_losses.tax_to_pay(), dec!(441.0));
    }
}
