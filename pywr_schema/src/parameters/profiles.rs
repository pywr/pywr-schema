use crate::parameters::{
    ExternalDataRef, ParameterMeta, ParameterValueType, ParameterValueTypeMut, TableDataRef,
};
use pywr_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
pub struct DailyProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalDataRef>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub table_ref: Option<TableDataRef>,
}

impl DailyProfileParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MonthInterpDay {
    First,
    Last,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
pub struct MonthlyProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub interp_day: Option<MonthInterpDay>,
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
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
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
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
pub struct WeeklyProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub values: Option<Vec<f64>>,
}

impl WeeklyProfileParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
