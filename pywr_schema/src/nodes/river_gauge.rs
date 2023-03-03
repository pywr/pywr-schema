use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValue, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RiverGaugeNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub mrf: Option<ParameterValue>,
    pub mrf_cost: Option<ParameterValue>,
}

impl RiverGaugeNode {
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.mrf {
            attributes.insert("mrf", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.mrf_cost {
            attributes.insert("mrf_cost", ParameterValueType::Single(p));
        }

        attributes
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
