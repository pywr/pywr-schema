use crate::nodes::NodeMeta;
use crate::parameters::ParameterValue;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct InputNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flow: Option<ParameterValue>,
    pub min_flow: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
}

impl InputNode {
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.max_flow {
            attributes.insert("max_flow", p);
        }
        if let Some(p) = &self.min_flow {
            attributes.insert("min_flow", p);
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", p);
        }

        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LinkNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flow: Option<ParameterValue>,
    pub min_flow: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
}

impl LinkNode {
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.max_flow {
            attributes.insert("max_flow", p);
        }
        if let Some(p) = &self.min_flow {
            attributes.insert("min_flow", p);
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", p);
        }

        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OutputNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub max_flow: Option<ParameterValue>,
    pub min_flow: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
}

impl OutputNode {
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.max_flow {
            attributes.insert("max_flow", p);
        }
        if let Some(p) = &self.min_flow {
            attributes.insert("min_flow", p);
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", p);
        }

        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
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
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.max_volume {
            attributes.insert("max_volume", p);
        }
        if let Some(p) = &self.min_volume {
            attributes.insert("min_volume", p);
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", p);
        }

        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
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
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.max_volume {
            attributes.insert("max_volume", p);
        }
        if let Some(p) = &self.min_volume {
            attributes.insert("min_volume", p);
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", p);
        }

        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CatchmentNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub flow: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
}

impl CatchmentNode {
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.flow {
            attributes.insert("flow", p);
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", p);
        }

        attributes
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AggregatedNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    pub max_flow: Option<ParameterValue>,
    pub min_flow: Option<ParameterValue>,
    pub factors: Option<Vec<ParameterValue>>,
}

impl AggregatedNode {
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AggregatedStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub storage_nodes: Vec<String>,
}

impl AggregatedStorageNode {
    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        HashMap::new()
    }
}
