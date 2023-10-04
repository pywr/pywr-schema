use crate::parameters::{ParameterMeta, ParameterValue, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
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
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes: HashMap<&str, ParameterValueType> = HashMap::new();
        attributes.insert("target", (&self.target).into());

        if let Some(p) = &self.water_elevation_parameter {
            attributes.insert("water_elevation_parameter", p.into());
        }
        if let Some(p) = &self.max_flow {
            attributes.insert("max_flow", p.into());
        }
        if let Some(p) = &self.min_flow {
            attributes.insert("min_flow", p.into());
        }
        attributes
    }
}
