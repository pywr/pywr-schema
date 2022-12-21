use crate::nodes::NodeMeta;
use crate::parameters::ParameterValue;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RiverGaugeNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub mrf: Option<ParameterValue>,
    pub mrf_cost: Option<ParameterValue>,
}

impl RiverGaugeNode {
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.mrf {
            attributes.insert("mrf", p);
        }
        if let Some(p) = &self.mrf_cost {
            attributes.insert("mrf_cost", p);
        }

        attributes
    }
}
