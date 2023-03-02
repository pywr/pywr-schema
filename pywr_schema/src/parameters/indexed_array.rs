use crate::parameters::{ParameterMeta, ParameterValue, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct IndexedArrayParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    #[serde(alias = "params")]
    pub parameters: Vec<ParameterValue>,
    pub index_parameter: ParameterValue,
}

impl IndexedArrayParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();

        let p = &self.index_parameter;
        attributes.insert("index_parameter", p.into());

        let parameters = &self.parameters;
        attributes.insert("parameters", parameters.into());

        attributes
    }
}
