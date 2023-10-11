use crate::parameters::{ParameterMeta, ParameterValueType, ParameterValueTypeMut};
use pywr_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
pub struct Polynomial1DParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub storage_node: String,
    pub coefficients: Vec<f64>,
    pub use_proportional_volume: Option<bool>,
    pub scale: Option<f64>,
    pub offset: Option<f64>,
}

impl Polynomial1DParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("storage_node", self.storage_node.as_str())]
            .into_iter()
            .collect()
    }
}
