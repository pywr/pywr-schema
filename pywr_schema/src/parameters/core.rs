use crate::parameters::{ExternalDataRef, ParameterMeta, ParameterValue, ParameterValueType, TableDataRef};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
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
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        attributes.insert("parameter", (&self.parameter).into());
        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct NegativeParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
}

impl NegativeParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        attributes.insert("parameter", (&self.parameter).into());
        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
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
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        attributes.insert("parameter", (&self.parameter).into());
        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
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
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        attributes.insert("numerator", (&self.numerator).into());
        attributes.insert("denominator", (&self.denominator).into());
        attributes
    }
}
