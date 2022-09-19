//! # InOut
//!
//! The direction of a transaction

/// Defines the direction of a trade on bitpanda
#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum InOut {
    Incoming,
    Outgoing,
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    #[test]
    fn should_decode_in_out() {
        let csv = r#"id,in_out
0,incoming
1,outgoing
"#;
        let buffer = Cursor::new(csv);
        let mut reader = csv::Reader::from_reader(buffer);
        let mut fakes: Vec<InOut> = Vec::new();
        for result in reader.deserialize::<Fake>() {
            fakes.push(result.expect("failed to decode").in_out);
        }
        assert_eq!(fakes, vec![InOut::Incoming, InOut::Outgoing,]);
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Fake {
        id: u64,
        in_out: InOut,
    }
}
