mod aggregated;
mod control_curves;
mod core;
mod indexed_array;
mod polynomial;
mod profiles;
mod tables;

use crate::parameters::aggregated::{AggregatedIndexParameter, AggregatedParameter};
use crate::parameters::control_curves::{
    ControlCurveIndexParameter, ControlCurveInterpolatedParameter, ControlCurveParameter,
    ControlCurvePiecewiseInterpolatedParameter,
};
use crate::parameters::core::{ConstantParameter, MaxParameter, NegativeParameter};
use crate::parameters::indexed_array::IndexedArrayParameter;
use crate::parameters::polynomial::Polynomial1DParameter;
use crate::parameters::profiles::{DailyProfileParameter, MonthlyProfileParameter};
use crate::parameters::tables::TablesArrayParameter;
use serde::de::value::MapDeserializer;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ParameterMeta {
    pub name: Option<String>,
    pub comment: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CustomParameter {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(flatten)]
    pub meta: ParameterMeta,
    #[serde(flatten)]
    pub attributes: HashMap<String, Value>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CoreParameter {
    Aggregated(AggregatedParameter),
    AggregatedIndex(AggregatedIndexParameter),
    Constant(ConstantParameter),
    ControlCurvePiecewiseInterpolated(ControlCurvePiecewiseInterpolatedParameter),
    ControlCurveInterpolated(ControlCurveInterpolatedParameter),
    ControlCurveIndex(ControlCurveIndexParameter),
    ControlCurve(ControlCurveParameter),
    DailyProfile(DailyProfileParameter),
    IndexedArray(IndexedArrayParameter),
    MonthlyProfile(MonthlyProfileParameter),
    Max(MaxParameter),
    Negative(NegativeParameter),
    Polynomial1D(Polynomial1DParameter),
    TablesArray(TablesArrayParameter),
}

pub struct NodeReference {
    pub attribute: String,
    pub node: String,
}

impl CoreParameter {
    fn name(&self) -> Option<&str> {
        match self {
            Self::Constant(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::ControlCurveInterpolated(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Aggregated(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::AggregatedIndex(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::ControlCurvePiecewiseInterpolated(p) => {
                p.meta.as_ref().and_then(|m| m.name.as_deref())
            }
            Self::ControlCurveIndex(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::ControlCurve(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::DailyProfile(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::IndexedArray(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::MonthlyProfile(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Max(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Negative(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::Polynomial1D(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
            Self::TablesArray(p) => p.meta.as_ref().and_then(|m| m.name.as_deref()),
        }
    }

    fn node_references(&self) -> Vec<NodeReference> {
        match self {
            Self::Constant(p) => p.node_references(),
            Self::ControlCurveInterpolated(p) => p.node_references(),
            Self::Aggregated(p) => p.node_references(),
            Self::AggregatedIndex(p) => p.node_references(),
            Self::ControlCurvePiecewiseInterpolated(p) => p.node_references(),
            Self::ControlCurveIndex(p) => p.node_references(),
            Self::ControlCurve(p) => p.node_references(),
            Self::DailyProfile(p) => p.node_references(),
            Self::IndexedArray(p) => p.node_references(),
            Self::MonthlyProfile(p) => p.node_references(),
            Self::Max(p) => p.node_references(),
            Self::Negative(p) => p.node_references(),
            Self::Polynomial1D(p) => p.node_references(),
            Self::TablesArray(p) => p.node_references(),
        }
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        match self {
            Self::Constant(p) => p.parameters(),
            Self::ControlCurveInterpolated(p) => p.parameters(),
            Self::Aggregated(p) => p.parameters(),
            Self::AggregatedIndex(p) => p.parameters(),
            Self::ControlCurvePiecewiseInterpolated(p) => p.parameters(),
            Self::ControlCurveIndex(p) => p.parameters(),
            Self::ControlCurve(p) => p.parameters(),
            Self::DailyProfile(p) => p.parameters(),
            Self::IndexedArray(p) => p.parameters(),
            Self::MonthlyProfile(p) => p.parameters(),
            Self::Max(p) => p.parameters(),
            Self::Negative(p) => p.parameters(),
            Self::Polynomial1D(p) => p.parameters(),
            Self::TablesArray(p) => p.parameters(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Parameter {
    Core(CoreParameter),
    Custom(CustomParameter),
}

impl Parameter {
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Core(p) => p.name(),
            Self::Custom(p) => p.meta.name.as_deref(),
        }
    }

    pub fn node_references(&self) -> Vec<NodeReference> {
        match self {
            Self::Core(p) => p.node_references(),
            Self::Custom(_) => vec![],
        }
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType> {
        match self {
            Self::Core(n) => n.parameters(),
            Self::Custom(_) => HashMap::new(),
        }
    }
}

pub struct ParameterVec(Vec<Parameter>);

impl ParameterVec {
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
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
                    Err(e) => {
                        println!("Failed to deserialize {:#?} with error: {}", value, e);
                        Parameter::Custom(CustomParameter {
                            meta: ParameterMeta {
                                name: Some(name),
                                comment: value.comment,
                            },
                            ty: value.ty,
                            attributes: value.attributes,
                        })
                    }
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

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum ParameterValue {
    Constant(f64),
    Reference(String),
    Table(TableDataRef),
    Inline(Box<Parameter>),
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ExternalDataRef {
    url: String,
    column: Option<String>,
    index: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TableDataRef {
    table: String,
    column: Option<String>,
    index: Option<String>,
}
