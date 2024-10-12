use std::collections::HashMap;
use std::fmt;

use log::warn;
use log::{info, error};

use crate::lib;
use crate::structs;
use crate::constants;

pub fn load(arguments: &structs::Arguments) -> () {
    let mut assets: Vec<structs::Asset> = Vec::new();

    if is_valid_directory(&arguments.directory) == true {
        assets = index_assets(&arguments.directory);
        display_assets_information(assets.clone());
        compress_assets(assets);
    }
}

fn compress_assets(assets: Vec<structs::Asset>) -> std::io::Result<()> {
    let mut input: String;
    let mut output: String;

    for asset in assets.iter() {
        input = asset.path.clone();
        output = format!("{}.rcp", asset.path);
        warn!("[{:?}] compressing: {}", asset.kind, input);
        (asset.module)(&input, &output)?
    }

    Ok(())
}

fn detect_module(file_name: &String) -> constants::CompressionFunction {
    for (extension, asset_module) in constants::EXTENSIONS_MODULE.iter() {
        if file_name.to_lowercase().ends_with(extension) {
            return asset_module.clone();
        }
    }

    lib::compression::Rle::compress
}

fn detect_kind(file_name: &String) -> structs::AssetKind {
    for (extension, asset_kind) in constants::EXTENSIONS_KIND.iter() {
        if file_name.to_lowercase().ends_with(extension) {
            return asset_kind.clone();
        }
    }

    structs::AssetKind::Other
}

fn build_asset(file_name: &String, file_path: &String) -> structs::Asset {
    structs::Asset
    {
        kind: detect_kind(file_name),
        name: file_name.to_owned(),
        path: file_path.to_owned(),
        module: detect_module(file_name)
    }
}

fn index_assets(directory: &String) -> Vec<structs::Asset> {
    let content: HashMap<String, String> = lib::io::Directory::get_directory(directory, true);
    let mut assets: Vec<structs::Asset> = Vec::new();

    for (name, path) in content.iter() {
        assets.push(build_asset(name, path));
    }

    assets
}

fn display_assets_information(assets: Vec<structs::Asset>) -> () {
    let mut asset_kind_counts: HashMap<structs::AssetKind, u32> = HashMap::new();

    for asset in assets {
        *asset_kind_counts.entry(asset.kind).or_insert(0) += 1;
    }

    for (kind, count) in asset_kind_counts {
        info!("[{:?}]: {}", kind, count);
    }
}

fn is_valid_directory(directory: &String) -> bool {
    if lib::io::Directory::exists(directory) == true {
        if lib::io::Directory::is_directory(directory) == true {
            info!("directory set to '{}'", directory);
            return true;
        } else {
            error!("'{}' is not a directory", directory);
        }
    } else {
        error!("'{}' directory not found", directory);
    }

    false
}