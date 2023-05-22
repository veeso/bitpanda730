//! # Quadro RT
//!
//! This module exposes the "Quadro RT" data for the "730"

use rust_decimal::Decimal;

use crate::tax::{CapitalDiff, GainsAndLosses};

/// Quadro RT - Plusvalenze di natura finanziaria
///
/// Ref: <https://info730.agenziaentrate.it/portale/istruzioni-per-la-compilazione-del-quadro-rt>
#[derive(Debug)]
pub struct QuadroRt {
    pub sezione_1: Sezione1,
    pub sezione_2: Sezione2,
}

/// Sezione I - Plusvalenze assoggettate ad imposta sostitutiva del 12.5%
#[derive(Debug)]
pub struct Sezione1 {
    /// Corrispettivo incassato (gain + loss)
    pub rt1: Decimal,
    /// Valore fiscale riconosciuto alla partecipazione; loss
    pub rt2_col3: Decimal,
    /// Plusvalenza (RT1 - RT2); only if < 0
    pub rt3_col1: Option<Decimal>,
    /// Plusvalenza (RT1 - RT2); only if > 0
    pub rt3_col2: Option<Decimal>,
}

/// Sezione II - Plusvalenze assoggettate ad imposta sostitutiva del 26%
#[derive(Debug)]
pub struct Sezione2 {
    /// Corrispettivo incassato (gain + loss)
    pub rt21: Decimal,
    /// Valore fiscale riconosciuto alla partecipazione; loss
    pub rt22_col3: Decimal,
    /// Plusvalenza (RT21 - RT22); only if < 0
    pub rt23_col1: Option<Decimal>,
    /// Plusvalenza (RT21 - RT22); only if > 0
    pub rt23_col2: Option<Decimal>,
}

impl QuadroRt {
    pub fn prepare(gains_and_losses: &GainsAndLosses) -> Self {
        Self {
            sezione_1: Sezione1::prepare(
                gains_and_losses
                    .iter()
                    .filter(|x| x.tax_percentage() == dec!(12.5))
                    .cloned()
                    .collect::<Vec<CapitalDiff>>()
                    .into(),
            ),
            sezione_2: Sezione2::prepare(
                gains_and_losses
                    .iter()
                    .filter(|x| x.tax_percentage() == dec!(26.0))
                    .cloned()
                    .collect::<Vec<CapitalDiff>>()
                    .into(),
            ),
        }
    }
}

impl Sezione1 {
    pub fn prepare(gains_and_losses_12_percent: GainsAndLosses) -> Self {
        let total_sold = gains_and_losses_12_percent
            .iter()
            .map(|x| x.value().abs())
            .sum::<Decimal>()
            .round_dp(2);
        let loss = gains_and_losses_12_percent
            .iter()
            .filter(|x| x.is_loss())
            .map(|x| x.value().abs())
            .sum::<Decimal>()
            .round_dp(2);
        let diff: Decimal = total_sold - loss;
        let rt3 = if diff.is_sign_negative() {
            (Some(diff.abs()), None)
        } else {
            (None, Some(diff))
        };
        Self {
            rt1: total_sold,
            rt2_col3: loss,
            rt3_col1: rt3.0,
            rt3_col2: rt3.1,
        }
    }
}

impl Sezione2 {
    pub fn prepare(gains_and_losses_26_percent: GainsAndLosses) -> Self {
        let total_sold = gains_and_losses_26_percent
            .iter()
            .map(|x| x.value().abs())
            .sum::<Decimal>()
            .round_dp(2);
        let loss = gains_and_losses_26_percent
            .iter()
            .filter(|x| x.is_loss())
            .map(|x| x.value().abs())
            .sum::<Decimal>()
            .round_dp(2);
        let diff: Decimal = total_sold - loss;
        let rt23 = if diff.is_sign_negative() {
            (Some(diff.abs()), None)
        } else {
            (None, Some(diff))
        };
        Self {
            rt21: total_sold,
            rt22_col3: loss,
            rt23_col1: rt23.0,
            rt23_col2: rt23.1,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use bitpanda_csv::{Asset, AssetClass, Metal};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_prepare_quadro_rt() {
        crate::mock::log();
        let quadro_rt = QuadroRt::prepare(&gains_and_losses());
        assert_eq!(quadro_rt.sezione_1.rt1, dec!(680.0));
        assert_eq!(quadro_rt.sezione_1.rt2_col3, dec!(80.0));
        assert_eq!(quadro_rt.sezione_1.rt3_col1, None);
        assert_eq!(quadro_rt.sezione_1.rt3_col2, Some(dec!(600.0)));

        assert_eq!(quadro_rt.sezione_2.rt21, dec!(632.0));
        assert_eq!(quadro_rt.sezione_2.rt22_col3, dec!(32.0));
        assert_eq!(quadro_rt.sezione_2.rt23_col1, None);
        assert_eq!(quadro_rt.sezione_2.rt23_col2, Some(dec!(600.0)));
    }

    fn gains_and_losses() -> GainsAndLosses {
        GainsAndLosses::from(vec![
            CapitalDiff::gain(
                Asset::Ticker(String::from("USGOVIES")),
                AssetClass::Etf,
                dec!(12.5),
                dec!(500.0),
            ),
            CapitalDiff::gain(
                Asset::Ticker(String::from("EUROGOV")),
                AssetClass::Etf,
                dec!(12.5),
                dec!(100.0),
            ),
            CapitalDiff::loss(
                Asset::Ticker(String::from("CHINABOND")),
                AssetClass::Etf,
                dec!(12.5),
                dec!(-80.0),
            ),
            // 26%
            CapitalDiff::gain(
                Asset::Metal(Metal::Gold),
                AssetClass::Metal,
                dec!(26.0),
                dec!(500.0),
            ),
            CapitalDiff::gain(
                Asset::Ticker(String::from("AMZN")),
                AssetClass::Stock,
                dec!(26.0),
                dec!(100.0),
            ),
            CapitalDiff::loss(
                Asset::Ticker(String::from("TSLA")),
                AssetClass::Stock,
                dec!(26.0),
                dec!(-32.0),
            ),
        ])
    }
}
