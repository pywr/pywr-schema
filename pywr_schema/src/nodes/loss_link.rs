use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValue, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct LossLinkNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flow: Option<ParameterValue>,
    pub min_flow: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
    pub loss_factor: Option<ParameterValue>,
}

impl LossLinkNode {
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
        if let Some(p) = &self.loss_factor {
            attributes.insert("loss_factor", ParameterValueType::Single(p));
        }

        attributes
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
