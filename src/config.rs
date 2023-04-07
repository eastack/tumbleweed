use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub site_name: String,
    pub publish_dir: String,
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Config {
        let config = fs::read_to_string(path).expect("Config file config.toml should exists");
        toml::from_str(&config).expect("Config file content should will formatted")
    }
}
