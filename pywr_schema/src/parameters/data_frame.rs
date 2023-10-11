use crate::parameters::{ParameterMeta, ParameterValueType, ParameterValueTypeMut};
use pywr_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
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
}
