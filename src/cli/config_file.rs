use std::path::Path;
use std::convert::AsRef;
use std::io::prelude::*;
use std::io;
use std::fs::File;

pub enum HandlerMethod {
    Log,
    File,
}

pub struct SwerveConfig {
    pub field_handling: HandlerMethod,
    pub file_handling: HandlerMethod,
}

impl SwerveConfig {
    pub fn from_file<P>(path: P) -> io::Result<SwerveConfig> where P: AsRef<Path> {
//        let mut file = File::open(path)?;
//        let mut buffer = String::new();
//        file.read_to_string(buffer)?;

        Ok(SwerveConfig {
            field_handling: HandlerMethod::Log,
            file_handling: HandlerMethod::Log,
        })
    }
}