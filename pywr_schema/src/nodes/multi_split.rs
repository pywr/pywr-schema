use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValueType, ParameterValueTypeMut, ParameterValues};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct MultiSplitLinkNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flows: Option<ParameterValues>,
    pub costs: Option<ParameterValues>,
    pub extra_slots: Option<i64>,
    pub slot_names: Option<Vec<String>>,
    pub factors: Option<ParameterValues>,
}

impl MultiSplitLinkNode {
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(max_flows) = &self.max_flows {
            attributes.insert("max_flows", max_flows.into());
        }
        if let Some(costs) = &self.costs {
            attributes.insert("costs", costs.into());
        }
        if let Some(factors) = &self.factors {
            attributes.insert("factors", factors.into());
        }
        attributes
    }

    pub fn parameters_mut(&mut self) -> HashMap<&str, ParameterValueTypeMut> {
        let mut attributes = HashMap::new();
        if let Some(max_flows) = &mut self.max_flows {
            attributes.insert("max_flows", max_flows.into());
        }
        if let Some(costs) = &mut self.costs {
            attributes.insert("costs", costs.into());
        }
        if let Some(factors) = &mut self.factors {
            attributes.insert("factors", factors.into());
        }
        attributes
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
