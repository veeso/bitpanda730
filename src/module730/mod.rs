//! # Module 730
//!
//! This module exposes utilities to compile the 730 module

use rust_decimal::Decimal;

use crate::tax::GainsAndLosses;

mod paginate;
mod quadro_rt;
mod quadro_rw;

pub use paginate::{Paginate, Stdout};
use quadro_rt::QuadroRt;
use quadro_rw::QuadroRw;

/// Module 730 data for investments gains
#[derive(Debug)]
pub struct Module730 {
    pub quadro_rt: QuadroRt,
    pub quadro_rw: QuadroRw,
}

impl Module730 {
    /// Instantiate a new `Module730`
    pub fn prepare(
        average_balance: Decimal,
        ivafe: Decimal,
        gains_and_losses: &GainsAndLosses,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            quadro_rt: QuadroRt::prepare(gains_and_losses),
            quadro_rw: QuadroRw::prepare(average_balance, ivafe),
        })
    }

    /// Output the 730 columns using the provided paginator
    pub fn output(
        &self,
        paginator: impl Paginate,
        gains_and_losses: &GainsAndLosses,
    ) -> anyhow::Result<()> {
        paginator.paginate(self, gains_and_losses)
    }
}
