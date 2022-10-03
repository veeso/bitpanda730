//! # Paginate
//!
//! Paginate provides a trait and types to paginate the 730 data

use super::Module730;

mod stdout;

pub use stdout::Stdout;

pub trait Paginate {
    /// Paginate module 730 to some kind of output
    fn paginate(&self, module: &Module730) -> anyhow::Result<()>;
}
