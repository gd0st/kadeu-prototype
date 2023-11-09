use config;
use serde::Deserialize;
use std::fs;
use std::fs::ReadDir;
use std::io;

#[derive(Deserialize, Debug)]
pub struct Settings {
    shuffle: bool,
    pub repo: String,
}

pub fn get_config(path: &String) -> Result<Settings, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::with_name(path))
        .build()?
        .try_deserialize::<Settings>()
}

impl Settings {
    pub fn read_repo(&self) -> io::Result<ReadDir> {
        fs::read_dir(self.repo.clone())
    }
}
