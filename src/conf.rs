use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub local: Local,
    pub staging: Staging,
}

#[derive(Debug, Deserialize)]
pub struct Local {
    pub source: String,
    pub branch: String,
}

#[derive(Debug, Deserialize)]
pub struct Staging {
    pub source: String,
    pub branch: String,
}

#[derive(Debug, Deserialize)]
pub struct Production {
    pub source: String,
    pub branch: String,
}

pub fn read() -> Config {
    let contents = fs::read_to_string("tuan.yaml").expect("Something went wrong reading the file");
    let config: Config = serde_yaml::from_str(&contents).expect("Failed to parse config file");
    return config;
}
