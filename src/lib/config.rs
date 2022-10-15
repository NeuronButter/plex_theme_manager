use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub folders: Vec<String>,
    pub themes: String,
}

pub fn get_config(path: String) -> Result<Config> {
    let file_contents = fs::read_to_string(path).expect("Config file is unreadable");
    let config: Config = serde_json::from_str(&file_contents)?;
    Ok(config)
}