use crate::parameters::{ParameterMeta, ParameterValueType, ParameterValues};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ScenarioWrapperParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub scenario: String,
    pub parameters: ParameterValues,
}

impl ScenarioWrapperParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes: HashMap<&str, ParameterValueType> = HashMap::new();

        attributes.insert("parameters", (&self.parameters).into());

        attributes
    }
}
