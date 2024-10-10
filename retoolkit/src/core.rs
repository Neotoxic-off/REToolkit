use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

use log::{info, error, warn};

use crate::lib;
use crate::structs;

pub fn load(arguments: &structs::Arguments) -> () {
    if is_valid_directory(&arguments.directory) == true {
        index_assets(&arguments.directory);
    }
}

fn detect_asset(file_name: String, _file_path: String) -> structs::AssetKind {
    let extensions: HashMap<&str, structs::AssetKind> = HashMap::from([
        (".png", structs::AssetKind::Image)
    ]);

    for (extension, asset_kind) in &extensions {
        if file_name.ends_with(extension) {
            return asset_kind.clone();
        }
    }

    structs::AssetKind::Other
}

fn index_assets(directory: &String) -> Vec<structs::Asset> {
    let content: HashMap<String, String> = lib::io::Directory::get_directory(directory, true);
    let mut assets: Vec<structs::Asset> = Vec::new();

    for element in content.iter() {

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


