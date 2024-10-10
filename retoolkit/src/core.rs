use std::collections::HashMap;

use log::{info, error};

use crate::lib;
use crate::structs;
use crate::constants;

pub fn load(arguments: &structs::Arguments) -> () {
    if is_valid_directory(&arguments.directory) == true {
        index_assets(&arguments.directory);
    }
}

fn detect_asset(file_name: &String) -> structs::AssetKind {
    for (extension, asset_kind) in constants::EXTENSIONS.iter() {
        if file_name.to_lowercase().ends_with(extension) {
            return asset_kind.clone();
        }
    }

    structs::AssetKind::Other
}

fn index_assets(directory: &String) -> Vec<structs::Asset> {
    let content: HashMap<String, String> = lib::io::Directory::get_directory(directory, true);
    let mut assets: Vec<structs::Asset> = Vec::new();

    for (name, path) in content.iter() {
        let item: structs::Asset = structs::Asset {
            path: path.to_string(),
            kind: detect_asset(name),
        };

        assets.push(item.clone());

        info!("AssetKind: ({:?}) {}", &item.kind, name);
    }

    assets
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


