use crate::parameters::{
    NodeReference, ParameterMeta, ParameterValue, ParameterValueType, TableDataRef,
};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ConstantParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    #[serde(alias = "values")]
    pub value: Option<f64>,
    #[serde(flatten)]
    pub table: Option<TableDataRef>,
}

impl ConstantParameter {
    pub fn node_references(&self) -> Vec<NodeReference> {
        Vec::new()
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MaxParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: Box<ParameterValue>,
    pub threshold: Option<f64>,
}

impl MaxParameter {
    pub fn node_references(&self) -> Vec<NodeReference> {
        Vec::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        attributes.insert("parameter", self.parameter.as_ref().into());
        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NegativeParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: Box<ParameterValue>,
}

impl NegativeParameter {
    pub fn node_references(&self) -> Vec<NodeReference> {
        Vec::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        attributes.insert("parameter", self.parameter.as_ref().into());
        attributes
    }
}
