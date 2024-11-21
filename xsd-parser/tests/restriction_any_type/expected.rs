// pub type AppSequence = AppSequenceType;
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://schemas.xmlsoap.org/ws/2005/04/discovery")]
pub struct AppSequenceType {
    #[serde(rename = "InstanceId")]
    pub instance_id: u32,

    #[serde(rename = "SequenceId")]
    pub sequence_id: Option<String>,

    #[serde(rename = "MessageNumber")]
    pub message_number: u32,
}

impl Validate for AppSequenceType {}
