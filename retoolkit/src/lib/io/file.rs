use std::io::prelude::*;

pub struct File;

impl File {
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
