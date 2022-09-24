use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

use crate::bitpanda::{
    trade::{Asset, AssetClass, CsvOption, Currency, Fiat, InOut, TransactionType},
    Trade,
};

pub struct TradeGenerator;

impl TradeGenerator {
    pub fn deposit(timestamp: DateTime<FixedOffset>, amount: Decimal, fiat: Fiat) -> Trade {
        // fee is 1.8%
        let fee = amount - (amount * dec!(100.0)) / dec!(101.8);
        TradeBuilder::default()
            .timestamp(timestamp)
            .amount_fiat(amount)
            .fiat(fiat)
            .in_out(InOut::Incoming)
            .transaction_type(TransactionType::Deposit)
            .asset(Asset::Currency(Currency::Fiat(fiat)))
            .fee(fee.round_dp(2))
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
            .asset(Asset::Currency(Currency::Fiat(fiat)))
            .into()
    }

    pub fn buy(
        timestamp: DateTime<FixedOffset>,
        amount: Decimal,
        fiat: Fiat,
        amount_asset: Decimal,
        asset: Asset,
        asset_class: AssetClass,
        asset_market_price: Decimal,
    ) -> Trade {
        TradeBuilder::default()
            .timestamp(timestamp)
            .amount_fiat(amount)
            .fiat(fiat)
            .in_out(InOut::Outgoing)
            .transaction_type(TransactionType::Buy)
            .amount_asset(amount_asset)
            .asset(asset)
            .asset_class(asset_class)
            .asset_market_price(asset_market_price)
            .into()
    }

    pub fn sell(
        timestamp: DateTime<FixedOffset>,
        amount: Decimal,
        fiat: Fiat,
        amount_asset: Decimal,
        asset: Asset,
        asset_class: AssetClass,
        asset_market_price: Decimal,
    ) -> Trade {
        TradeBuilder::default()
            .timestamp(timestamp)
            .amount_fiat(amount)
            .fiat(fiat)
            .in_out(InOut::Incoming)
            .transaction_type(TransactionType::Sell)
            .amount_asset(amount_asset)
            .asset(asset)
            .asset_class(asset_class)
            .asset_market_price(asset_market_price)
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

    pub fn amount_asset(mut self, amount: Decimal) -> Self {
        self.amount_asset = CsvOption::Some(amount);
        self
    }

    pub fn asset(mut self, asset: Asset) -> Self {
        self.asset = asset;
        self
    }

    pub fn asset_class(mut self, class: AssetClass) -> Self {
        self.asset_class = class;
        self
    }

    pub fn asset_market_price(mut self, asset_market_price: Decimal) -> Self {
        self.asset_market_price = CsvOption::Some(asset_market_price);
        self
    }
}
