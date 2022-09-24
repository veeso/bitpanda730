//! # Tax
//!
//! This module expose the tax calculators for Italian taxation ruleset

use crate::database::TradeDatabase;

pub struct Taxes<'a> {
    trades: &'a TradeDatabase,
}

impl<'a> From<&'a TradeDatabase> for Taxes<'a> {
    fn from(trades: &'a TradeDatabase) -> Self {
        Self { trades }
    }
}

impl<'a> Taxes<'a> {}

#[cfg(test)]
mod test {

    use super::*;

    use crate::mock::database::DatabaseTradeMock;

    #[test]
    fn should_init_taxes() {
        let db = DatabaseTradeMock::mock();
        let _ = Taxes::from(&db);
    }
}
