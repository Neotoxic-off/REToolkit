use std::collections::HashMap;
use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AssetKind {
    Image,
    Video,
    Audio,
    Archive,
    Object,
    Binary,
    Script,
    Configuration,
    Other
}

#[derive(Clone, Debug)]
pub struct Asset {
    pub kind: AssetKind,
    pub name: String,
    pub path: String,
    pub module: constants::CompressionFunction
}
