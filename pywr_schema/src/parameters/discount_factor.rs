use crate::parameters::{ParameterMeta, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct DiscountFactorParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub rate: f64,
    pub base_year: i64,
}

impl DiscountFactorParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}
