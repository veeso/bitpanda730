//! # Module 730
//!
//! This module exposes utilities to compile the 730 module

use rust_decimal::Decimal;

use crate::tax::GainsAndLosses;

mod paginate;
mod quadro_rw;
pub use paginate::{Paginate, Stdout};
use quadro_rw::QuadroRw;

/// Module 730 data
pub struct Module730 {
    ivafe: Decimal,
    gains_and_losses: GainsAndLosses,
    quadro_rw: QuadroRw,
}

impl Module730 {
    /// Instantiate a new `Module730`
    pub fn prepare(ivafe: Decimal, gains_and_losses: GainsAndLosses) -> anyhow::Result<Self> {
        todo!();
        Ok(Self {
            ivafe,
            gains_and_losses,
            quadro_rw: QuadroRw::prepare(ivafe),
        })
    }

    /// Output the 730 columns using the provided paginator
    pub fn output(&self, paginator: impl Paginate) -> anyhow::Result<()> {
        paginator.paginate(self)
    }
}
