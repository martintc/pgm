use anyhow::Error;
use anyhow::Result;
use home::home_dir;
use serde::Deserialize;
use serde::Serialize;
use std::{io::Write, process::exit};

use crate::models::monitor::Monitor;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub monitors: Vec<Monitor>,
}

impl Configuration {
    pub fn add_monitor(&mut self, monitor: Monitor) {
        self.monitors.push(monitor);
    }
}

pub fn get_path() -> std::path::PathBuf {
    let config_path = match home_dir() {
        Some(home_path) => home_path,
        None => exit(1),
    };

    let config_path = config_path.join(".config/pgm");

    config_path
}

pub fn create_blank_config() -> anyhow::Result<(), anyhow::Error> {
    let config = Configuration { monitors: vec![] };

    let mut file = std::fs::File::create(get_path())?;

    file.write_all(serde_yaml::to_string(&config).unwrap().as_bytes())?;

    Ok(())
}

pub fn read_config() -> Result<Configuration, Error> {
    let file_contents = std::fs::read_to_string(get_path())?;
    let config: Configuration = serde_yaml::from_str(file_contents.as_str())?;
    Ok(config)
}

pub fn get_config() -> Result<Configuration, Error> {
    let config_path = get_path();
    if !std::path::Path::exists(&config_path) {
        match create_blank_config() {
            Ok(()) => (),
            Err(e) => panic!("Error: {}", e),
        };
    }

    let config = match read_config() {
        Ok(config) => config,
        Err(e) => panic!("Error: {}", e),
    };
    Ok(config)
}

pub fn write_config(config: &Configuration) -> Result<(), anyhow::Error> {
    let mut file = std::fs::File::create(get_path())?;

    file.write_all(serde_yaml::to_string(&config).unwrap().as_bytes())?;

    Ok(())
}
