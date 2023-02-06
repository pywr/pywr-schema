use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValue, ParameterValues};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RiverSplitNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flows: Option<ParameterValues>,
    pub costs: Option<ParameterValues>,
    pub extra_slots: Option<i64>,
    pub slot_names: Option<Vec<String>>,
    pub factors: Option<Vec<f64>>
}

impl RiverSplitNode {
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        HashMap::new()
    }
}