mod aggregated;
mod asymmetric_switch;
mod control_curves;
mod core;
mod data_frame;
mod deficit;
mod discount_factor;
mod flow;
mod hydropower;
mod indexed_array;
mod interpolated;
mod polynomial;
mod profiles;
mod rolling_mean_flow_node;
mod scenario_wrapper;
mod storage;
mod tables;
mod thresholds;

pub use crate::parameters::aggregated::{
    AggFunc, AggregatedIndexParameter, AggregatedParameter, IndexAggFunc,
};
pub use crate::parameters::asymmetric_switch::AsymmetricSwitchIndexParameter;
pub use crate::parameters::control_curves::{
    ControlCurveIndexParameter, ControlCurveInterpolatedParameter, ControlCurveParameter,
    ControlCurvePiecewiseInterpolatedParameter,
};
pub use crate::parameters::core::{
    ConstantParameter, DivisionParameter, MaxParameter, MinParameter, NegativeParameter,
};
pub use crate::parameters::deficit::DeficitParameter;
pub use crate::parameters::discount_factor::DiscountFactorParameter;
pub use crate::parameters::flow::FlowParameter;
pub use crate::parameters::hydropower::HydropowerTargetParameter;
pub use crate::parameters::indexed_array::IndexedArrayParameter;
pub use crate::parameters::interpolated::{InterpolatedFlowParameter, InterpolatedVolumeParameter};
pub use crate::parameters::polynomial::Polynomial1DParameter;
use crate::parameters::profiles::RbfProfileParameter;
pub use crate::parameters::profiles::{
    DailyProfileParameter, MonthInterpDay, MonthlyProfileParameter, UniformDrawdownProfileParameter,
};
pub use crate::parameters::rolling_mean_flow_node::RollingMeanFlowNodeParameter;
pub use crate::parameters::scenario_wrapper::ScenarioWrapperParameter;
pub use crate::parameters::storage::StorageParameter;
pub use crate::parameters::tables::TablesArrayParameter;
pub use crate::parameters::thresholds::{ParameterThresholdParameter, Predicate};
pub use data_frame::DataFrameParameter;
use serde::de::value::MapDeserializer;
use serde::de::{MapAccess, Visitor};
use serde::ser::{Error, SerializeMap};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::vec::IntoIter;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ParameterMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CustomParameter {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(flatten)]
    pub meta: ParameterMeta,
    #[serde(flatten)]
    pub attributes: HashMap<String, Value>,
}

// A lot of alias here until serde supports case-insensitive deserialization of tags
// Issues:
//   - https://github.com/serde-rs/serde/pull/1902
//   - https://github.com/serde-rs/serde/pull/2161
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum CoreParameter {
    #[serde(
        alias = "aggregated",
        alias = "aggregatedparameter",
        alias = "AggregatedParameter"
    )]
    Aggregated(AggregatedParameter),
    #[serde(
        alias = "aggregatedindex",
        alias = "aggregatedindexparameter",
        alias = "AggregatedIndexParameter"
    )]
    AggregatedIndex(AggregatedIndexParameter),
    #[serde(
        alias = "asymmetricswitchindex",
        alias = "asymmetricswitchindexparameter",
        alias = "AsymmetricSwitchIndexParameter"
    )]
    AsymmetricSwitchIndex(AsymmetricSwitchIndexParameter),
    #[serde(
        alias = "constant",
        alias = "constantparameter",
        alias = "ConstantParameter"
    )]
    Constant(ConstantParameter),
    #[serde(
        alias = "controlcurvepiecewiseinterpolated",
        alias = "controlcurvepiecewiseinterpolatedparameter",
        alias = "ControlCurvePiecewiseInterpolatedParameter"
    )]
    ControlCurvePiecewiseInterpolated(ControlCurvePiecewiseInterpolatedParameter),
    #[serde(
        alias = "controlcurveinterpolated",
        alias = "controlcurveinterpolatedparameter",
        alias = "ControlCurveInterpolatedParameter"
    )]
    ControlCurveInterpolated(ControlCurveInterpolatedParameter),
    #[serde(
        alias = "controlcurveindex",
        alias = "controlcurveindexparameter",
        alias = "ControlCurveIndexParameter"
    )]
    ControlCurveIndex(ControlCurveIndexParameter),
    #[serde(
        alias = "controlcurve",
        alias = "controlcurveparameter",
        alias = "ControlCurveParameter"
    )]
    ControlCurve(ControlCurveParameter),
    #[serde(
        alias = "dailyprofile",
        alias = "dailyprofileparameter",
        alias = "DailyProfileParameter"
    )]
    DailyProfile(DailyProfileParameter),
    #[serde(
        alias = "indexedarray",
        alias = "indexedarrayparameter",
        alias = "IndexedArrayParameter"
    )]
    IndexedArray(IndexedArrayParameter),
    #[serde(
        alias = "monthlyprofile",
        alias = "monthlyprofileparameter",
        alias = "MonthlyProfileParameter"
    )]
    MonthlyProfile(MonthlyProfileParameter),
    #[serde(
        alias = "uniformdrawdownprofile",
        alias = "uniformdrawdownprofileparameter",
        alias = "UniformDrawdownProfileParameter"
    )]
    UniformDrawdownProfile(UniformDrawdownProfileParameter),
    #[serde(alias = "max", alias = "maxparameter", alias = "MaxParameter")]
    Max(MaxParameter),
    #[serde(alias = "min", alias = "minparameter", alias = "MinParameter")]
    Min(MinParameter),
    #[serde(
        alias = "division",
        alias = "divisionparameter",
        alias = "DivisionParameter"
    )]
    Division(DivisionParameter),
    #[serde(
        alias = "negative",
        alias = "negativeparameter",
        alias = "NegativeParameter"
    )]
    Negative(NegativeParameter),
    #[serde(
        alias = "polynomial1d",
        alias = "polynomial1dparameter",
        alias = "Polynomial1DParameter"
    )]
    Polynomial1D(Polynomial1DParameter),
    #[serde(
        alias = "parameterthreshold",
        alias = "parameterthresholdparameter",
        alias = "ParameterThresholdParameter"
    )]
    ParameterThreshold(ParameterThresholdParameter),
    #[serde(
        alias = "tablesarray",
        alias = "tablesarrayparameter",
        alias = "TablesArrayParameter"
    )]
    TablesArray(TablesArrayParameter),
    #[serde(
        alias = "dataframe",
        alias = "dataframeparameter",
        alias = "DataFrameParameter"
    )]
    DataFrame(DataFrameParameter),
    #[serde(
        alias = "deficit",
        alias = "deficitparameter",
        alias = "DeficitParameter"
    )]
    Deficit(DeficitParameter),
    #[serde(
        alias = "discountfactor",
        alias = "discountfactorparameter",
        alias = "DiscountFactorParameter"
    )]
    DiscountFactor(DiscountFactorParameter),
    #[serde(
        alias = "interpolatedvolume",
        alias = "interpolatedvolumeparameter",
        alias = "InterpolatedVolumeParameter"
    )]
    InterpolatedVolume(InterpolatedVolumeParameter),
    #[serde(
        alias = "interpolatedflow",
        alias = "interpolatedflowparameter",
        alias = "InterpolatedFlowParameter"
    )]
    InterpolatedFlow(InterpolatedFlowParameter),
    #[serde(
        alias = "hydropowertarget",
        alias = "hydropowertargetparameter",
        alias = "HydropowerTargetParameter"
    )]
    HydropowerTarget(HydropowerTargetParameter),
    #[serde(
        alias = "storage",
        alias = "storageparameter",
        alias = "StorageParameter"
    )]
    Storage(StorageParameter),
    #[serde(
        alias = "rollingmeanflownode",
        alias = "rollingmeanflownodeparameter",
        alias = "RollingMeanFlowNodeParameter"
    )]
    RollingMeanFlowNode(RollingMeanFlowNodeParameter),
    #[serde(
        alias = "scenariowrapper",
        alias = "scenariowrapperparameter",
        alias = "ScenarioWrapperParameter"
    )]
    ScenarioWrapper(ScenarioWrapperParameter),
    #[serde(alias = "flow", alias = "flowparameter", alias = "FlowParameter")]
    Flow(FlowParameter),
    #[serde(
        alias = "rbfprofile",
        alias = "rbfprofileparameter",
        alias = "RbfProfileParameter"
    )]
    RbfProfile(RbfProfileParameter),
}

impl CoreParameter {
    fn name(&self) -> Option<&str> {
        match self {
            Self::Constant(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::ControlCurveInterpolated(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Aggregated(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::AggregatedIndex(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::AsymmetricSwitchIndex(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::ControlCurvePiecewiseInterpolated(p) => {
                p.meta.as_ref().and_then(|m| m.name.as_deref())
            }
            Self::ControlCurveIndex(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::ControlCurve(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::DailyProfile(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::IndexedArray(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::MonthlyProfile(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::UniformDrawdownProfile(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Max(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Min(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Division(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Negative(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Polynomial1D(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::ParameterThreshold(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::TablesArray(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::DataFrame(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Deficit(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::DiscountFactor(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::InterpolatedVolume(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::InterpolatedFlow(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::HydropowerTarget(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Storage(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::RollingMeanFlowNode(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::ScenarioWrapper(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Flow(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::RbfProfile(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
        }
    }

    fn node_references(&self) -> HashMap<&str, &str> {
        match self {
            Self::Constant(p) => p.node_references(),
            Self::ControlCurveInterpolated(p) => p.node_references(),
            Self::Aggregated(p) => p.node_references(),
            Self::AggregatedIndex(p) => p.node_references(),
            Self::AsymmetricSwitchIndex(p) => p.node_references(),
            Self::ControlCurvePiecewiseInterpolated(p) => p.node_references(),
            Self::ControlCurveIndex(p) => p.node_references(),
            Self::ControlCurve(p) => p.node_references(),
            Self::DailyProfile(p) => p.node_references(),
            Self::IndexedArray(p) => p.node_references(),
            Self::MonthlyProfile(p) => p.node_references(),
            Self::UniformDrawdownProfile(p) => p.node_references(),
            Self::Max(p) => p.node_references(),
            Self::Min(p) => p.node_references(),
            Self::Division(p) => p.node_references(),
            Self::Negative(p) => p.node_references(),
            Self::Polynomial1D(p) => p.node_references(),
            Self::ParameterThreshold(p) => p.node_references(),
            Self::TablesArray(p) => p.node_references(),
            Self::DataFrame(p) => p.node_references(),
            Self::Deficit(p) => p.node_references(),
            Self::DiscountFactor(p) => p.node_references(),
            Self::InterpolatedVolume(p) => p.node_references(),
            Self::InterpolatedFlow(p) => p.node_references(),
            Self::HydropowerTarget(p) => p.node_references(),
            Self::Storage(p) => p.node_references(),
            Self::RollingMeanFlowNode(p) => p.node_references(),
            Self::ScenarioWrapper(p) => p.node_references(),
            Self::Flow(p) => p.node_references(),
            Self::RbfProfile(p) => p.node_references(),
        }
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        match self {
            Self::Constant(p) => p.parameters(),
            Self::ControlCurveInterpolated(p) => p.parameters(),
            Self::Aggregated(p) => p.parameters(),
            Self::AggregatedIndex(p) => p.parameters(),
            Self::AsymmetricSwitchIndex(p) => p.parameters(),
            Self::ControlCurvePiecewiseInterpolated(p) => p.parameters(),
            Self::ControlCurveIndex(p) => p.parameters(),
            Self::ControlCurve(p) => p.parameters(),
            Self::DailyProfile(p) => p.parameters(),
            Self::IndexedArray(p) => p.parameters(),
            Self::MonthlyProfile(p) => p.parameters(),
            Self::UniformDrawdownProfile(p) => p.parameters(),
            Self::Min(p) => p.parameters(),
            Self::Max(p) => p.parameters(),
            Self::Division(p) => p.parameters(),
            Self::Negative(p) => p.parameters(),
            Self::Polynomial1D(p) => p.parameters(),
            Self::ParameterThreshold(p) => p.parameters(),
            Self::TablesArray(p) => p.parameters(),
            Self::DataFrame(p) => p.parameters(),
            Self::Deficit(p) => p.parameters(),
            Self::DiscountFactor(p) => p.parameters(),
            Self::InterpolatedVolume(p) => p.parameters(),
            Self::InterpolatedFlow(p) => p.parameters(),
            Self::HydropowerTarget(p) => p.parameters(),
            Self::Storage(p) => p.parameters(),
            Self::RollingMeanFlowNode(p) => p.parameters(),
            Self::ScenarioWrapper(p) => p.parameters(),
            Self::Flow(p) => p.parameters(),
            Self::RbfProfile(p) => p.parameters(),
        }
    }

    pub fn parameters_mut(&mut self) -> HashMap<&str, ParameterValueTypeMut> {
        match self {
            Self::Constant(p) => p.parameters_mut(),
            Self::ControlCurveInterpolated(p) => p.parameters_mut(),
            Self::Aggregated(p) => p.parameters_mut(),
            Self::AggregatedIndex(p) => p.parameters_mut(),
            Self::AsymmetricSwitchIndex(p) => p.parameters_mut(),
            Self::ControlCurvePiecewiseInterpolated(p) => p.parameters_mut(),
            Self::ControlCurveIndex(p) => p.parameters_mut(),
            Self::ControlCurve(p) => p.parameters_mut(),
            Self::DailyProfile(p) => p.parameters_mut(),
            Self::IndexedArray(p) => p.parameters_mut(),
            Self::MonthlyProfile(p) => p.parameters_mut(),
            Self::UniformDrawdownProfile(p) => p.parameters_mut(),
            Self::Min(p) => p.parameters_mut(),
            Self::Max(p) => p.parameters_mut(),
            Self::Division(p) => p.parameters_mut(),
            Self::Negative(p) => p.parameters_mut(),
            Self::Polynomial1D(p) => p.parameters_mut(),
            Self::ParameterThreshold(p) => p.parameters_mut(),
            Self::TablesArray(p) => p.parameters_mut(),
            Self::DataFrame(p) => p.parameters_mut(),
            Self::Deficit(p) => p.parameters_mut(),
            Self::DiscountFactor(p) => p.parameters_mut(),
            Self::InterpolatedVolume(p) => p.parameters_mut(),
            Self::InterpolatedFlow(p) => p.parameters_mut(),
            Self::HydropowerTarget(p) => p.parameters_mut(),
            Self::Storage(p) => p.parameters_mut(),
            Self::RollingMeanFlowNode(p) => p.parameters_mut(),
            Self::ScenarioWrapper(p) => p.parameters_mut(),
            Self::Flow(p) => p.parameters_mut(),
            Self::RbfProfile(p) => p.parameters_mut(),
        }
    }

    pub fn ty(&self) -> &'static str {
        match self {
            Self::Constant(_) => "Constant",
            Self::ControlCurveInterpolated(_) => "ControlCurveInterpolated",
            Self::Aggregated(_) => "Aggregated",
            Self::AggregatedIndex(_) => "AggregatedIndex",
            Self::AsymmetricSwitchIndex(_) => "AsymmetricSwitch",
            Self::ControlCurvePiecewiseInterpolated(_) => "ControlCurvePiecewiseInterpolated",
            Self::ControlCurveIndex(_) => "ControlCurveIndex",
            Self::ControlCurve(_) => "ControlCurve",
            Self::DailyProfile(_) => "DailyProfile",
            Self::IndexedArray(_) => "IndexedArray",
            Self::MonthlyProfile(_) => "MonthlyProfile",
            Self::UniformDrawdownProfile(_) => "UniformDrawdownProfile",
            Self::Max(_) => "Max",
            Self::Min(_) => "Min",
            Self::Division(_) => "Division",
            Self::Negative(_) => "Negative",
            Self::Polynomial1D(_) => "Polynomial1D",
            Self::ParameterThreshold(_) => "ParameterThreshold",
            Self::TablesArray(_) => "TablesArray",
            Self::DataFrame(_) => "DataFrame",
            Self::Deficit(_) => "Deficit",
            Self::DiscountFactor(_) => "DiscountFactor",
            Self::InterpolatedVolume(_) => "InterpolatedVolume",
            Self::InterpolatedFlow(_) => "InterpolatedFlow",
            Self::HydropowerTarget(_) => "HydropowerTarget",
            Self::Storage(_) => "Storage",
            Self::RollingMeanFlowNode(_) => "RollingMeanFlowNode",
            Self::ScenarioWrapper(_) => "ScenarioWrapper",
            Self::Flow(_) => "Flow",
            Self::RbfProfile(_) => "RbfProfile",
        }
    }

    /// Return any external resource paths referenced by this parameter
    pub fn resource_paths(&self) -> Vec<PathBuf> {
        match self {
            CoreParameter::Aggregated(p) => p.resource_paths(),
            CoreParameter::AggregatedIndex(p) => p.resource_paths(),
            CoreParameter::AsymmetricSwitchIndex(p) => p.resource_paths(),
            CoreParameter::Constant(p) => p.resource_paths(),
            CoreParameter::ControlCurvePiecewiseInterpolated(p) => p.resource_paths(),
            CoreParameter::ControlCurveInterpolated(p) => p.resource_paths(),
            CoreParameter::ControlCurveIndex(p) => p.resource_paths(),
            CoreParameter::ControlCurve(p) => p.resource_paths(),
            CoreParameter::DailyProfile(p) => p.resource_paths(),
            CoreParameter::IndexedArray(p) => p.resource_paths(),
            CoreParameter::MonthlyProfile(p) => p.resource_paths(),
            CoreParameter::UniformDrawdownProfile(p) => p.resource_paths(),
            CoreParameter::Max(p) => p.resource_paths(),
            CoreParameter::Min(p) => p.resource_paths(),
            CoreParameter::Division(p) => p.resource_paths(),
            CoreParameter::Negative(p) => p.resource_paths(),
            CoreParameter::Polynomial1D(p) => p.resource_paths(),
            CoreParameter::ParameterThreshold(p) => p.resource_paths(),
            CoreParameter::TablesArray(p) => p.resource_paths(),
            CoreParameter::DataFrame(p) => p.resource_paths(),
            CoreParameter::Deficit(p) => p.resource_paths(),
            CoreParameter::DiscountFactor(p) => p.resource_paths(),
            CoreParameter::InterpolatedVolume(p) => p.resource_paths(),
            CoreParameter::InterpolatedFlow(p) => p.resource_paths(),
            CoreParameter::HydropowerTarget(p) => p.resource_paths(),
            CoreParameter::Storage(p) => p.resource_paths(),
            CoreParameter::RollingMeanFlowNode(p) => p.resource_paths(),
            CoreParameter::ScenarioWrapper(p) => p.resource_paths(),
            CoreParameter::Flow(p) => p.resource_paths(),
            CoreParameter::RbfProfile(p) => p.resource_paths(),
        }
    }

    pub fn resource_paths_recursive(&self) -> Vec<PathBuf> {
        // This parameter's resources
        let mut resource_paths = self.resource_paths();

        for (_, value_type) in self.parameters() {
            match value_type {
                ParameterValueType::Single(value) => resource_paths.extend(value.resource_paths()),
                ParameterValueType::List(values) => {
                    for value in values {
                        resource_paths.extend(value.resource_paths());
                    }
                }
            }
        }

        resource_paths
    }

    pub fn update_resource_paths(&mut self, new_paths: &HashMap<PathBuf, PathBuf>) {
        match self {
            CoreParameter::Aggregated(p) => p.update_resource_paths(new_paths),
            CoreParameter::AggregatedIndex(p) => p.update_resource_paths(new_paths),
            CoreParameter::AsymmetricSwitchIndex(p) => p.update_resource_paths(new_paths),
            CoreParameter::Constant(p) => p.update_resource_paths(new_paths),
            CoreParameter::ControlCurvePiecewiseInterpolated(p) => {
                p.update_resource_paths(new_paths)
            }
            CoreParameter::ControlCurveInterpolated(p) => p.update_resource_paths(new_paths),
            CoreParameter::ControlCurveIndex(p) => p.update_resource_paths(new_paths),
            CoreParameter::ControlCurve(p) => p.update_resource_paths(new_paths),
            CoreParameter::DailyProfile(p) => p.update_resource_paths(new_paths),
            CoreParameter::IndexedArray(p) => p.update_resource_paths(new_paths),
            CoreParameter::MonthlyProfile(p) => p.update_resource_paths(new_paths),
            CoreParameter::UniformDrawdownProfile(p) => p.update_resource_paths(new_paths),
            CoreParameter::Max(p) => p.update_resource_paths(new_paths),
            CoreParameter::Min(p) => p.update_resource_paths(new_paths),
            CoreParameter::Division(p) => p.update_resource_paths(new_paths),
            CoreParameter::Negative(p) => p.update_resource_paths(new_paths),
            CoreParameter::Polynomial1D(p) => p.update_resource_paths(new_paths),
            CoreParameter::ParameterThreshold(p) => p.update_resource_paths(new_paths),
            CoreParameter::TablesArray(p) => p.update_resource_paths(new_paths),
            CoreParameter::DataFrame(p) => p.update_resource_paths(new_paths),
            CoreParameter::Deficit(p) => p.update_resource_paths(new_paths),
            CoreParameter::DiscountFactor(p) => p.update_resource_paths(new_paths),
            CoreParameter::InterpolatedVolume(p) => p.update_resource_paths(new_paths),
            CoreParameter::InterpolatedFlow(p) => p.update_resource_paths(new_paths),
            CoreParameter::HydropowerTarget(p) => p.update_resource_paths(new_paths),
            CoreParameter::Storage(p) => p.update_resource_paths(new_paths),
            CoreParameter::RollingMeanFlowNode(p) => p.update_resource_paths(new_paths),
            CoreParameter::ScenarioWrapper(p) => p.update_resource_paths(new_paths),
            CoreParameter::Flow(p) => p.update_resource_paths(new_paths),
            CoreParameter::RbfProfile(p) => p.update_resource_paths(new_paths),
        }
    }

    pub fn update_resource_paths_recursive(&mut self, new_paths: &HashMap<PathBuf, PathBuf>) {
        // This parameter's resources
        self.update_resource_paths(new_paths);

        for (_, value_type) in self.parameters_mut() {
            match value_type {
                ParameterValueTypeMut::Single(value) => value.update_resource_paths(new_paths),
                ParameterValueTypeMut::List(values) => {
                    for value in values {
                        value.update_resource_paths(new_paths);
                    }
                }
            }
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Parameter {
    Core(CoreParameter),
    Custom(CustomParameter),
}

impl Parameter {
    /// Return the parameter's name if it has one.
    ///
    /// Not all parameters are required to have a name in Pywr. Inline parameters
    /// may be defined without a name.
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Core(p) => p.name(),
            Self::Custom(p) => p.meta.name.as_deref(),
        }
    }

    /// Return a map of attribute to node references.
    pub fn node_references(&self) -> HashMap<&str, &str> {
        match self {
            Self::Core(p) => p.node_references(),
            Self::Custom(_) => HashMap::new(),
        }
    }

    /// Return a map of attribute to parameter values.
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        match self {
            Self::Core(p) => p.parameters(),
            Self::Custom(_) => HashMap::new(),
        }
    }

    /// Return the type of the parameter
    pub fn ty(&self) -> &str {
        match self {
            Self::Core(p) => p.ty(),
            Self::Custom(p) => p.ty.as_str(),
        }
    }

    /// Return any external resource paths referenced by this parameter
    pub fn resource_paths(&self) -> Vec<PathBuf> {
        match self {
            Self::Core(p) => p.resource_paths_recursive(),
            // It is not possible to determine external resources for custom parameters
            Self::Custom(_) => Vec::new(),
        }
    }

    /// Update any external resource paths referenced by this parameter if they are
    /// in the provided map.
    pub fn update_resource_paths(&mut self, new_paths: &HashMap<PathBuf, PathBuf>) {
        match self {
            Self::Core(p) => p.update_resource_paths_recursive(new_paths),
            Self::Custom(_) => {}
        }
    }

    /// Return true if this is a customer parameter.
    pub fn is_custom(&self) -> bool {
        match self {
            Self::Core(_) => false,
            Self::Custom(_) => true,
        }
    }
}

#[derive(Clone)]
pub struct ParameterVec(Vec<Parameter>);

impl ParameterVec {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl IntoIterator for ParameterVec {
    type Item = Parameter;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for ParameterVec {
    type Target = Vec<Parameter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ParameterVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct PywrParameterMapVisitor {
    marker: PhantomData<fn() -> ParameterVec>,
}

impl PywrParameterMapVisitor {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

fn remove_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    match s.strip_suffix(suffix) {
        Some(s) => s,
        None => s,
    }
}

impl<'de> Visitor<'de> for PywrParameterMapVisitor {
    type Value = ParameterVec;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a valid Pywr parameter definition")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = ParameterVec::with_capacity(access.size_hint().unwrap_or(0));

        #[derive(serde::Deserialize, Debug)]
        struct Helper {
            pub comment: Option<String>,
            #[serde(rename = "type")]
            pub ty: String,
            #[serde(flatten)]
            attributes: HashMap<String, Value>,
        }

        // While there are entries remaining in the input, add them into our map.
        while let Some((name, value)) = access.next_entry::<String, Helper>()? {
            let ty = value.ty.to_lowercase();
            let ty = remove_suffix(&ty, "parameter");

            // Try to deserialize the parameter as a core Pywr parameter.
            // If that fails assume it is a custom parameter
            // First we need to create a copy of the attributes containing both the name and type
            // keys
            let mut py_attributes = value.attributes.clone();
            py_attributes.insert("name".to_string(), Value::String(name.clone()));
            py_attributes.insert("type".to_string(), Value::String(ty.to_string()));

            let p =
                match CoreParameter::deserialize(MapDeserializer::new(py_attributes.into_iter())) {
                    Ok(p) => Parameter::Core(p),
                    // Deserializing a core parameter failed; deserialize as a custom parameter
                    Err(_) => Parameter::Custom(CustomParameter {
                        meta: ParameterMeta {
                            name: Some(name),
                            comment: value.comment,
                        },
                        ty: value.ty,
                        attributes: value.attributes,
                    }),
                };

            map.push(p);
        }

        Ok(map)
    }
}

impl<'de> Deserialize<'de> for ParameterVec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(PywrParameterMapVisitor::new())
    }
}

impl Serialize for ParameterVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;

        for p in &self.0 {
            let name = p
                .name()
                .ok_or(S::Error::custom("Parameter name is required"))?;
            map.serialize_entry(name, p)?;
        }
        map.end()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ParameterValue {
    Constant(f64),
    Reference(String),
    Inline(Box<Parameter>),
    Table(TableDataRef),
}

pub type ParameterValues = Vec<ParameterValue>;

pub enum ParameterValueType<'a> {
    Single(&'a ParameterValue),
    List(&'a ParameterValues),
}

impl<'a> From<&'a ParameterValue> for ParameterValueType<'a> {
    fn from(v: &'a ParameterValue) -> Self {
        Self::Single(v)
    }
}

impl<'a> From<&'a ParameterValues> for ParameterValueType<'a> {
    fn from(v: &'a ParameterValues) -> Self {
        Self::List(v)
    }
}

pub enum ParameterValueTypeMut<'a> {
    Single(&'a mut ParameterValue),
    List(&'a mut ParameterValues),
}

impl<'a> From<&'a mut ParameterValue> for ParameterValueTypeMut<'a> {
    fn from(v: &'a mut ParameterValue) -> Self {
        Self::Single(v)
    }
}

impl<'a> From<&'a mut ParameterValues> for ParameterValueTypeMut<'a> {
    fn from(v: &'a mut ParameterValues) -> Self {
        Self::List(v)
    }
}

impl ParameterValue {
    pub fn resource_paths(&self) -> Vec<PathBuf> {
        match self {
            ParameterValue::Constant(_) => Vec::new(),
            ParameterValue::Reference(_) => Vec::new(),
            ParameterValue::Inline(p) => p.resource_paths(),
            ParameterValue::Table(_) => Vec::new(),
        }
    }

    pub fn update_resource_paths(&mut self, new_paths: &HashMap<PathBuf, PathBuf>) {
        match self {
            ParameterValue::Constant(_) => {}
            ParameterValue::Reference(_) => {}
            ParameterValue::Inline(p) => p.update_resource_paths(new_paths),
            ParameterValue::Table(_) => {}
        };
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ExternalDataRef {
    pub url: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<TableIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<TableIndex>,
    #[serde(flatten)]
    pub attributes: HashMap<String, Value>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum TableIndex {
    Single(TableIndexEntry),
    Multi(Vec<TableIndexEntry>),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum TableIndexEntry {
    Name(String),
    Index(usize),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct TableDataRef {
    pub table: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<TableIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<TableIndex>,
}

#[cfg(test)]
mod tests {
    use crate::parameters::{CoreParameter, Parameter, ParameterValue};

    /// Test loading a DailyProfile with a tables definition.
    #[test]
    fn test_daily_profile_from_table() {
        let data = r#"
        {
            "type": "dailyprofile",
            "table": "my-table",
            "column": ["A", "B"]
        }
        "#;

        let p: Parameter =
            serde_json::from_str(data).expect("Failed to create Parameter from expected data!");

        assert_eq!("DailyProfile", p.ty());
    }

    /// Test deserializing inline profiles with values from a table.
    #[test]
    fn test_indexed_array_with_profile_tables() {
        let data = r#"
        {
            "type": "indexedarrayparameter",
            "index_parameter": "Demand Saving - DPs Index",
            "parameters": [
                {"type": "dailyprofile", "table": "my-table", "column": ["A", "B"]},
                {"type": "dailyprofile", "table": "my-table", "column": ["A", "B"]},
                {"type": "dailyprofile", "table": "my-table", "column": ["A", "B"]}
            ]
        }
        "#;

        let p: Parameter =
            serde_json::from_str(data).expect("Failed to create Parameter from expected data!");

        assert_eq!("IndexedArray", p.ty());

        if let Parameter::Core(p) = &p {
            if let CoreParameter::IndexedArray(p) = p {
                if let ParameterValue::Inline(daily_profile) = &p.parameters[0] {
                    assert_eq!("DailyProfile", daily_profile.ty())
                } else {
                    panic!("Expected an inline parameter found: {:?}", &p.parameters[0]);
                }
            } else {
                panic!("Expected an IndexedArray parameter.")
            }
        } else {
            panic!("Expected a CoreParameter.")
        }
    }
}
