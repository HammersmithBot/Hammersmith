use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct Data {
    pub(crate) bot: Bot,
}

#[derive(Deserialize)]
pub struct Bot {
    pub(crate) token: String,
}

pub fn read_config(file_path: &str) -> Data {
    let config_data = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            exit(1);
        }
    };
    let config: Data = match toml::from_str(&config_data) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Error parsing file: {}", error);
            exit(1);
        }
    };
    config
}
