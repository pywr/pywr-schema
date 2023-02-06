use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValue, ParameterValues};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PiecewiseLinkNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flows: Option<ParameterValues>,
    pub costs: Option<ParameterValues>,
}

impl PiecewiseLinkNode {
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        HashMap::new()
    }
}
