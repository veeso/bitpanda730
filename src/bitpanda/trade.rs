//! # Trade
//!
//! This module defines the trade data type, exposed by the Bitpanda CSV

use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

mod asset_class;
mod currency;
mod fiat;
mod in_out;
mod option;
mod transaction_type;

pub use asset_class::AssetClass;
pub use currency::{CryptoCurrency, Currency};
pub use fiat::Fiat;
pub use in_out::InOut;
pub use option::CsvOption;
pub use transaction_type::TransactionType;

/// Defines a single `Trade` made on Bitpanda exchange
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Trade {
    /// Identity uniquely a transaction on bitpanda
    transaction_id: String,
    /// ISO8601 timestmap of the transaction issuing time
    timestamp: DateTime<FixedOffset>,
    /// Defines the kind of transaction on bitpanda
    transaction_type: TransactionType,
    /// Defines whether the trade assets were given to us or given to bitpanda
    in_out: InOut,
    /// The amount in FIAT currency of the asset
    amount_fiat: Decimal,
    /// The FIAT currency which describes the trade
    fiat: Fiat,
    /// The amount of assets in a Buy/Transfer/Sell
    amount_asset: CsvOption<Decimal>,
    /// The asset name
    asset: String,
    /// The price of the asset in the market. Set only for Buy/Sell
    asset_market_price: CsvOption<Decimal>,
    /// Describes the price currency of the asset market price
    asset_market_price_currency: CsvOption<Fiat>,
    /// Describes the asset kind. Mind that some cryptos are somehow tagged as Fiat (e.g. MATIC, SHIB...)
    asset_class: AssetClass,
    /// Defines uniquely the asset in the bitpanda ecosystem
    product_id: u64,
    /// An amount taken by Bitpanda on a Deposit/Withdrawal operation
    fee: CsvOption<Decimal>,
    /// The currency which describes the fee amount
    fee_asset: CsvOption<Currency>,
    /// Difference between "bid price" and "ask price"
    spread: CsvOption<Decimal>,
    /// The currency which describes the spread amount
    spread_currency: CsvOption<Fiat>,
}
