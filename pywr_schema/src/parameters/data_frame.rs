use crate::parameters::{ParameterMeta, ParameterValueType};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct DataFrameParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub scenario: Option<String>,
    pub timestep_offset: Option<i32>,
    pub column: Option<String>,
    pub index: Option<String>,
    pub indexes: Option<String>,
    pub table: Option<String>,
    pub url: Option<PathBuf>,
    #[serde(flatten)]
    pub pandas_kwargs: HashMap<String, serde_json::Value>,
}

impl DataFrameParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
    pub fn resource_paths(&self) -> Vec<PathBuf> {
        let mut resource_paths = Vec::new();
        if let Some(url) = &self.url {
            resource_paths.push(url.clone());
        }
        resource_paths
    }
}
