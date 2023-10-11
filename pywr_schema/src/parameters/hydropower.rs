use crate::parameters::{ParameterMeta, ParameterValue, ParameterValueType, ParameterValueTypeMut};
use pywr_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
pub struct HydropowerTargetParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub target: ParameterValue,
    pub water_elevation_parameter: Option<ParameterValue>,
    pub max_flow: Option<ParameterValue>,
    pub min_flow: Option<ParameterValue>,
    pub turbine_elevation: Option<f64>,
    pub efficiency: Option<f64>,
    pub density: Option<f64>,
    pub min_head: Option<f64>,
    pub flow_unit_conversion: Option<f64>,
    pub energy_unit_conversion: Option<f64>,
}

impl HydropowerTargetParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
