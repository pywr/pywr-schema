use crate::parameters::{ParameterMeta, ParameterValue, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct AsymmetricSwitchIndexParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub on_index_parameter: ParameterValue,
    pub off_index_parameter: ParameterValue,
}

impl AsymmetricSwitchIndexParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        attributes.insert("on_index_parameter", (&self.on_index_parameter).into());
        attributes.insert("off_index_parameter", (&self.off_index_parameter).into());
        attributes
    }
}
