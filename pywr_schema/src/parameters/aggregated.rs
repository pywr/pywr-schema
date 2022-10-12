use crate::parameters::{NodeReference, ParameterMeta, ParameterValue, ParameterValueType};
use std::collections::HashMap;

// TODO complete these
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AggFunc {
    Sum,
    Product,
    Max,
    Min,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AggregatedParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub agg_func: AggFunc,
    pub parameters: Vec<ParameterValue>,
}

impl AggregatedParameter {
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AggregatedIndexParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub agg_func: AggFunc,
    pub parameters: Vec<ParameterValue>,
}

impl AggregatedIndexParameter {
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
