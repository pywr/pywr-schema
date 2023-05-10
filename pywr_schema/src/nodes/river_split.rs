use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValueType, ParameterValues};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct RiverSplitNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flows: Option<ParameterValues>,
    pub costs: Option<ParameterValues>,
    pub slot_names: Option<Vec<String>>,
    pub factors: ParameterValues,
}

impl RiverSplitNode {
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(max_flows) = &self.max_flows {
            attributes.insert("max_flows", max_flows.into());
        }
        if let Some(costs) = &self.costs {
            attributes.insert("costs", costs.into());
        }
        let factors = &self.factors;
        attributes.insert("factors", factors.into());

        attributes
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
