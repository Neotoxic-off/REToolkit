use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::{lib, structs};

pub static COMPRESSION_EXTENSION: &str = ".rtc";

pub type CompressionFunction = fn(&str, &str) -> std::io::Result<()>;

pub static EXTENSIONS_MODULE: Lazy<HashMap<&'static str, CompressionFunction>> = Lazy::new(|| {
    HashMap::from([
        (".o", lib::compression::Lz77::compress as CompressionFunction),
        (".png", lib::compression::Image::png),
        (".jpg", lib::compression::Rle::compress),
        (".jpeg", lib::compression::Rle::compress),
        (".gif", lib::compression::Rle::compress),
        (".mov", lib::compression::Rle::compress),
        (".mkv", lib::compression::Rle::compress),
        (".mp4", lib::compression::Rle::compress),
        (".avi", lib::compression::Rle::compress),
        (".dll", lib::compression::Lz77::compress),
        (".exe", lib::compression::Lz77::compress),
        (".so", lib::compression::Lz77::compress),
        (".lib", lib::compression::Lz77::compress),
        (".bin", lib::compression::Lz77::compress),
        (".bat", lib::compression::Rle::compress),
        (".vbs", lib::compression::Rle::compress),
        (".sh", lib::compression::Rle::compress),
        (".json", lib::compression::Rle::compress),
        (".ini", lib::compression::Rle::compress),
        (".xml", lib::compression::Rle::compress)
    ])
});

pub static EXTENSIONS_KIND: Lazy<HashMap<&'static str, structs::AssetKind>> = Lazy::new(|| {
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
