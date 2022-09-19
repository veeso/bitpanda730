use crate::{bitpanda::Trade, database::TradeDatabase};

pub struct DatabaseTradeMock;

impl DatabaseTradeMock {
    pub fn get() -> TradeDatabase {
        TradeDatabase::from(vec![])
    }
}
