use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::structs;

pub static EXTENSIONS: Lazy<HashMap<&'static str, structs::AssetKind>> = Lazy::new(|| {
    HashMap::from([
        (".o", structs::AssetKind::Object),
        (".png", structs::AssetKind::Image),
        (".jpg", structs::AssetKind::Image),
        (".jpeg", structs::AssetKind::Image),
        (".gif", structs::AssetKind::Image),
        (".mov", structs::AssetKind::Video),
        (".mkv", structs::AssetKind::Video),
        (".mp4", structs::AssetKind::Video),
        (".avi", structs::AssetKind::Video),
        (".zip", structs::AssetKind::Archive),
        (".rar", structs::AssetKind::Archive),
        (".7z", structs::AssetKind::Archive),
        (".dll", structs::AssetKind::Binary),
        (".exe", structs::AssetKind::Binary),
        (".so", structs::AssetKind::Binary),
        (".lib", structs::AssetKind::Binary),
        (".bin", structs::AssetKind::Binary),
        (".bat", structs::AssetKind::Script),
        (".vbs", structs::AssetKind::Script),
        (".sh", structs::AssetKind::Script),
        (".json", structs::AssetKind::Configuration),
        (".ini", structs::AssetKind::Configuration),
        (".xml", structs::AssetKind::Configuration)
    ])
});
