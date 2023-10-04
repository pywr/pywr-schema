use crate::parameters::{ParameterMeta, ParameterValueType, ParameterValues};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct InterpolatedVolumeParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    pub volumes: ParameterValues,
    pub values: ParameterValues,
    pub interp_kwargs: Option<HashMap<String, serde_json::Value>>,
}

impl InterpolatedVolumeParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("node", self.node.as_str())].into_iter().collect()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes: HashMap<&str, ParameterValueType> = HashMap::new();

        attributes.insert("volumes", (&self.volumes).into());
        attributes.insert("values", (&self.values).into());

        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct InterpolatedFlowParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    pub flows: ParameterValues,
    pub values: ParameterValues,
    pub interp_kwargs: Option<HashMap<String, serde_json::Value>>,
}

impl InterpolatedFlowParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("node", self.node.as_str())].into_iter().collect()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes: HashMap<&str, ParameterValueType> = HashMap::new();

        attributes.insert("flows", (&self.flows).into());
        attributes.insert("values", (&self.values).into());

        attributes
    }
}
