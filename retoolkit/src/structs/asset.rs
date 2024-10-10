use std::collections::HashMap;

#[derive(Clone, Debug)]
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
    pub path: String
}
