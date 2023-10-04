use crate::parameters::{ParameterMeta, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct RollingMeanFlowNodeParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    pub timesteps: Option<i64>,
    pub days: Option<i64>,
    pub initial_flow: Option<f64>,
}

impl RollingMeanFlowNodeParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("node", self.node.as_str())].into_iter().collect()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}
