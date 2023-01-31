use crate::edge::Edge;
use crate::nodes::Node;
use crate::parameters::{Parameter, ParameterVec};
use crate::tables::TableVec;
use time::Date;

#[derive(serde::Deserialize)]
pub struct Metadata {
    pub title: String,
    pub description: Option<String>,
    pub minimum_version: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum Timestep {
    Days(u64),
    Frequency(String),
}

#[derive(serde::Deserialize)]
pub struct Timestepper {
    pub start: Date,
    pub end: Date,
    pub timestep: Timestep,
}

#[derive(serde::Deserialize)]
pub struct Scenario {
    pub name: String,
    pub size: usize,
    pub ensemble_names: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
pub struct PywrModel {
    pub metadata: Metadata,
    pub timestepper: Timestepper,
    pub scenarios: Option<Vec<Scenario>>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub parameters: Option<ParameterVec>,
    pub tables: Option<TableVec>,
    pub recorders: Option<serde_json::Value>,
}

impl PywrModel {
    /// Return a [`Node`] from its name. If no node  with that name exists return [`None`].
    pub fn get_node_by_name(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name() == name)
    }

    /// Return a node's index from its name. If no node with that name exists return [`None`].
    pub fn get_node_index_by_name(&self, name: &str) -> Option<usize> {
        self.nodes.iter().position(|n| (n.name() == name))
    }

    /// Return a [`Node`] from its index. If no node  with that name exists return [`None`].
    pub fn get_node(&self, idx: usize) -> Option<&Node> {
        self.nodes.get(idx)
    }

    /// Return a [`Parameter`] from its name. If no node  with that name exists return [`None`].
    pub fn get_parameter_by_name(&self, name: &str) -> Option<&Parameter> {
        match &self.parameters {
            Some(parameters) => parameters.iter().find(|p| p.name() == Some(name)),
            None => None,
        }
    }

    /// Return a parameter's index from its name. If no parameter with that name exists
    /// return [`None`].
    pub fn get_parameter_index_by_name(&self, name: &str) -> Option<usize> {
        match &self.parameters {
            Some(parameters) => parameters.iter().position(|n| (n.name() == Some(name))),
            None => None,
        }
    }

    /// Return a [`Parameter`] from its index. If no parameter with that name exists return [`None`].
    pub fn get_parameter(&self, idx: usize) -> Option<&Parameter> {
        match &self.parameters {
            Some(parameters) => parameters.get(idx),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::PywrModel;

    #[test]
    fn test_simple1() {
        let data = r#"
            {
                "metadata": {
                    "title": "Simple 1",
                    "description": "A very simple example.",
                    "minimum_version": "0.1"
                },
                "timestepper": {
                    "start": "2015-01-01",
                    "end": "2015-12-31",
                    "timestep": 1
                },
                "nodes": [
                    {
                        "name": "supply1",
                        "type": "Input",
                        "max_flow": 15
                    },
                    {
                        "name": "link1",
                        "type": "Link"
                    },
                    {
                        "name": "demand1",
                        "type": "Output",
                        "max_flow": 10,
                        "cost": -10
                    }
                ],
                "edges": [
                    ["supply1", "link1"],
                    ["link1", "demand1"]
                ]
            }
            "#;

        let model: PywrModel = serde_json::from_str(data).unwrap();

        assert_eq!(model.nodes.len(), 3);
        assert_eq!(model.edges.len(), 2);
    }
}
