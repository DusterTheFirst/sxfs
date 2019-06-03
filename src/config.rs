use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io::{self, ErrorKind};
use std::fs;
use std::path::{PathBuf, Path};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub users: HashMap<String, String>,
    pub verbose: bool,
    pub uploads_dir: PathBuf
}

impl Default for Config {
    fn default() -> Config {
        Config {
            users: HashMap::default(),
            verbose: false,
            uploads_dir: PathBuf::from("uploads")
        }
    }
}

pub enum ConfigError {
    Read(io::Error),
    Parse(toml::de::Error),
    Create(toml::ser::Error),
    Write(io::Error)
}

impl Config {
    pub fn load(path: &Path) -> Result<Config, ConfigError> {
        match fs::read_to_string(path) {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match toml::to_string(&Config::default()) {
                    Ok(c) => match fs::write(path, &c) {
                        Ok(_) => Ok(Config::default()),
                        Err(e) => Err(ConfigError::Write(e))
                    }
                    Err(e) => Err(ConfigError::Create(e))
                },
                _ => {
                    Err(ConfigError::Read(e))
                }
            },
            Ok(file) => match toml::from_str::<Config>(&file) {
                Ok(c) => Ok(c),
                Err(e) => Err(ConfigError::Parse(e))
            }
        }
    }
}