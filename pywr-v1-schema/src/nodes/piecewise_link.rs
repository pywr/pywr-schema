use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValueType, ParameterValueTypeMut, ParameterValues};
use pywr_v1_schema_macros::PywrNode;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrNode)]
pub struct PiecewiseLinkNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nsteps: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_flows: Option<ParameterValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costs: Option<ParameterValues>,
}

impl PiecewiseLinkNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
