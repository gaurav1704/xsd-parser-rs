use num_bigint::{BigUint, ToBigUint};
use std::fmt;
use std::str::FromStr;
use xsd_macro_utils::UtilsDefaultSerde;

// https://www.w3.org/TR/xmlschema-2/#positiveInteger
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsDefaultSerde)]
pub struct PositiveInteger(pub BigUint);

impl PositiveInteger {
    pub fn from_biguint(bigint: BigUint) -> Self {
        PositiveInteger(bigint)
    }
}

impl ToBigUint for PositiveInteger {
    fn to_biguint(&self) -> Option<BigUint> {
        Some(self.0.clone())
    }
}

impl FromStr for PositiveInteger {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = BigUint::from_str(s).map_err(|e| e.to_string())?;
        if value <= 0.to_biguint().unwrap() {
            Err("Bad value for PositiveInteger".to_string())
        } else {
            Ok(PositiveInteger(value))
        }
    }
}

impl fmt::Display for PositiveInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_str_radix(10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;
    use serde::{Deserialize, Serialize};

    #[test]
    fn positive_integer_parse_test() {
        assert_eq!(
            PositiveInteger::from_str("12678967543233"),
            Ok(PositiveInteger(
                BigUint::from_str("12678967543233").unwrap()
            ))
        );

        assert_eq!(
            PositiveInteger::from_str("+100000"),
            Ok(PositiveInteger(100000.to_biguint().unwrap()))
        );

        // Invalid values.
        assert!(PositiveInteger::from_str("0").is_err());
        assert!(PositiveInteger::from_str("+0").is_err());
        assert!(PositiveInteger::from_str("-0").is_err());
        assert!(PositiveInteger::from_str("-1").is_err());
        assert!(PositiveInteger::from_str("-1234").is_err());
        assert!(PositiveInteger::from_str("A").is_err());
        assert!(PositiveInteger::from_str("--1").is_err());
        assert!(PositiveInteger::from_str("++1").is_err());
        assert!(PositiveInteger::from_str("-+1").is_err());
    }

    #[test]
    fn positive_integer_display_test() {
        assert_eq!(
            PositiveInteger(BigUint::from_str("12678967543233").unwrap()).to_string(),
            "12678967543233"
        );

        assert_eq!(
            PositiveInteger(100000.to_biguint().unwrap()).to_string(),
            "100000"
        );
    }

    #[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
    #[serde(prefix = "t", namespace = "t: test")]
    pub struct PositiveIntegerPair {
        #[serde(prefix = "t", rename = "First")]
        pub first: PositiveInteger,

        #[serde(prefix = "t", rename = "Second")]
        pub second: PositiveInteger,
    }

    #[test]
    fn positive_integer_serialize_test() {
        let expected = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:PositiveIntegerPair xmlns:t="test">
                <t:First>1234</t:First>
                <t:Second>1</t:Second>
            </t:PositiveIntegerPair>
            "#;
        let i = PositiveIntegerPair {
            first: PositiveInteger::from_biguint(1234.to_biguint().unwrap()),
            second: PositiveInteger::from_biguint(1.to_biguint().unwrap()),
        };
        let actual = serde::ser::to_string(&i).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn positive_integer_deserialize_test() {
        // Value "+1234" is used to check optional plus sign deserialization.
        let s = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:PositiveIntegerPair xmlns:t="test">
                <t:First>+1234</t:First>
                <t:Second>1</t:Second>
            </t:PositiveIntegerPair>
            "#;
        let i: PositiveIntegerPair = serde::de::from_str(s).unwrap();
        assert_eq!(i.first.to_biguint().unwrap(), 1234.to_biguint().unwrap());
        assert_eq!(i.second.to_biguint().unwrap(), 1.to_biguint().unwrap());
    }
}
