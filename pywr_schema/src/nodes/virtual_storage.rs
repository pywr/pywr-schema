use crate::nodes::NodeMeta;
use crate::parameters::ParameterValue;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
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

fn default_reset_day() -> u8 {
    1
}

fn default_reset_month() -> time::Month {
    time::Month::January
}

#[derive(serde::Deserialize, serde::Serialize)]
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
