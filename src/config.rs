use serde::Deserialize;
use std::{fs, path::{Path, PathBuf}};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub site_name: String,
    pub publish_dir: String,
    pub posts_dir: String,
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Config {
        let config = fs::read_to_string(path).expect("Config file config.toml should exists");
        toml::from_str(&config).expect("Config file content should will formatted")
    }

    pub fn post_dir(&self) -> PathBuf {
        PathBuf::new().join(&self.posts_dir)
    }

    pub fn publish_dir(&self) -> PathBuf {
        PathBuf::new().join(&self.publish_dir)
    }
}
