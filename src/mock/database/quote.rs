//! # Quote
//!
//! Quote mock

use super::DatabaseTradeMock;
use crate::database::QuoteDatabase;

use chrono::prelude::*;
use chrono::{NaiveDate, Utc};

pub struct DatabaseQuoteMock;

impl DatabaseQuoteMock {
    pub fn mock() -> QuoteDatabase {
        let db = DatabaseTradeMock::mock();
        QuoteDatabase::load(
            &db,
            Utc.from_utc_datetime(&NaiveDate::from_ymd(2021, 1, 1).and_hms(0, 0, 0)),
            Utc.from_utc_datetime(&NaiveDate::from_ymd(2021, 12, 31).and_hms(23, 59, 59)),
        )
        .unwrap()
    }
}
