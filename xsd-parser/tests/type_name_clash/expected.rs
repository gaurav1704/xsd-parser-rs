#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct BarType {
    #[serde(rename = "a")]
    pub a: Option<String>,
}

impl Validate for BarType {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[serde(prefix = "tns", rename = "Bar")]
    pub bar: foo_type::BarType,
}

impl Validate for FooType {}

pub mod foo_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
    #[serde(prefix = "tns", namespace = "tns: http://example.com")]
    pub struct BarType {
        #[serde(rename = "b")]
        pub b: Option<String>,

        #[serde(rename = "a")]
        pub a: Option<String>,

    }

    impl Validate for BarType {}
}
