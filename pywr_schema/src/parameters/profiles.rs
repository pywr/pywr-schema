use crate::parameters::{ExternalDataRef, ParameterMeta, ParameterValueType, TableDataRef};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
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
    pub fn resource_paths(&self) -> Vec<PathBuf> {
        let mut resource_paths = Vec::new();
        if let Some(external) = &self.external {
            resource_paths.push(external.url.clone());
        }
        resource_paths
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MonthInterpDay {
    First,
    Last,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
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
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
    pub fn resource_paths(&self) -> Vec<PathBuf> {
        let mut resource_paths = Vec::new();
        if let Some(external) = &self.external {
            resource_paths.push(external.url.clone());
        }
        resource_paths
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
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
