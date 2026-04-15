use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "task")]
    pub tasks: Vec<TaskConfig>,
}

#[derive(Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub schedule: String,
    pub command: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("tasks.toml")?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
