use std::fs;
use std::io::prelude::*;
use std::path::{PathBuf, Path};

pub struct File;

impl File {
    pub fn exists(path: &String) -> bool {
        Path::new(path).exists()
    }

    pub fn is_file(path: &String) -> bool {
        File::exists(path) && Path::new(path).is_file()
    }

    pub fn write(path: String, content: &[u8]) -> std::io::Result<()> {
        let mut file = std::fs::File::create(&path)?;
    
        file.write_all(content)?;
    
        Ok(())
    }
    
    pub fn open(path: String) -> std::io::Result<()> {
        let mut file = std::fs::File::open(&path)?;
        let mut contents = String::new();
    
        file.read_to_string(&mut contents)?;
    
        Ok(())
    }
}
