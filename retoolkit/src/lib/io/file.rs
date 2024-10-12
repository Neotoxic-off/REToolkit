use std::io::prelude::*;
use std::path::{Path};
use std::collections::HashMap;
use std::error::Error;
use serde::{Serialize, de::DeserializeOwned};
use serde_json;

pub struct File;

impl File {
    pub fn exists(path: &String) -> bool {
        Path::new(path).exists()
    }

    pub fn is_file(path: &String) -> bool {
        File::exists(path) && Path::new(path).is_file()
    }

    pub fn write(path: &str, content: &[u8]) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
    
        file.write_all(content)?;
    
        Ok(())
    }
    
    pub fn open(path: &str) -> std::io::Result<Vec<u8>> {
        let mut file: std::fs::File = std::fs::File::open(path)?;
        let mut contents: Vec<u8> = Vec::new();

        file.read_to_end(&mut contents)?;
    
        Ok(contents)
    }

    pub fn write_json<T>(path: &str, content: &T) -> Result<(), Box<dyn std::error::Error>> where T: Serialize, {
        let json_data = serde_json::to_string_pretty(&content)?;
        File::write(path, json_data.as_bytes())?;
        Ok(())
    }

    pub fn open_json<T>(path: &str) -> Result<T, Box<dyn std::error::Error>> where T: DeserializeOwned, {
        let mut file = std::fs::File::open(path)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;

        let content: T = serde_json::from_str(&json_data)?;
        Ok(content)
    }
}
