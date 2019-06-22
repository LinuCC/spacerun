use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use directories::ProjectDirs;
use serde_derive::Deserialize;

use crate::commands::Command;
// use crate::window_position::WindowPosition;

#[derive(Debug, Clone, Deserialize)]
pub struct SpacerunConfig {
    pub commands: Command,
    pub font_size: Option<u32>,
    // pub position: Option<WindowPosition>,
}

pub fn load_config() -> Result<SpacerunConfig, Box<Error>> {
    let mut config_dir = ProjectDirs::from("cc", "linu", "spacerun")
        .unwrap()
        .config_dir()
        .to_owned();
    config_dir.push("config.json");
    println!("CFG dir {:?}", config_dir);
    let mut file = File::open(config_dir)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("contents: {}", contents);
    let config: SpacerunConfig = serde_json::from_str(&contents)?;
    Ok(config)
}
