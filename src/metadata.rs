use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Pertains to [CHIP-0007](https://github.com/Chia-Network/chips/blob/main/CHIPs/chip-0007.md) off-chain metadata for NFTs.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Chip0007Metadata {
    pub format: String,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minting_tool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_number: Option<NonZeroUsize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_total: Option<NonZeroUsize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<NftAttribute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<Collection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NftAttribute {
    pub trait_type: AttributeValue,
    pub value: AttributeValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    Integer(usize),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<CollectionAttribute>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionAttribute {
    #[serde(rename = "type")]
    pub kind: AttributeValue,
    pub value: AttributeValue,
}
