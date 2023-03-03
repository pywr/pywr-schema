use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValue, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DelayNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flow: Option<ParameterValue>,
    pub min_flow: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
    pub days: Option<i64>,
    pub timesteps: Option<i64>,
    pub initial_flow: Option<i64>,
}

impl DelayNode {
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.max_flow {
            attributes.insert("max_flow", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.min_flow {
            attributes.insert("min_flow", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", ParameterValueType::Single(p));
        }
        attributes
    }
}
