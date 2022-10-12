use crate::parameters::{NodeReference, ParameterMeta, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Polynomial1DParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub storage_node: String,
    pub coefficients: Vec<f64>,
    pub use_proportional_volume: Option<bool>,
}

impl Polynomial1DParameter {
    pub fn node_references(&self) -> Vec<NodeReference> {
        vec![NodeReference {
            attribute: "storage_node".to_string(),
            node: self.storage_node.clone(),
        }]
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}
