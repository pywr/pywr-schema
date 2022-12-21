use crate::nodes::NodeMeta;
use crate::parameters::ParameterValue;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RiverSplitWithGaugeNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub mrf: Option<ParameterValue>,
    pub mrf_cost: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
    pub factors: Vec<ParameterValue>,
    pub slot_names: Vec<String>,
}

impl RiverSplitWithGaugeNode {
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.mrf {
            attributes.insert("mrf", p);
        }
        if let Some(p) = &self.mrf_cost {
            attributes.insert("mrf_cost", p);
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", p);
        }
        attributes
    }
}
