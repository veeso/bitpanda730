use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::bitpanda::{
    trade::{Asset, AssetClass, CryptoCurrency, CsvOption, Currency, Fiat, InOut, TransactionType},
    Trade,
};

pub struct TradeGenerator;

impl TradeGenerator {
    pub fn deposit(timestamp: DateTime<FixedOffset>, amount: Decimal, fiat: Fiat) -> Trade {
        // fee is 1.8%
        let fee = (amount * dec!(1.8)) / dec!(100.0);
        TradeBuilder::default()
            .timestamp(timestamp)
            .amount_fiat(amount)
            .fiat(fiat)
            .in_out(InOut::Incoming)
            .transaction_type(TransactionType::Deposit)
            .fee(fee)
            .fee_asset(Currency::Fiat(fiat))
            .into()
    }

    pub fn withdrawal(timestamp: DateTime<FixedOffset>, amount: Decimal, fiat: Fiat) -> Trade {
        TradeBuilder::default()
            .timestamp(timestamp)
            .amount_fiat(amount)
            .fiat(fiat)
            .in_out(InOut::Outgoing)
            .transaction_type(TransactionType::Withdrawal)
            .into()
    }
}

#[derive(Debug)]
pub struct TradeBuilder {
    pub transaction_id: String,
    pub timestamp: DateTime<FixedOffset>,
    pub transaction_type: TransactionType,
    pub in_out: InOut,
    pub amount_fiat: Decimal,
    pub fiat: Fiat,
    pub amount_asset: CsvOption<Decimal>,
    pub asset: Asset,
    pub asset_market_price: CsvOption<Decimal>,
    pub asset_market_price_currency: CsvOption<Fiat>,
    pub asset_class: AssetClass,
    pub product_id: CsvOption<u64>,
    pub fee: CsvOption<Decimal>,
    pub fee_asset: CsvOption<Currency>,
    pub spread: CsvOption<Decimal>,
    pub spread_currency: CsvOption<Fiat>,
}

impl Default for TradeBuilder {
    fn default() -> Self {
        Self {
            transaction_id: uuid::Uuid::new_v4().to_string(),
            timestamp: DateTime::default(),
            transaction_type: TransactionType::Buy,
            in_out: InOut::Incoming,
            amount_fiat: dec!(0.0),
            fiat: Fiat::Eur,
            amount_asset: CsvOption::none(),
            asset: Asset::Name("AMZN".to_string()),
            asset_market_price: CsvOption::none(),
            asset_market_price_currency: CsvOption::none(),
            asset_class: AssetClass::Stock,
            product_id: CsvOption::none(),
            fee: CsvOption::none(),
            fee_asset: CsvOption::none(),
            spread: CsvOption::none(),
            spread_currency: CsvOption::none(),
        }
    }
}

impl TradeBuilder {
    pub fn timestamp(mut self, timestamp: DateTime<FixedOffset>) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn amount_fiat(mut self, amount: Decimal) -> Self {
        self.amount_fiat = amount;
        self
    }

    pub fn fiat(mut self, fiat: Fiat) -> Self {
        self.fiat = fiat;
        self
    }

    pub fn in_out(mut self, in_out: InOut) -> Self {
        self.in_out = in_out;
        self
    }

    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = transaction_type;
        self
    }

    pub fn fee(mut self, fee: Decimal) -> Self {
        self.fee = CsvOption::Some(fee);
        self
    }

    pub fn fee_asset(mut self, currency: Currency) -> Self {
        self.fee_asset = CsvOption::Some(currency);
        self
    }
}
