//! # Quote
//!
//! Quote mock

use super::DatabaseTradeMock;
use crate::database::QuoteDatabase;

use chrono::prelude::*;
use chrono::FixedOffset;

pub struct DatabaseQuoteMock;

impl DatabaseQuoteMock {
    pub async fn mock() -> QuoteDatabase {
        let db = DatabaseTradeMock::mock();
        QuoteDatabase::load(
            &db,
            FixedOffset::east_opt(3600)
                .unwrap()
                .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
                .unwrap(),
            FixedOffset::east_opt(3600)
                .unwrap()
                .with_ymd_and_hms(2022, 12, 31, 23, 59, 59)
                .unwrap(),
        )
        .await
        .unwrap()
    }
}
