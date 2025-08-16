use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub base_dir: PathBuf
}