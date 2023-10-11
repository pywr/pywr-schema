use crate::nodes::NodeMeta;
use crate::parameters::{
    ParameterValue, ParameterValueType, ParameterValueTypeMut, ParameterValues,
};
use pywr_schema_macros::PywrNode;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Clone, PywrNode)]
pub struct InputNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flow: Option<ParameterValue>,
    pub min_flow: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
}

impl InputNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PywrNode)]
pub struct LinkNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flow: Option<ParameterValue>,
    pub min_flow: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
}

impl LinkNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PywrNode)]
pub struct OutputNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flow: Option<ParameterValue>,
    pub min_flow: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
}

impl OutputNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PywrNode)]
pub struct StorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_volume: Option<ParameterValue>,
    pub min_volume: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
    pub initial_volume: Option<f64>,
    pub initial_volume_pc: Option<f64>,
}

impl StorageNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PywrNode)]
pub struct ReservoirNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_volume: Option<ParameterValue>,
    pub min_volume: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
    pub initial_volume: Option<f64>,
    pub initial_volume_pc: Option<f64>,
}

impl ReservoirNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PywrNode)]
pub struct CatchmentNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub flow: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
}

impl CatchmentNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PywrNode)]
pub struct AggregatedNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    pub max_flow: Option<ParameterValue>,
    pub min_flow: Option<ParameterValue>,
    pub factors: Option<ParameterValues>,
    pub flow_weights: Option<Vec<f64>>,
}

impl AggregatedNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "nodes",
            self.nodes.iter().map(|n| n.as_str()).collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PywrNode)]
pub struct AggregatedStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub storage_nodes: Vec<String>,
}

impl AggregatedStorageNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "storage_nodes",
            self.storage_nodes
                .iter()
                .map(|n| n.as_str())
                .collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}
