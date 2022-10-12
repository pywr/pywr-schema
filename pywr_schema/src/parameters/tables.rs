use crate::parameters::{NodeReference, ParameterMeta, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
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
    pub fn node_references(&self) -> Vec<NodeReference> {
        vec![]
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}
