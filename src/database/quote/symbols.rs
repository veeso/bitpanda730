//! # Symbols
//!
//! This module provides lookup for the yahoo api queries

use crate::bitpanda::trade::Asset;

/// Symbols lookup resolver
pub struct Symbols;

impl Symbols {
    pub fn lookup(asset: Asset) -> String {
        todo!("lookup")
    }
}
