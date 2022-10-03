//! # Gains And Losses
//!
//! This module exposes the gains and losees type

use rust_decimal::Decimal;
use std::collections::HashSet;
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
    pub fn flatten(mut self) -> Self {
        // group capitals by asset
        let capitals_by_asset = Self::group_gains_and_losses_by_asset(self.capitals);
        // flatten capitals by asset
        self.capitals = capitals_by_asset
            .into_iter()
            .filter_map(Self::capital_diffs_flatten)
            .collect();

        self
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

    /// Group the list of capital diffs into a list of list of capitals diff where each list
    /// is grouped by the asset kind
    fn group_gains_and_losses_by_asset(capitals: Vec<CapitalDiff>) -> Vec<Vec<CapitalDiff>> {
        let mut capitals_by_asset: Vec<Vec<CapitalDiff>> = vec![];
        // iter assets
        for asset in capitals
            .iter()
            .map(|x| x.asset().clone())
            .collect::<HashSet<_>>()
            .into_iter()
        {
            capitals_by_asset.push(
                capitals
                    .iter()
                    .filter(|x| x.asset() == &asset)
                    .cloned()
                    .collect(),
            );
        }
        capitals_by_asset
    }

    /// Flat a list of capital diff
    fn capital_diffs_flatten(capitals_diff: Vec<CapitalDiff>) -> Option<CapitalDiff> {
        let asset = capitals_diff
            .iter()
            .map(|x| x.asset())
            .next()
            .unwrap()
            .clone();
        let asset_class = capitals_diff
            .iter()
            .map(|x| x.asset_class())
            .next()
            .unwrap();
        let total_value: Decimal = capitals_diff.iter().map(|x| x.value()).sum();
        debug!(
            "flattening capital diffs of {}; total value: {}",
            asset, total_value
        );
        if total_value.is_zero() {
            None
        } else if total_value.is_sign_positive() {
            // average tax on
            let tax_percentage: Decimal = capitals_diff
                .iter()
                .filter(|x| x.is_gain())
                .map(|x| x.tax_percentage())
                .max()
                .unwrap();
            Some(CapitalDiff::gain(
                asset,
                asset_class,
                tax_percentage,
                total_value,
            ))
        } else {
            Some(CapitalDiff::loss(asset, asset_class, total_value))
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use bitpanda_csv::{Asset, AssetClass, Metal};

    use pretty_assertions::assert_eq;

    #[test]
    fn should_init_gains_and_losses() {
        let gain_and_losses = GainsAndLosses::from(vec![
            CapitalDiff::gain(
                Asset::Metal(Metal::Gold),
                AssetClass::Metal,
                dec!(26.0),
                dec!(500.0),
            ),
            CapitalDiff::gain(
                Asset::Metal(Metal::Palladium),
                AssetClass::Metal,
                dec!(11.0),
                dec!(100.0),
            ),
            CapitalDiff::gain(
                Asset::Metal(Metal::Silver),
                AssetClass::Metal,
                dec!(50.0),
                dec!(600.0),
            ),
            CapitalDiff::loss(
                Asset::Ticker(String::from("TSLA")),
                AssetClass::Stock,
                dec!(-32.0),
            ),
            CapitalDiff::loss(
                Asset::Ticker(String::from("NASDAQ100")),
                AssetClass::Etf,
                dec!(-400.0),
            ),
        ]);
        assert_eq!(gain_and_losses.capitals.len(), 5);
    }

    #[test]
    fn should_calc_gains_and_losses() {
        let gain_and_losses = GainsAndLosses::from(vec![
            CapitalDiff::gain(
                Asset::Metal(Metal::Gold),
                AssetClass::Metal,
                dec!(26.0),
                dec!(500.0),
            ),
            CapitalDiff::gain(
                Asset::Metal(Metal::Palladium),
                AssetClass::Metal,
                dec!(11.0),
                dec!(100.0),
            ),
            CapitalDiff::gain(
                Asset::Metal(Metal::Silver),
                AssetClass::Metal,
                dec!(50.0),
                dec!(600.0),
            ),
            CapitalDiff::loss(
                Asset::Ticker(String::from("TSLA")),
                AssetClass::Stock,
                dec!(-32.0),
            ),
            CapitalDiff::loss(
                Asset::Ticker(String::from("NASDAQ100")),
                AssetClass::Etf,
                dec!(-400.0),
            ),
        ]);
        assert_eq!(gain_and_losses.gains_value(), dec!(1200.0));
        assert_eq!(gain_and_losses.losses_value(), dec!(-432.0));
        assert_eq!(gain_and_losses.tax_to_pay(), dec!(441.0));
    }

    #[test]
    fn should_flat_gains_and_losses() {
        let gain_and_losses = GainsAndLosses::from(vec![
            CapitalDiff::gain(
                Asset::Metal(Metal::Gold),
                AssetClass::Metal,
                dec!(26.0),
                dec!(500.0),
            ),
            CapitalDiff::gain(
                Asset::Metal(Metal::Gold),
                AssetClass::Metal,
                dec!(26.0),
                dec!(700.0),
            ),
            CapitalDiff::gain(
                Asset::Metal(Metal::Silver),
                AssetClass::Metal,
                dec!(26.0),
                dec!(200.0),
            ),
            CapitalDiff::loss(Asset::Metal(Metal::Silver), AssetClass::Metal, dec!(-50.0)),
            CapitalDiff::gain(
                Asset::Metal(Metal::Palladium),
                AssetClass::Metal,
                dec!(26.0),
                dec!(150.0),
            ),
            CapitalDiff::loss(
                Asset::Metal(Metal::Palladium),
                AssetClass::Metal,
                dec!(-350.0),
            ),
            CapitalDiff::gain(
                Asset::Metal(Metal::Platinum),
                AssetClass::Metal,
                dec!(26.0),
                dec!(500.0),
            ), // ignored since total diff is 0
            CapitalDiff::loss(
                Asset::Metal(Metal::Platinum),
                AssetClass::Metal,
                dec!(-100.0),
            ),
            CapitalDiff::loss(
                Asset::Metal(Metal::Platinum),
                AssetClass::Metal,
                dec!(-400.0),
            ),
        ])
        .flatten();
        assert_eq!(gain_and_losses.capitals.len(), 3);
        assert_eq!(gain_and_losses.gains_value(), dec!(1350.0));
        assert_eq!(gain_and_losses.losses_value(), dec!(-200.0));
        assert_eq!(gain_and_losses.tax_to_pay(), dec!(351.0));
    }
}
