use crate::parameters::{ExternalDataRef, ParameterMeta, ParameterValueType, TableDataRef};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DailyProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub values: Option<Vec<f64>>,
    #[serde(flatten)]
    pub external: Option<ExternalDataRef>,
    #[serde(flatten)]
    pub table_ref: Option<TableDataRef>,
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
    #[serde(flatten)]
    pub table_ref: Option<TableDataRef>,
}

impl MonthlyProfileParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct UniformDrawdownProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub reset_day: Option<u8>,
    pub reset_month: Option<u8>,
    pub residual_days: Option<u32>,
}

impl UniformDrawdownProfileParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}
