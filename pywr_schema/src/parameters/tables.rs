use crate::parameters::{ParameterMeta, ParameterValueType};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TablesArrayParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    #[serde(rename = "where")]
    pub wh: String,
    pub scenario: Option<String>,
    pub checksum: Option<HashMap<String, String>>,
    pub url: PathBuf,
}

impl TablesArrayParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}
