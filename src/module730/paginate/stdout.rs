//! # Stdout
//!
//! This module exposes the stdout paginator for 730

use super::{Module730, Paginate};

/// Stdout paginator
#[derive(Default)]
pub struct Stdout;

impl Paginate for Stdout {
    fn paginate(&self, module: &Module730) -> anyhow::Result<()> {
        todo!()
    }
}
