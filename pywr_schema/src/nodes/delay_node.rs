use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValueType, ParameterValueTypeMut};
use pywr_schema_macros::PywrNode;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrNode)]
pub struct DelayNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub days: Option<i64>,
    pub timesteps: Option<i64>,
    pub initial_flow: Option<f64>,
}

impl DelayNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
