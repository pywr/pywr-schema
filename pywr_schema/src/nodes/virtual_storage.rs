use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValue, ParameterValueType};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct VirtualStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    pub factors: Option<Vec<f64>>,
    pub max_volume: Option<ParameterValue>,
    pub min_volume: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
    pub initial_volume: Option<f64>,
    pub initial_volume_pc: Option<f64>,
}

impl VirtualStorageNode {
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.max_volume {
            attributes.insert("max_volume", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.min_volume {
            attributes.insert("min_volume", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", ParameterValueType::Single(p));
        }

        attributes
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "nodes",
            self.nodes.iter().map(|n| n.as_str()).collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}

fn default_reset_day() -> u8 {
    1
}

fn default_reset_month() -> time::Month {
    time::Month::January
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct AnnualVirtualStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    pub factors: Option<Vec<f64>>,
    pub max_volume: Option<ParameterValue>,
    pub min_volume: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
    pub initial_volume: Option<f64>,
    pub initial_volume_pc: Option<f64>,
    #[serde(default = "default_reset_day")]
    pub reset_day: u8,
    #[serde(default = "default_reset_month")]
    pub reset_month: time::Month,
    #[serde(default)]
    pub reset_to_initial_volume: bool,
}

impl AnnualVirtualStorageNode {
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.max_volume {
            attributes.insert("max_volume", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.min_volume {
            attributes.insert("min_volume", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", ParameterValueType::Single(p));
        }

        attributes
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "nodes",
            self.nodes.iter().map(|n| n.as_str()).collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}

fn default_months() -> u8 {
    1
}

fn default_initial_months() -> u8 {
    1
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct MonthlyVirtualStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    pub factors: Option<Vec<f64>>,
    pub max_volume: Option<ParameterValue>,
    pub min_volume: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
    pub initial_volume: Option<f64>,
    pub initial_volume_pc: Option<f64>,
    #[serde(default = "default_months")]
    pub months: u8,
    #[serde(default = "default_initial_months")]
    pub initial_months: u8,
    #[serde(default)]
    pub reset_to_initial_volume: bool,
}

impl MonthlyVirtualStorageNode {
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.max_volume {
            attributes.insert("max_volume", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.min_volume {
            attributes.insert("min_volume", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", ParameterValueType::Single(p));
        }

        attributes
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "nodes",
            self.nodes.iter().map(|n| n.as_str()).collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}

fn default_end_day() -> u8 {
    31
}

fn default_end_month() -> time::Month {
    time::Month::December
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SeasonalVirtualStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    pub factors: Option<Vec<f64>>,
    pub max_volume: Option<ParameterValue>,
    pub min_volume: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
    pub initial_volume: Option<f64>,
    pub initial_volume_pc: Option<f64>,
    #[serde(default = "default_reset_day")]
    pub reset_day: u8,
    #[serde(default = "default_reset_month")]
    pub reset_month: time::Month,
    #[serde(default = "default_end_day")]
    pub end_day: u8,
    #[serde(default = "default_end_month")]
    pub end_month: time::Month,
    #[serde(default)]
    pub reset_to_initial_volume: bool,
}

impl SeasonalVirtualStorageNode {
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.max_volume {
            attributes.insert("max_volume", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.min_volume {
            attributes.insert("min_volume", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", ParameterValueType::Single(p));
        }

        attributes
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "nodes",
            self.nodes.iter().map(|n| n.as_str()).collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct RollingVirtualStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    pub factors: Option<Vec<f64>>,
    pub max_volume: Option<ParameterValue>,
    pub min_volume: Option<ParameterValue>,
    pub cost: Option<ParameterValue>,
    pub initial_volume: Option<f64>,
    pub initial_volume_pc: Option<f64>,
    pub timesteps: Option<i64>,
    pub days: Option<i64>,
}

impl RollingVirtualStorageNode {
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        let mut attributes = HashMap::new();
        if let Some(p) = &self.max_volume {
            attributes.insert("max_volume", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.min_volume {
            attributes.insert("min_volume", ParameterValueType::Single(p));
        }
        if let Some(p) = &self.cost {
            attributes.insert("cost", ParameterValueType::Single(p));
        }

        attributes
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "nodes",
            self.nodes.iter().map(|n| n.as_str()).collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::AnnualVirtualStorageNode;

    #[test]
    fn test_annual_virtual_storage() {
        let data = r#"
            {
              "type": "annualvirtualstorage",
              "name": "Scales AL",
              "max_volume": 365,
              "nodes": [
                "Scales BHs"
              ],
              "initial_volume": 365
            }
            "#;

        let node: AnnualVirtualStorageNode = serde_json::from_str(data).unwrap();

        assert_eq!(node.meta.name, "Scales AL");
    }
}
