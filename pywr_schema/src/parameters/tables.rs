use crate::parameters::{ParameterMeta, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TablesArrayParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    #[serde(rename = "where")]
    pub wh: Option<String>,
    pub scenario: Option<String>,
    pub checksum: Option<HashMap<String, String>>,
    pub url: String,
}

impl TablesArrayParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}
