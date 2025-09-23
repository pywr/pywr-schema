use crate::edge::Edge;
use crate::nodes::Node;
use crate::parameters::{Parameter, ParameterVec};
use crate::tables::TableVec;
use crate::PywrSchemaError;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Deserializer};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_version: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(untagged)]
pub enum Timestep {
    Days(u64),
    Frequency(String),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug)]
#[serde(untagged)]
pub enum DateType {
    Date(NaiveDate),
    DateTime(NaiveDateTime),
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Timestepper {
    pub start: DateType,
    pub end: DateType,
    pub timestep: Timestep,
}

fn validate_scenario_slice_length<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<Option<usize>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let slice: Option<Vec<Option<usize>>> = Option::deserialize(deserializer)?;

    if let Some(ref vec) = slice {
        if vec.len() < 2 || vec.len() > 3 {
            return Err(serde::de::Error::custom(
                "A scenario slice must have a length between 2 and 3 elements",
            ));
        }
    }

    Ok(slice)
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Scenario {
    pub name: String,
    pub size: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "validate_scenario_slice_length")]
    pub slice: Option<Vec<Option<usize>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensemble_names: Option<Vec<String>>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct PywrNetwork {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<Edge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<TableVec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorders: Option<serde_json::Value>,
}

impl PywrNetwork {
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
        match &self.nodes {
            Some(nodes) => nodes.iter().find(|n| n.name() == name),
            None => None,
        }
    }

    /// Return a node's index from its name. If no node with that name exists return [`None`].
    pub fn get_node_index_by_name(&self, name: &str) -> Option<usize> {
        match &self.nodes {
            Some(nodes) => nodes.iter().position(|n| n.name() == name),
            None => None,
        }
    }

    /// Return a [`Node`] from its index. If no node  with that name exists return [`None`].
    pub fn get_node(&self, idx: usize) -> Option<&Node> {
        match &self.nodes {
            Some(nodes) => nodes.get(idx),
            None => None,
        }
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

    /// Return all of the model's resource paths
    pub fn resource_paths(&self) -> HashSet<PathBuf> {
        let mut resource_paths = HashSet::new();

        resource_paths.extend(self.node_resource_paths());
        resource_paths.extend(self.parameter_resource_paths());
        resource_paths.extend(self.table_resource_paths());

        resource_paths
    }

    /// Return the model's node resource paths
    ///
    /// Note that this will include resource paths for any parameters defined inline within the nodes.
    pub fn node_resource_paths(&self) -> HashSet<PathBuf> {
        let mut resource_paths = HashSet::new();

        if let Some(nodes) = &self.nodes {
            for node in nodes.iter() {
                let paths = node.resource_paths();
                resource_paths.extend(paths);
            }
        }

        resource_paths
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

    /// Update resource paths
    pub fn update_resource_paths(&mut self, new_paths: &HashMap<PathBuf, PathBuf>) {
        if let Some(nodes) = &mut self.nodes {
            for node in nodes.iter_mut() {
                node.update_resource_paths(new_paths);
            }
        }

        if let Some(parameters) = &mut self.parameters {
            for parameter in parameters.iter_mut() {
                parameter.update_resource_paths(new_paths);
            }
        }

        if let Some(tables) = &mut self.tables {
            for table in tables.iter_mut() {
                table.update_resource_paths(new_paths);
            }
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct PywrModel {
    pub metadata: Metadata,
    pub timestepper: Timestepper,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenarios: Option<Vec<Scenario>>,
    #[serde(flatten)]
    pub network: PywrNetwork,
}

impl crate::PywrModel {
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
        self.network.get_node_by_name(name)
    }

    /// Return a node's index from its name. If no node with that name exists return [`None`].
    pub fn get_node_index_by_name(&self, name: &str) -> Option<usize> {
        self.network.get_node_index_by_name(name)
    }

    /// Return a [`Node`] from its index. If no node  with that name exists return [`None`].
    pub fn get_node(&self, idx: usize) -> Option<&Node> {
        self.network.get_node(idx)
    }

    /// Return a [`Parameter`] from its name. If no node  with that name exists return [`None`].
    pub fn get_parameter_by_name(&self, name: &str) -> Option<&Parameter> {
        self.network.get_parameter_by_name(name)
    }

    /// Return a parameter's index from its name. If no parameter with that name exists
    /// return [`None`].
    pub fn get_parameter_index_by_name(&self, name: &str) -> Option<usize> {
        self.network.get_parameter_index_by_name(name)
    }

    /// Return a [`Parameter`] from its index. If no parameter with that name exists return [`None`].
    pub fn get_parameter(&self, idx: usize) -> Option<&Parameter> {
        self.network.get_parameter(idx)
    }

    /// Return all of the model's resource paths
    pub fn resource_paths(&self) -> HashSet<PathBuf> {
        self.network.resource_paths()
    }

    /// Return the model's node resource paths
    ///
    /// Note that this will include resource paths for any parameters defined inline within the nodes.
    pub fn node_resource_paths(&self) -> HashSet<PathBuf> {
        self.network.node_resource_paths()
    }

    /// Return the model's parameter resource paths
    pub fn parameter_resource_paths(&self) -> HashSet<PathBuf> {
        self.network.parameter_resource_paths()
    }

    /// Return the model's table resource paths
    pub fn table_resource_paths(&self) -> HashSet<PathBuf> {
        self.network.table_resource_paths()
    }

    /// Update resource paths
    pub fn update_resource_paths(&mut self, new_paths: &HashMap<PathBuf, PathBuf>) {
        self.network.update_resource_paths(new_paths)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::PywrModel;
    use std::collections::{HashMap, HashSet};
    use std::ffi::{OsStr, OsString};
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

        assert_eq!(model.network.nodes.unwrap().len(), 3);
        assert_eq!(model.network.edges.unwrap().len(), 2);
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

        // Create a map of models that have external resources
        let mut expected_resources_bv_model: HashMap<OsString, Vec<PathBuf>> = HashMap::new();
        expected_resources_bv_model.insert(
            OsString::from("demand_saving_hdf.json"),
            vec![PathBuf::from("demand_saving_level.h5")],
        );
        expected_resources_bv_model.insert(
            OsString::from("multiindex_df.json"),
            vec![PathBuf::from("multiindex_data.csv")],
        );

        expected_resources_bv_model.insert(
            OsString::from("reservoir_initial_vol_from_table.json"),
            vec![PathBuf::from("initial_volumes.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("reservoir_with_cc.json"),
            vec![PathBuf::from("control_curve.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("reservoir_with_cc_param_values.json"),
            vec![PathBuf::from("control_curve.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("reservoir_with_circular_cc.json"),
            vec![PathBuf::from("control_curve.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("scenario_monthly_profile.json"),
            vec![
                PathBuf::from("timeseries1.csv"),
                // TODO implement ScenarioMonthlyProfileParameter
                // PathBuf::from("monthly_profiles.csv"),
            ],
        );
        expected_resources_bv_model.insert(
            OsString::from("simple_df.json"),
            vec![PathBuf::from("simple_data.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("simple_df_shared.json"),
            vec![PathBuf::from("simple_data.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("timeseries1.json"),
            vec![PathBuf::from("timeseries1.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("timeseries1_weekly.json"),
            vec![PathBuf::from("timeseries1_weekly.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("timeseries1_weekly_hdf.json"),
            vec![PathBuf::from("timeseries1_weekly.h5")],
        );
        expected_resources_bv_model.insert(
            OsString::from("timeseries1_xlsx.json"),
            vec![PathBuf::from("test_data1.xlsx")],
        );
        expected_resources_bv_model.insert(
            OsString::from("timeseries2.json"),
            vec![PathBuf::from("timeseries2.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("timeseries2_hdf_wrong_hash.json"),
            vec![PathBuf::from("timeseries2.h5")],
        );
        expected_resources_bv_model.insert(
            OsString::from("timeseries2_hdf.json"),
            vec![PathBuf::from("timeseries2.h5")],
        );
        expected_resources_bv_model.insert(
            OsString::from("timeseries2_with_fdc.json"),
            vec![PathBuf::from("timeseries2.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("timeseries3.json"),
            vec![PathBuf::from("timeseries2.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("timeseries3_subsample.json"),
            vec![PathBuf::from("timeseries2.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("timeseries4.json"),
            vec![PathBuf::from("timeseries3.csv")],
        );
        expected_resources_bv_model.insert(
            OsString::from("two_reservoir.json"),
            vec![PathBuf::from("data/thames_stochastic_flow.gz")],
        );
        expected_resources_bv_model.insert(
            OsString::from("two_reservoir_constrained.json"),
            vec![PathBuf::from("data/thames_stochastic_flow.gz")],
        );

        for path in read_dir(test_models_dir).unwrap() {
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
            if let Some(parameters) = &model.network.parameters {
                if parameters.iter().any(|p| p.is_custom()) {
                    panic!(
                        "Deserialised model ({:?}) contains unexpected custom parameters!",
                        model_fn
                    )
                }
            }

            let found_resources = model.resource_paths();

            // If external resources are expected, check they are found
            match expected_resources_bv_model.get(model_fn.file_name().unwrap()) {
                Some(expected_resources) => {
                    let expected_resources: HashSet<PathBuf> =
                        expected_resources.iter().cloned().collect();
                    assert_eq!(
                        found_resources, expected_resources,
                        "Expected resources did not match those found in model ({:?})",
                        model_fn
                    );
                }
                None => {
                    if !found_resources.is_empty() {
                        panic!(
                            "Found unexpected resources in model ({:?}): {:?}",
                            model_fn, found_resources
                        );
                    }
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
