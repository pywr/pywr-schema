use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValue, ParameterValueType, ParameterValues};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct RiverSplitWithGaugeNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub mrf: Option<ParameterValue>,
    pub mrf_cost: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
    pub factors: ParameterValues,
    pub slot_names: Vec<String>,
}

impl RiverSplitWithGaugeNode {
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.mrf {
            attributes.insert("mrf", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.mrf_cost {
            attributes.insert("mrf_cost", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", ParameterValueType::Single(p));
        }
        let factors = &self.factors;
        attributes.insert("factors", factors.into());
        attributes
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
