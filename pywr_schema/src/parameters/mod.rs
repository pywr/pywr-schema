mod aggregated;
mod asymmetric_switch;
mod control_curves;
mod core;
mod indexed_array;
mod polynomial;
mod profiles;
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
pub use crate::parameters::core::{ConstantParameter, MaxParameter, NegativeParameter};
pub use crate::parameters::indexed_array::IndexedArrayParameter;
pub use crate::parameters::polynomial::Polynomial1DParameter;
pub use crate::parameters::profiles::{
    DailyProfileParameter, MonthInterpDay, MonthlyProfileParameter, UniformDrawdownProfileParameter,
};
pub use crate::parameters::tables::TablesArrayParameter;
pub use crate::parameters::thresholds::{ParameterThresholdParameter, Predicate};
use serde::de::value::MapDeserializer;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ParameterMeta {
    pub name: Option<String>,
    pub comment: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
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
#[derive(serde::Deserialize, serde::Serialize, Debug)]
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
            Self::Negative(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Polynomial1D(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::ParameterThreshold(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::TablesArray(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
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
            Self::Negative(p) => p.node_references(),
            Self::Polynomial1D(p) => p.node_references(),
            Self::ParameterThreshold(p) => p.node_references(),
            Self::TablesArray(p) => p.node_references(),
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
            Self::Max(p) => p.parameters(),
            Self::Negative(p) => p.parameters(),
            Self::Polynomial1D(p) => p.parameters(),
            Self::ParameterThreshold(p) => p.parameters(),
            Self::TablesArray(p) => p.parameters(),
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
            Self::Negative(_) => "Negative",
            Self::Polynomial1D(_) => "Polynomial1D",
            Self::ParameterThreshold(_) => "ParameterThreshold",
            Self::TablesArray(_) => "TablesArray",
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
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
}

pub struct ParameterVec(Vec<Parameter>);

impl ParameterVec {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn into_iter(self) -> IntoIter<Parameter> {
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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ExternalDataRef {
    pub url: String,
    pub column: Option<TableIndex>,
    pub index: Option<TableIndex>,
    #[serde(flatten)]
    pub attributes: HashMap<String, Value>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(untagged)]
pub enum TableIndex {
    Single(String),
    Multi(Vec<String>),
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TableDataRef {
    pub table: String,
    pub column: Option<TableIndex>,
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
