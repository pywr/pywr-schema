use crate::parameters::{NodeReference, ParameterMeta, ParameterValue, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IndexedArrayParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    #[serde(alias = "params")]
    pub parameters: Vec<ParameterValue>,
    pub index_parameter: ParameterValue,
}

impl IndexedArrayParameter {
    pub fn node_references(&self) -> Vec<NodeReference> {
        Vec::new()
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();

        let parameters = &self.parameters;
        attributes.insert("parameters", parameters.into());

        attributes
    }
}
