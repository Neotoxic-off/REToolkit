use std::path::Path;

pub struct Content;

impl Content {
    pub fn is_directory(path: &String) -> bool {
        return Path::new(path).exists() && Path::new(path).is_dir();
    }
    
    pub fn is_file(path: &String) -> bool {
        return Path::new(path).exists() && Path::new(path).is_file();
    }
}

