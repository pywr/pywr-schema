use crate::parameters::{ParameterMeta, ParameterValues, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct InterpolatedVolumeParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    pub volumes: ParameterValues, 
    pub values: ParameterValues,  
    pub interp_kwargs: Option<HashMap<String, serde_json::Value>>,
}

impl InterpolatedVolumeParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("node", self.node.as_str())].into_iter().collect()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct InterpolatedFlowParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    pub flows: Vec<f64>, // TODO this also supports loading data from tables, etc.
    pub values: Vec<f64>, // TODO this also supports loading data from tables, etc.
    pub interp_kwargs: Option<HashMap<String, serde_json::Value>>,
}

impl InterpolatedFlowParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("node", self.node.as_str())].into_iter().collect()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}
