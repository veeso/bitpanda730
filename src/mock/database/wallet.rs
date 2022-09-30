//! # Wallet
//!
//! Wallet mock

use super::DatabaseTradeMock;
use crate::database::WalletDatabase;

pub struct DatabaseWalletMock;

impl DatabaseWalletMock {
    pub fn mock() -> WalletDatabase {
        let db = DatabaseTradeMock::mock();
        WalletDatabase::load(db.all())
    }
}
