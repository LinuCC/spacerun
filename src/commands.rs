use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use directories::ProjectDirs;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CommandNode {
    pub key: String,
    pub name: String,
    pub cmd: Option<String>,
    pub children: Vec<Command>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandLeaf {
    pub key: String,
    pub name: String,
    pub cmd: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Node(CommandNode),
    Leaf(CommandLeaf),
}

#[derive(Debug, Clone, Deserialize)]
struct ConfigBase {
    commands: Command,
}

pub fn get_commands() -> Result<Command, Box<Error>> {
    println!("Oh hai!!!!");
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
    let config_base: ConfigBase = serde_json::from_str(&contents)?;
    Ok(config_base.commands)
}
