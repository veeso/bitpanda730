//! # Trade
//!
//! This module defines the trade data type, exposed by the Bitpanda CSV

use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

mod asset;
mod asset_class;
mod currency;
mod fiat;
mod in_out;
mod option;
mod transaction_type;

pub use asset::Asset;
pub use asset_class::AssetClass;
pub use currency::{CryptoCurrency, Currency};
pub use fiat::Fiat;
pub use in_out::InOut;
pub use option::CsvOption;
pub use transaction_type::TransactionType;

/// Defines a single `Trade` made on Bitpanda exchange
#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Trade {
    /// Identity uniquely a transaction on bitpanda
    #[serde(rename = "Transaction ID")]
    transaction_id: String,
    /// ISO8601 timestmap of the transaction issuing time
    #[serde(rename = "Timestamp")]
    timestamp: DateTime<FixedOffset>,
    /// Defines the kind of transaction on bitpanda
    #[serde(rename = "Transaction Type")]
    transaction_type: TransactionType,
    /// Defines whether the trade assets were given to us or given to bitpanda
    #[serde(rename = "In/Out")]
    in_out: InOut,
    /// The amount in FIAT currency of the asset
    #[serde(rename = "Amount Fiat")]
    amount_fiat: Decimal,
    /// The FIAT currency which describes the trade
    #[serde(rename = "Fiat")]
    fiat: Fiat,
    /// The amount of assets in a Buy/Transfer/Sell
    #[serde(rename = "Amount Asset")]
    amount_asset: CsvOption<Decimal>,
    /// The asset name
    #[serde(rename = "Asset")]
    asset: Asset,
    /// The price of the asset in the market. Set only for Buy/Sell
    #[serde(rename = "Asset market price")]
    asset_market_price: CsvOption<Decimal>,
    /// Describes the price currency of the asset market price
    #[serde(rename = "Asset market price currency")]
    asset_market_price_currency: CsvOption<Fiat>,
    /// Describes the asset kind. Mind that some cryptos are somehow tagged as Fiat (e.g. MATIC, SHIB...)
    #[serde(rename = "Asset class")]
    asset_class: AssetClass,
    /// Defines uniquely the asset in the bitpanda ecosystem
    #[serde(rename = "Product ID")]
    product_id: CsvOption<u64>,
    /// An amount taken by Bitpanda on a Deposit/Withdrawal operation
    #[serde(rename = "Fee")]
    fee: CsvOption<Decimal>,
    /// The currency which describes the fee amount
    #[serde(rename = "Fee asset")]
    fee_asset: CsvOption<Currency>,
    /// Difference between "bid price" and "ask price"
    #[serde(rename = "Spread")]
    spread: CsvOption<Decimal>,
    /// The currency which describes the spread amount
    #[serde(rename = "Spread Currency")]
    spread_currency: CsvOption<Fiat>,
}

impl Trade {
    pub fn transaction_id(&self) -> &str {
        &self.transaction_id
    }

    pub fn timestamp(&self) -> DateTime<FixedOffset> {
        self.timestamp
    }

    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type
    }

    pub fn in_out(&self) -> InOut {
        self.in_out
    }

    pub fn amount_fiat(&self) -> Decimal {
        self.amount_fiat
    }

    pub fn fiat(&self) -> Fiat {
        self.fiat
    }

    pub fn amount_asset(&self) -> Option<Decimal> {
        self.amount_asset.into()
    }

    pub fn asset(&self) -> Asset {
        self.asset.clone()
    }

    pub fn asset_market_price(&self) -> Option<Decimal> {
        self.asset_market_price.into()
    }

    pub fn asset_market_price_currency(&self) -> Option<Fiat> {
        self.asset_market_price_currency.into()
    }

    pub fn asset_class(&self) -> AssetClass {
        self.asset_class
    }

    pub fn product_id(&self) -> Option<u64> {
        self.product_id.into()
    }

    pub fn fee(&self) -> Option<Decimal> {
        self.fee.into()
    }

    pub fn fee_asset(&self) -> Option<Currency> {
        self.fee_asset.into()
    }

    pub fn spread(&self) -> Option<Decimal> {
        self.spread.into()
    }

    pub fn spread_currency(&self) -> Option<Fiat> {
        self.spread_currency.into()
    }
}

#[cfg(test)]
impl From<crate::mock::bitpanda::TradeBuilder> for Trade {
    fn from(builder: crate::mock::bitpanda::TradeBuilder) -> Self {
        Self {
            transaction_id: builder.transaction_id,
            timestamp: builder.timestamp,
            transaction_type: builder.transaction_type,
            in_out: builder.in_out,
            amount_fiat: builder.amount_fiat,
            fiat: builder.fiat,
            amount_asset: builder.amount_asset,
            asset: builder.asset,
            asset_market_price: builder.asset_market_price,
            asset_market_price_currency: builder.asset_market_price_currency,
            asset_class: builder.asset_class,
            product_id: builder.product_id,
            fee: builder.fee,
            fee_asset: builder.fee_asset,
            spread: builder.spread,
            spread_currency: builder.spread_currency,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;
    use std::io::Cursor;

    #[test]
    fn should_decode_trade() {
        let csv = r#""Transaction ID",Timestamp,"Transaction Type",In/Out,"Amount Fiat",Fiat,"Amount Asset",Asset,"Asset market price","Asset market price currency","Asset class","Product ID",Fee,"Fee asset",Spread,"Spread Currency"
F48a0adaa-824f-4753-8e2e-***********,2022-07-02T08:53:13+02:00,deposit,incoming,1000.00,EUR,-,EUR,-,-,Fiat,-,17.69000000,EUR,-,-
T02f7a7ce-9c38-4b18-9306-***********,2022-07-02T09:09:41+02:00,buy,outgoing,150.00,EUR,1.42307692,AMZN,105.41,EUR,"Stock (derivative)",73,-,-,0.15000000,EUR
T8123f94c-2580-4129-ae62-***********,2022-07-02T09:19:36+02:00,buy,outgoing,250.00,EUR,0.01329013,BTC,18810.95,EUR,Cryptocurrency,1,-,-,-,-
2cbcc5dd-67c1-4ded-8020-6***********,2022-07-02T09:23:35+02:00,transfer,incoming,0.00,EUR,0.00869699,BEST,0.34,EUR,Cryptocurrency,33,-,-,-,-
F9d880b45-e2bf-4a72-b39a-***********,2022-07-02T09:33:29+02:00,deposit,incoming,500.00,EUR,-,EUR,-,-,Fiat,-,8.85000000,EUR,-,-
F04ce50ab-80e9-4bde-bc74-***********,2022-07-04T11:34:39+02:00,deposit,incoming,500.00,EUR,-,EUR,-,-,Fiat,-,8.85000000,EUR,-,-
F9b623f2d-4432-445b-90a7-***********,2022-07-28T19:27:49+02:00,deposit,incoming,1527.00,EUR,-,EUR,-,-,Fiat,-,27.00000000,EUR,-,-
C04e9125e-9688-4fbb-b23b-***********,2022-08-04T15:16:04+02:00,withdrawal,outgoing,0,EUR,0.34905088,ETH,0.00,-,Cryptocurrency,5,0.00100136,ETH,-,-
Cd0386774-b60a-4f60-bc1e-***********,2022-08-04T15:17:46+02:00,withdrawal,outgoing,0,EUR,0.05039663,BTC,0.00,-,Cryptocurrency,1,0.00006000,BTC,-,-
T2fdbfca0-fc44-4032-941e-***********,2022-08-05T14:35:07+02:00,sell,incoming,129.17,EUR,15.00000000,FTSE100,8.61,EUR,"ETF (derivative)",115,-,-,0.33000000,EUR
F93590637-4ca5-4edf-af9f-***********,2022-08-13T10:28:48+02:00,withdrawal,outgoing,20.00,EUR,-,EUR,-,-,Fiat,-,0.00000000,EUR,-,-
F542dc58a-c88e-45d5-9f00-***********,2022-08-24T01:32:08+02:00,withdrawal,outgoing,1197.70,EUR,-,EUR,-,-,Fiat,-,0.00000000,EUR,-,-
"#;
        let buffer = Cursor::new(csv);
        let mut reader = csv::Reader::from_reader(buffer);
        let mut trades: Vec<Trade> = Vec::new();
        for result in reader.deserialize::<Trade>() {
            trades.push(result.expect("failed to decode row"));
        }
        assert_eq!(trades.len(), 12);
        let trade0 = &trades[0];
        assert_eq!(
            trade0.transaction_id(),
            "F48a0adaa-824f-4753-8e2e-***********"
        );
        assert_eq!(
            trade0.timestamp().to_rfc3339().as_str(),
            "2022-07-02T08:53:13+02:00"
        );
        assert_eq!(trade0.transaction_type(), TransactionType::Deposit);
        assert_eq!(trade0.in_out(), InOut::Incoming);
        assert_eq!(trade0.amount_fiat(), dec!(1000.0));
        assert_eq!(trade0.fiat(), Fiat::Eur);
        assert_eq!(trade0.amount_asset(), None);
        assert_eq!(trade0.asset(), Asset::Currency(Currency::Fiat(Fiat::Eur)));
        assert_eq!(trade0.asset_market_price(), None);
        assert_eq!(trade0.asset_market_price_currency(), None);
        assert_eq!(trade0.asset_class(), AssetClass::Fiat);
        assert_eq!(trade0.product_id(), None);
        assert_eq!(trade0.fee(), Some(dec!(17.69000000)));
        assert_eq!(trade0.fee_asset(), Some(Currency::Fiat(Fiat::Eur)));
        assert_eq!(trade0.spread(), None);
        assert_eq!(trade0.spread_currency(), None);
    }
}
