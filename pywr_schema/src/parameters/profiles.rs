use crate::parameters::{ExternalDataRef, ParameterMeta, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DailyProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub values: Option<Vec<f64>>,
    #[serde(flatten)]
    pub external: Option<ExternalDataRef>,
}

impl DailyProfileParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct MonthlyProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub values: Option<[f64; 12]>,
    #[serde(flatten)]
    pub external: Option<ExternalDataRef>,
}

impl MonthlyProfileParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}
