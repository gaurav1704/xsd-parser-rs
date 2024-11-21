use bigdecimal::{BigDecimal, ParseBigDecimalError};
use std::fmt;
use std::str::FromStr;
use xsd_macro_utils::UtilsDefaultSerde;

#[derive(Default, PartialEq, PartialOrd, Debug, UtilsDefaultSerde)]
pub struct Decimal(pub BigDecimal);

impl Decimal {
    pub fn from_bigdecimal(bigdecimal: BigDecimal) -> Self {
        Decimal(bigdecimal)
    }

    pub fn to_bigdecimal(&self) -> BigDecimal {
        self.0.clone()
    }
}

impl FromStr for Decimal {
    type Err = ParseBigDecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Decimal(BigDecimal::from_str(s)?))
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;
    use num_bigint::ToBigInt;
    use serde::{Deserialize, Serialize};

    #[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
    #[serde(prefix = "t", namespace = "t: test")]
    pub struct DecimalPair {
        #[serde(prefix = "t", rename = "First")]
        pub first: Decimal,

        #[serde(prefix = "t", rename = "Second")]
        pub second: Decimal,
    }

    #[test]
    fn integer_serialize_test() {
        let expected = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:DecimalPair xmlns:t="test">
                <t:First>0.01234</t:First>
                <t:Second>-12.34</t:Second>
            </t:DecimalPair>
            "#;
        let i = DecimalPair {
            first: Decimal::from_bigdecimal(BigDecimal::new(1234.to_bigint().unwrap(), 5)),
            second: Decimal::from_bigdecimal(BigDecimal::new((-1234).to_bigint().unwrap(), 2)),
        };
        let actual = serde::ser::to_string(&i).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn integer_deserialize_test() {
        // Value "+0.01234" is used to check optional plus sign deserialization.
        let s = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:DecimalPair xmlns:t="test">
                <t:First>+0.01234</t:First>
                <t:Second>-12.34</t:Second>
            </t:DecimalPair>
            "#;
        let i: DecimalPair = serde::de::from_str(s).unwrap();
        assert_eq!(
            i.first.to_bigdecimal(),
            BigDecimal::new(1234.to_bigint().unwrap(), 5)
        );
        assert_eq!(
            i.second.to_bigdecimal(),
            BigDecimal::new((-1234).to_bigint().unwrap(), 2)
        );
    }
}
