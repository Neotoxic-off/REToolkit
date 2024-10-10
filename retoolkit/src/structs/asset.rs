#[derive(Clone)]
pub enum AssetKind {
    Image,
    Video,
    Audio,
    Archive,
    Other
}

pub struct Asset {
    kind: AssetKind,
    path: String
}
