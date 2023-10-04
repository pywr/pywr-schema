use crate::edge::Edge;
use crate::nodes::Node;
use crate::parameters::{Parameter, ParameterVec};
use crate::tables::TableVec;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;
use time::Date;

#[derive(Error, Debug)]
pub enum PywrSchemaError {
    #[error("An invalid URL was found.")]
    InvalidUrlFound,
    #[error("data store disconnected")]
    IoError(#[from] io::Error),
    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),
    #[error("Resource not found on local host: {0}")]
    LocalResourceNotFound(PathBuf),
    #[error("Invalid Pywr format")]
    InvalidPywrDataFormat,
}

#[derive(serde::Deserialize, Clone)]
pub struct Metadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub minimum_version: Option<String>,
}

#[derive(serde::Deserialize, Clone)]
#[serde(untagged)]
pub enum Timestep {
    Days(u64),
    Frequency(String),
}

#[derive(serde::Deserialize, Clone)]
pub struct Timestepper {
    pub start: Date,
    pub end: Date,
    pub timestep: Timestep,
}

#[derive(serde::Deserialize, Clone)]
pub struct Scenario {
    pub name: String,
    pub size: usize,
    pub ensemble_names: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Clone)]
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
    /// Load a PywrNetwork from a file path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, PywrSchemaError> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        // Read the JSON contents of the file as an instance of `User`.
        let data = serde_json::from_reader(reader)?;

        Ok(data)
    }

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

    /// Return the model's parameter resource paths
    pub fn parameter_resource_paths(&self) -> HashSet<PathBuf> {
        let mut resource_paths = HashSet::new();

        if let Some(parameters) = &self.parameters {
            for parameter in parameters.iter() {
                let paths = parameter.resource_paths();
                resource_paths.extend(paths);
            }
        }

        resource_paths
    }

    /// Return the model's table resource paths
    pub fn table_resource_paths(&self) -> HashSet<PathBuf> {
        let mut resource_paths = HashSet::new();

        if let Some(tables) = &self.tables {
            for table in tables.iter() {
                let paths = table.resource_paths();
                resource_paths.extend(paths);
            }
        }

        resource_paths
    }
}

#[cfg(test)]
mod tests {
    use crate::model::PywrModel;
    use std::ffi::OsStr;
    use std::fs::read_dir;
    use std::path::PathBuf;

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

    // #[test]
    // fn find_urls() {
    //     let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     let model_path = test_dir.join("../test-data/timeseries1.json");
    //     let network = PywrModel::from_path(&model_path).unwrap();
    //     let resources = network.parameter_resource_paths();
    //     let mut expected = HashSet::new();
    //     expected.insert(test_dir.join("../test-data/timeseries1.csv"));
    //     assert_eq!(resources, expected);
    // }

    /// Test the models from the Pywr repository
    #[test]
    fn test_pywr_models() {
        let test_models_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("models");
        // Some models have custom parameters that cannot be deserialised
        let exclude_models: [&OsStr; 2] = [
            OsStr::new("extra2.json"),
            OsStr::new("reservoir_evaporation_areafromfile.json"),
        ];

        for path in read_dir(&test_models_dir).unwrap() {
            let model_fn = path.unwrap().path();
            if exclude_models.contains(&model_fn.file_name().unwrap()) {
                continue;
            }
            // Ensure the model can be deserialised
            let model = PywrModel::from_path(&model_fn).unwrap_or_else(|e| {
                panic!(
                    "Failed to deserialise model ({:?}) with error: {:?}",
                    model_fn, e
                )
            });

            // None of the standard models should have custom parameters
            if let Some(parameters) = &model.parameters {
                if parameters.iter().any(|p| p.is_custom()) {
                    panic!(
                        "Deserialised model ({:?}) contains unexpected custom parameters!",
                        model_fn
                    )
                }
            }
        }
    }

    // #[test]
    // fn replace_urls() {
    //     let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     let model_path = test_dir.join("../test-data/timeseries1.json");
    //     let mut network = PywrModel::from_path(&model_path).unwrap();
    //
    //     let urls = HashMap::from([("timeseries1.csv".to_string(), id.clone())]);
    //     network.replace_external_resources_urls(&urls).unwrap();
    //     let flow_url = network
    //         .parameters
    //         .unwrap()
    //         .get("flow")
    //         .unwrap()
    //         .get("url")
    //         .unwrap()
    //         .as_str()
    //         .unwrap()
    //         .to_string();
    //     let expected_url = format!("{}.csv", id);
    //     assert!(flow_url == expected_url);
    // }
}
