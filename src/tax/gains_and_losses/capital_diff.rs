//! # Capital diff

use rust_decimal::Decimal;

use crate::bitpanda::trade::Asset;

/// Capital diff defines a gain or a loss in the investor's capital
#[derive(Debug, Clone)]
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

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
enum Diff {
    Gain,
    Loss,
}

impl CapitalDiff {
    /// Construct a Gain capital diff
    pub fn gain(asset: Asset, tax_percentage: Decimal, value: Decimal) -> Self {
        assert!(tax_percentage >= Decimal::ZERO && tax_percentage <= dec!(100.0));
        assert!(value.is_sign_positive());
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
        assert!(value.is_sign_negative());
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
    use crate::bitpanda::trade::Metal;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_init_gain() {
        let gain = CapitalDiff::gain(Asset::Metal(Metal::Gold), dec!(26.0), dec!(1000.0));
        assert!(gain.is_gain());
        assert!(matches!(gain.asset(), Asset::Metal(Metal::Gold)));
        assert_eq!(gain.is_loss(), false);
        assert_eq!(gain.tax(), dec!(260.0));
        assert_eq!(gain.tax_percentage(), dec!(26.0));
        assert_eq!(gain.value(), dec!(1000.0));
    }

    #[test]
    #[should_panic]
    fn should_panic_on_bad_tax_percentage() {
        CapitalDiff::gain(Asset::Metal(Metal::Gold), dec!(126.0), dec!(1000.0));
    }

    #[test]
    #[should_panic]
    fn should_panic_on_negative_tax_percentage() {
        CapitalDiff::gain(Asset::Metal(Metal::Gold), dec!(-26.0), dec!(1000.0));
    }

    #[test]
    fn should_init_loss() {
        let loss = CapitalDiff::loss(Asset::Metal(Metal::Gold), dec!(-56.0));
        assert_eq!(loss.is_loss(), true);
        assert_eq!(loss.is_gain(), false);
        assert_eq!(loss.tax(), Decimal::ZERO);
        assert_eq!(loss.tax_percentage(), Decimal::ZERO);
        assert_eq!(loss.value(), dec!(-56.0));
    }
}
