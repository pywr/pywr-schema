use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValueType, ParameterValueTypeMut, ParameterValues};
use pywr_schema_macros::PywrNode;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrNode)]
pub struct PiecewiseLinkNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nsteps: usize,
    pub max_flows: Option<ParameterValues>,
    pub costs: Option<ParameterValues>,
}

impl PiecewiseLinkNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
