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
    pub sezione_5: Sezione5,
}

/// Sezione I - Plusvalenze assoggettate ad imposta sostitutiva del 12.5%
#[derive(Debug)]
pub struct Sezione1 {
    /// Corrispettivo incassatoc (gain + loss)
    pub rt1: Decimal,
    /// Valore fiscale riconosciuto alla partecipazione; gain
    pub rt2_col3: Decimal,
    /// Plusvalenza (RT1 - RT2); only if > 0
    pub rt3: Option<Decimal>,
}

/// Sezione II - Plusvalenze assoggettate ad imposta sostitutiva del 26%
#[derive(Debug)]
pub struct Sezione2 {
    /// Corrispettivo incassato (gain + loss)
    pub rt21: Decimal,
    /// Valore fiscale riconosciuto alla partecipazione; gain
    pub rt22_col3: Decimal,
    /// Plusvalenza (RT21 - RT22); only if > 0
    pub rt23: Option<Decimal>,
}

/// Sezione V - Minusvalenze non compensate nell'anno
#[derive(Debug)]
pub struct Sezione5 {
    /// Minusvalenze anno corrente NON compensate
    pub rt93_col5: Decimal,
}

impl QuadroRt {
    pub fn prepare(gains_and_losses: GainsAndLosses) -> Self {
        let uncompensated_loss = todo!();
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
            sezione_5: Sezione5::prepare(uncompensated_loss),
        }
    }
}

impl Sezione1 {
    pub fn prepare(gains_and_losses_12_percent: GainsAndLosses) -> Self {
        todo!()
    }
}

impl Sezione2 {
    pub fn prepare(gains_and_losses_26_percent: GainsAndLosses) -> Self {
        todo!()
    }
}

impl Sezione5 {
    pub fn prepare(uncompensated_loss: Decimal) -> Self {
        Self {
            rt93_col5: uncompensated_loss,
        }
    }
}
