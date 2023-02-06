mod break_link;
mod core;
mod loss_link;
mod piecewise_link;
mod multi_split;
mod river_gauge;
mod river_split;
mod river_split_with_gauge;
mod virtual_storage;
mod delay_node;

pub use crate::nodes::core::{
    AggregatedNode, AggregatedStorageNode, CatchmentNode, InputNode, LinkNode, OutputNode,
    ReservoirNode, StorageNode,
};
use crate::parameters::ParameterValue;
pub use break_link::BreakLinkNode;
pub use loss_link::LossLinkNode;
pub use piecewise_link::PiecewiseLinkNode;
pub use multi_split::MultiSplitLinkNode;
pub use river_gauge::RiverGaugeNode;
pub use delay_node::DelayNode;
pub use river_split::RiverSplitNode;
pub use crate::nodes::river_split_with_gauge::RiverSplitWithGaugeNode;
use serde_json::Value;
use std::collections::HashMap;
pub use virtual_storage::{
    AnnualVirtualStorageNode, MonthlyVirtualStorageNode, RollingVirtualStorageNode,
    SeasonalVirtualStorageNode, VirtualStorageNode,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NodePosition {
    pub schematic: Option<(f32, f32)>,
    pub geographic: Option<(f32, f32)>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NodeMeta {
    pub name: String,
    pub comment: Option<String>,
    pub position: Option<NodePosition>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CustomNode {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(flatten)]
    pub meta: NodeMeta,
    #[serde(flatten)]
    pub attributes: HashMap<String, Value>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum CoreNode {
    #[serde(alias = "input")]
    Input(InputNode),
    #[serde(alias = "link")]
    Link(LinkNode),
    #[serde(alias = "output")]
    Output(OutputNode),
    #[serde(alias = "storage")]
    Storage(StorageNode),
    #[serde(alias = "reservoir")]
    Reservoir(ReservoirNode),
    #[serde(alias = "catchment")]
    Catchment(CatchmentNode),
    #[serde(alias = "rivergauge")]
    RiverGauge(RiverGaugeNode),
    #[serde(alias = "losslink")]
    LossLink(LossLinkNode),
    #[serde(alias = "river")]
    River(LinkNode), // TODO make this its own type.
    #[serde(alias = "piecewiselink")]
    PiecewiseLink(PiecewiseLinkNode),
    #[serde(alias = "multisplitlink")]
    MultiSplitLink(MultiSplitLinkNode),
    #[serde(alias = "breaklink")]
    BreakLink(BreakLinkNode),
    #[serde(alias = "delaynode")]
    Delay(DelayNode),
    #[serde(alias = "riversplit")]
    RiverSplit(RiverSplitNode),
    #[serde(alias = "riversplitwithgauge")]
    RiverSplitWithGauge(RiverSplitWithGaugeNode),
    #[serde(alias = "aggregatednode")]
    Aggregated(AggregatedNode),
    #[serde(alias = "aggregatedstorage")]
    AggregatedStorage(AggregatedStorageNode),
    #[serde(alias = "virtualstorage")]
    VirtualStorage(VirtualStorageNode),
    #[serde(alias = "annualvirtualstorage")]
    AnnualVirtualStorage(AnnualVirtualStorageNode),
    #[serde(alias = "monthlyvirtualstorage")]
    MonthlyVirtualStorage(MonthlyVirtualStorageNode),
    #[serde(alias = "seasonalvirtualstorage")]
    SeasonalVirtualStorage(SeasonalVirtualStorageNode),
    #[serde(alias = "rollingvirtualstorage")]
    RollingVirtualStorage(RollingVirtualStorageNode),
}

impl CoreNode {
    pub fn name(&self) -> &str {
        self.meta().name.as_str()
    }

    pub fn position(&self) -> Option<&NodePosition> {
        self.meta().position.as_ref()
    }

    pub fn node_type(&self) -> &str {
        match self {
            CoreNode::Input(_) => "input",
            CoreNode::Link(_) => "link",
            CoreNode::Output(_) => "output",
            CoreNode::Storage(_) => "storage",
            CoreNode::Reservoir(_) => "reservoir",
            CoreNode::Catchment(_) => "catchment",
            CoreNode::RiverGauge(_) => "rivergauge",
            CoreNode::LossLink(_) => "losslink",
            CoreNode::PiecewiseLink(_) => "piecewiselink",
            CoreNode::MultiSplitLink(_) => "multisplitlink",
            CoreNode::BreakLink(_) => "breaklink",
            CoreNode::Delay(_) => "delaynode",
            CoreNode::River(_) => "river",
            CoreNode::RiverSplit(_) => "riversplit",
            CoreNode::RiverSplitWithGauge(_) => "riversplitwithgauge",
            CoreNode::Aggregated(_) => "aggregated",
            CoreNode::AggregatedStorage(_) => "aggregatedstorage",
            CoreNode::VirtualStorage(_) => "virtualstorage",
            CoreNode::AnnualVirtualStorage(_) => "annualvirtualstorage",
            CoreNode::MonthlyVirtualStorage(_) => "monthlyvirtualstorage",
            CoreNode::SeasonalVirtualStorage(_) => "seasonalvirtualstorage",
            CoreNode::RollingVirtualStorage(_) => "rollingvirtualstorage"
        }
    }

    pub fn meta(&self) -> &NodeMeta {
        match self {
            CoreNode::Input(n) => &n.meta,
            CoreNode::Link(n) => &n.meta,
            CoreNode::Output(n) => &n.meta,
            CoreNode::Storage(n) => &n.meta,
            CoreNode::Reservoir(n) => &n.meta,
            CoreNode::Catchment(n) => &n.meta,
            CoreNode::RiverGauge(n) => &n.meta,
            CoreNode::LossLink(n) => &n.meta,
            CoreNode::PiecewiseLink(n) => &n.meta,
            CoreNode::MultiSplitLink(n) => &n.meta,
            CoreNode::BreakLink(n) => &n.meta,
            CoreNode::Delay(n) => &n.meta,
            CoreNode::River(n) => &n.meta,
            CoreNode::RiverSplit(n) => &n.meta,
            CoreNode::RiverSplitWithGauge(n) => &n.meta,
            CoreNode::Aggregated(n) => &n.meta,
            CoreNode::AggregatedStorage(n) => &n.meta,
            CoreNode::VirtualStorage(n) => &n.meta,
            CoreNode::AnnualVirtualStorage(n) => &n.meta,
            CoreNode::MonthlyVirtualStorage(n) => &n.meta,
            CoreNode::SeasonalVirtualStorage(n) => &n.meta,
            CoreNode::RollingVirtualStorage(n) => &n.meta,
        }
    }

    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        match self {
            CoreNode::Input(n) => n.parameters(),
            CoreNode::Link(n) => n.parameters(),
            CoreNode::Output(n) => n.parameters(),
            CoreNode::Storage(n) => n.parameters(),
            CoreNode::Reservoir(n) => n.parameters(),
            CoreNode::Catchment(n) => n.parameters(),
            CoreNode::RiverGauge(n) => n.parameters(),
            CoreNode::LossLink(n) => n.parameters(),
            CoreNode::PiecewiseLink(n) => n.parameters(),
            CoreNode::MultiSplitLink(n) => n.parameters(),
            CoreNode::BreakLink(n) => n.parameters(),
            CoreNode::Delay(n) => n.parameters(),
            CoreNode::River(n) => n.parameters(),
            CoreNode::RiverSplit(n) => n.parameters(),
            CoreNode::RiverSplitWithGauge(n) => n.parameters(),
            CoreNode::Aggregated(n) => n.parameters(),
            CoreNode::AggregatedStorage(n) => n.parameters(),
            CoreNode::VirtualStorage(n) => n.parameters(),
            CoreNode::AnnualVirtualStorage(n) => n.parameters(),
            CoreNode::MonthlyVirtualStorage(n) => n.parameters(),
            CoreNode::SeasonalVirtualStorage(n) => n.parameters(),
            CoreNode::RollingVirtualStorage(n) => n.parameters(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Node {
    Core(Box<CoreNode>),
    Custom(CustomNode),
}

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Node::Core(n) => n.name(),
            Node::Custom(n) => n.meta.name.as_str(),
        }
    }

    pub fn position(&self) -> Option<&NodePosition> {
        match self {
            Node::Core(n) => n.position(),
            Node::Custom(n) => n.meta.position.as_ref(),
        }
    }

    pub fn node_type(&self) -> &str {
        match self {
            Node::Core(n) => n.node_type(),
            Node::Custom(n) => n.ty.as_str(),
        }
    }

    pub fn parameters(&self) -> HashMap<&str, &ParameterValue> {
        match self {
            Node::Core(n) => n.parameters(),
            Node::Custom(_) => HashMap::new(),
        }
    }
}
