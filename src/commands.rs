extern crate serde_json;

use self::serde_json::{Map, Value};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct CommandNode {
    pub key: String,
    pub name: String,
    pub cmd: Option<String>,
    pub children: Option<Vec<Command>>,
}

#[derive(Clone)]
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

pub enum Command {
    Node(CommandNode),
    Leaf(CommandLeaf),
}

pub fn get_commands() -> Result<Command, Box<Error>> {
    println!("Oh hai!!!!");
    let mut file = File::open("config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("contents: {}", contents);
    let json_value: Value = serde_json::from_str(&contents)?;

    match json_value {
        Value::Object(map) => {
            let command = parse_json_value(map.get("commands").unwrap())
                .unwrap()
                .unwrap();
            Ok(command)
        }
        _ => {
            panic!("dafuq is dis json config");
        }
    }
}

fn parse_json_value(value: &Value) -> Result<Option<Command>, ()> {
    return match value {
        Value::Object(map) => {
            if map.contains_key("children") {
                let children = match map.get("children") {
                    Some(children_value) => {
                        let children_commands = match children_value {
                            Value::Array(children_values) => children_values
                                .iter()
                                .map(|child_value| parse_json_value(child_value).unwrap().unwrap())
                                .collect::<Vec<Command>>(),
                            _ => {
                                return Err(());
                            }
                        };
                        Some(children_commands)
                    }
                    None => None,
                };

                let node = CommandNode {
                    key: get_string_json_value(map, String::from("key"))
                        .unwrap()
                        .to_owned(),
                    name: get_string_json_value(map, String::from("name"))
                        .unwrap()
                        .to_owned(),
                    cmd: get_string_json_value(map, String::from("cmd")),
                    children: children,
                };
                Ok(Some(Command::Node(node)))
            } else {
                let leaf = CommandLeaf {
                    key: get_string_json_value(map, String::from("key")).unwrap(),
                    name: get_string_json_value(map, String::from("name")).unwrap(),
                    cmd: get_string_json_value(map, String::from("cmd")).unwrap(),
                };
                Ok(Some(Command::Leaf(leaf)))
            }
        }
        _ => Err(()),
    };
}

fn get_string_json_value(map: &Map<String, Value>, key: String) -> Option<String> {
    match map.get(&key)? {
        Value::String(string) => Some(string.to_owned()),
        _ => None,
    }
}

//
// lazy_static! {
// 	static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
// }
//
// fn try_main() -> Result<(), Box<Error>> {
// 	// Set property
// 	SETTINGS.write()?.set("property", 42)?;
//
// 	// Get property
// 	println!("property: {}", SETTINGS.read()?.get::<i32>("property")?);
//
// 	Ok(())
// }
