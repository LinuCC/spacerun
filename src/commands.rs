use directories::ProjectDirs;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use serde_derive::{Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CommandNode {
    pub key: String,
    pub name: String,
    pub cmd: Option<String>,
    pub children: Option<Vec<Command>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandLeaf {
    pub key: String,
    pub name: String,
    pub cmd: String,
}

impl From<CommandNode> for CommandLeaf {
    fn from(node: CommandNode) -> Self {
        CommandLeaf {
            key: node.key,
            name: node.name,
            cmd: node.cmd.unwrap(),
        }
    }
}

impl<'a> From<&'a CommandNode> for CommandLeaf {
    fn from(node: &'a CommandNode) -> Self {
        CommandLeaf {
            key: node.key.clone(),
            name: node.name.clone(),
            cmd: node.cmd.clone().unwrap(),
        }
    }
}

impl From<Box<CommandNode>> for CommandLeaf {
    fn from(node: Box<CommandNode>) -> Self {
        CommandLeaf {
            key: node.key.clone(),
            name: node.name.clone(),
            cmd: node.cmd.unwrap().clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Node(CommandNode),
    Leaf(CommandLeaf),
}

#[derive(Debug, Clone, Deserialize)]
struct ConfigBase {
    commands: Command
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
