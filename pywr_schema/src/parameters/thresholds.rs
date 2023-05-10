use crate::parameters::{ParameterMeta, ParameterValue, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
pub enum Predicate {
    #[serde(alias = "<")]
    LT,
    #[serde(alias = ">")]
    GT,
    #[serde(alias = "==")]
    EQ,
    #[serde(alias = "<=")]
    LE,
    #[serde(alias = ">=")]
    GE,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ParameterThresholdParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
    pub threshold: ParameterValue,
    pub values: Option<Vec<f64>>,
    pub predicate: Predicate,
}

impl ParameterThresholdParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();

        attributes.insert("parameter", (&self.parameter).into());

        attributes.insert("threshold", (&self.threshold).into());

        attributes
    }
}
