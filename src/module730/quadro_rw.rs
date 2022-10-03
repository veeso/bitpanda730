//! # Quadro RW
//!
//! This module exposes the columns for the "Quadro RW" in 739

use rust_decimal::Decimal;

/// According to the 730:
///
/// > il quadro RW è quello dedicato al monitoraggio degli investimenti patrimoniali e
/// delle attività finanziarie detenuti all’estero da persone fisiche, enti non commerciali
/// con sede in Italia e società semplici.
///
/// Ref: <https://il730.online/come-compilare-il-quadro-rw-del-modello-redditi-pf-2022/>
pub struct QuadroRw {
    /// indicare il valore dell’IVAFE calcolata dal rapporto tra valore inserito nella colonna 8 alla quota e al periodo di detenzione.
    column11: Decimal,
}

impl QuadroRw {
    pub fn prepare(ivafe: Decimal) -> Self {
        todo!();
        Self { column11: ivafe }
    }
}
