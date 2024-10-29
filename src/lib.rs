use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schema {
    overrides: Vec<Override>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Override {
    path: PathBuf,
    setting: ron::Value
}

pub fn load_template(path: PathBuf) {
    
}

pub fn generate_template() {
    
}