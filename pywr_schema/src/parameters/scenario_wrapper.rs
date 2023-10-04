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
        let attributes: HashMap<&str, ParameterValueType> = HashMap::new();

        for (_, _) in self.parameters.iter().enumerate() {
            // TODO this doesn't work because the temporary String; needs an API change
            // let key = format!("parameters[{}]", i);
            // attributes.insert(&key, p.into());
        }

        attributes
    }
}
