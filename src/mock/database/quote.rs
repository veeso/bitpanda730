//! # Quote
//!
//! Quote mock

use super::DatabaseTradeMock;
use crate::database::QuoteDatabase;

use chrono::prelude::*;
use chrono::FixedOffset;

pub struct DatabaseQuoteMock;

impl DatabaseQuoteMock {
    pub fn mock() -> QuoteDatabase {
        let db = DatabaseTradeMock::mock();
        QuoteDatabase::load(
            &db,
            FixedOffset::east(3600).ymd(2021, 1, 1).and_hms(0, 0, 0),
            FixedOffset::east(3600)
                .ymd(2021, 12, 31)
                .and_hms(23, 59, 59),
        )
        .unwrap()
    }
}
