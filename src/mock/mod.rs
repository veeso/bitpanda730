//! # Mock
//!
//! This module contains mocked data

pub mod database;

#[cfg(test)]
pub fn log() {
    let _ = env_logger::builder().is_test(true).try_init();
}
