//! # Transaction type
//!
//! Transaction type definition

/// Defines the `TransactionType` in the bitpanda trade
#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    /// Currency deposited to bitpanda
    Deposit,
    /// The buy operation of an asset
    Buy,
    /// An asset transferred from Bitpanda to your wallet (e.g. BEST rewards, staking rewards, ...)
    Transfer,
    /// A sell operation of an asset
    Sell,
    /// A withdrawal of a currency (NOTE: can be FIAT or Crypto).
    /// A bitpanda Card transaction is a Withdrawal too.
    Withdrawal,
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    #[test]
    fn should_decode_transaction_type() {
        let csv = r#"id,transaction_type
0,deposit
1,buy
2,transfer
3,sell
4,withdrawal
"#;
        let buffer = Cursor::new(csv);
        let mut reader = csv::Reader::from_reader(buffer);
        let mut fakes: Vec<TransactionType> = Vec::new();
        for result in reader.deserialize::<Fake>() {
            fakes.push(result.expect("failed to decode").transaction_type);
        }
        assert_eq!(
            fakes,
            vec![
                TransactionType::Deposit,
                TransactionType::Buy,
                TransactionType::Transfer,
                TransactionType::Sell,
                TransactionType::Withdrawal,
            ]
        );
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Fake {
        id: u64,
        transaction_type: TransactionType,
    }
}
