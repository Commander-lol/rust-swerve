use std::path::Path;
use std::convert::AsRef;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::default::Default;
use serde::{Deserialize, Deserializer, de};
use std::fmt;
use serde_yaml as yaml;
use std::collections::HashMap;
use cli;

#[derive(Debug, Copy, Clone)]
pub enum HandlerMethod {
    Log,
    File,
}

struct HandlerMethodVisitor;
impl <'de>de::Visitor<'de> for HandlerMethodVisitor {
    type Value = HandlerMethod;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string with value 'Log' or 'File'")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
        E: de::Error, {
        match v {
            "Log" => Ok(HandlerMethod::Log),
            "File" => Ok(HandlerMethod::File),
            _ => Err(de::Error::unknown_variant(v, &["Log", "File"])),
        }
    }
}

impl <'de>Deserialize<'de> for HandlerMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_str(HandlerMethodVisitor)
    }
}

impl Default for HandlerMethod {
    fn default() -> Self {
        HandlerMethod::Log
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SwerveConfig {
    #[serde(default)]
    pub field_handling: HandlerMethod,
    #[serde(default)]
    pub file_handling: HandlerMethod,
    #[serde(default)]
    pub server: ServerOptions,
    #[serde(default="get_empty_routes")]
    pub routes: Vec<cli::RouteHandler>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerOptions {
    #[serde(default="get_default_port")]
    pub port: u16,
    #[serde(default="get_default_threads")]
    pub threads: u16,
    #[serde(default="get_default_address")]
    pub address: String,
    #[serde(default="get_default_quiet_attr")]
    pub quiet: bool,
    #[serde(default="get_default_index_attr")]
    pub no_index: bool,
}

fn get_default_port() -> u16 { 8200 }
fn get_default_threads() -> u16 { 32}
fn get_default_address() -> String { String::from("localhost") }
fn get_default_quiet_attr() -> bool { false }
fn get_default_index_attr() -> bool { false }

fn get_empty_routes() -> Vec<cli::RouteHandler> { vec![] }

impl Default for SwerveConfig {
    fn default() -> Self {
        SwerveConfig {
            field_handling: HandlerMethod::Log,
            file_handling: HandlerMethod::Log,
            server: ServerOptions::default(),
            routes: get_empty_routes(),
        }
    }
}

impl Default for ServerOptions {
    fn default() -> Self {
        ServerOptions {
            port: get_default_port(),
            threads: get_default_threads(),
            address: get_default_address(),
            quiet: get_default_quiet_attr(),
            no_index: get_default_index_attr(),
        }
    }
}

impl SwerveConfig {
    pub fn from_file<P>(path: P) -> io::Result<SwerveConfig> where P: AsRef<Path> {
        let mut buffer = String::new();
        {
            match File::open(path) {
                Ok(mut file) => file.read_to_string(&mut buffer)?,
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound {
                        return Ok(SwerveConfig::default());
                    } else {
                        return Err(e);
                    }
                }
            };
        }

        match yaml::from_str(&buffer) {
            Ok(conf) => Ok(conf),
            Err(er) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("{}", er))),
        }
    }
}