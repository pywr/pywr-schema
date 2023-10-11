use crate::parameters::{
    ExternalDataRef, ParameterMeta, ParameterValue, ParameterValueType, ParameterValueTypeMut,
    TableDataRef,
};
use pywr_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
pub struct ConstantParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    #[serde(alias = "values")]
    pub value: Option<f64>,
    #[serde(flatten)]
    pub external: Option<ExternalDataRef>,
    #[serde(flatten)]
    pub table: Option<TableDataRef>,
}

impl ConstantParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
pub struct MaxParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
    pub threshold: Option<f64>,
}

impl MaxParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
pub struct NegativeParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
}

impl NegativeParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
pub struct MinParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
    pub threshold: Option<f64>,
}

impl MinParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
pub struct DivisionParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub numerator: ParameterValue,
    pub denominator: ParameterValue,
}

impl DivisionParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
