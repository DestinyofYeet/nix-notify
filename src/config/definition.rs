use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub branches: Vec<Branch>,
    pub general: General,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct General {
    pub database_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub delay_minutes: u64,
}
