use crate::nodes::NodeMeta;
use crate::parameters::{
    ParameterValue, ParameterValueType, ParameterValueTypeMut, ParameterValues,
};
use pywr_schema_macros::PywrNode;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrNode)]
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
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
