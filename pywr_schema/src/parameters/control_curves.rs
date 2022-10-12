use crate::parameters::{
    NodeReference, ParameterMeta, ParameterValue, ParameterValueType, ParameterValues,
};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ControlCurveInterpolatedParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub control_curve: Option<Box<ParameterValue>>,
    pub control_curves: Option<ParameterValues>,
    pub storage_node: String,
    pub values: Vec<f64>,
}

impl ControlCurveInterpolatedParameter {
    pub fn node_references(&self) -> Vec<NodeReference> {
        vec![NodeReference {
            attribute: "storage_node".to_string(),
            node: self.storage_node.clone(),
        }]
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.control_curve {
            attributes.insert("control_curve", p.as_ref().into());
        }
        if let Some(parameters) = &self.control_curves {
            attributes.insert("control_curves", parameters.into());
        }
        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ControlCurveIndexParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub control_curves: ParameterValues,
    pub parameters: Option<Vec<ParameterValue>>,
    pub storage_node: String,
}

impl ControlCurveIndexParameter {
    pub fn node_references(&self) -> Vec<NodeReference> {
        vec![NodeReference {
            attribute: "storage_node".to_string(),
            node: self.storage_node.clone(),
        }]
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();

        let cc = &self.control_curves;
        attributes.insert("control_curve", cc.into());

        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ControlCurveParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub control_curve: Option<Box<ParameterValue>>,
    pub control_curves: Option<Vec<ParameterValue>>,

    pub storage_node: String,
    pub values: Option<Vec<f64>>,
    pub parameters: Option<Vec<ParameterValue>>,
}

impl ControlCurveParameter {
    pub fn node_references(&self) -> Vec<NodeReference> {
        vec![NodeReference {
            attribute: "storage_node".to_string(),
            node: self.storage_node.clone(),
        }]
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.control_curve {
            attributes.insert("control_curve", p.as_ref().into());
        }
        if let Some(parameters) = &self.control_curves {
            attributes.insert("control_curves", parameters.into());
        }
        if let Some(parameters) = &self.parameters {
            attributes.insert("parameters", parameters.into());
        }
        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ControlCurvePiecewiseInterpolatedParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub control_curves: Vec<ParameterValue>,
    pub storage_node: String,
    pub values: Option<Vec<[f64; 2]>>,
    pub minimum: f64,
}

impl ControlCurvePiecewiseInterpolatedParameter {
    pub fn node_references(&self) -> Vec<NodeReference> {
        vec![NodeReference {
            attribute: "storage_node".to_string(),
            node: self.storage_node.clone(),
        }]
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();

        let cc = &self.control_curves;
        attributes.insert("control_curve", cc.into());

        attributes
    }
}
