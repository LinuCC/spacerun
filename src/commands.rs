use serde_derive::Deserialize;

use crate::bindings::KeyCode;

#[derive(Debug, Clone, Deserialize)]
pub struct CommandNode {
    pub key: KeyCode,
    pub name: String,
    pub cmd: Option<String>,
    pub children: Vec<Command>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandLeaf {
    pub key: KeyCode,
    pub name: String,
    pub cmd: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Node(CommandNode),
    Leaf(CommandLeaf),
}

/**
 * Easily displayable command
 */
#[derive(Clone)]
pub struct CommandDisplay {
    pub key: String,
    pub name: String,
}

impl From<CommandNode> for CommandDisplay {
    fn from(node: CommandNode) -> Self {
        CommandDisplay {
            key: node.key.to_string(),
            name: node.name,
        }
    }
}

impl From<CommandLeaf> for CommandDisplay {
    fn from(node: CommandLeaf) -> Self {
        CommandDisplay {
            key: node.key.to_string(),
            name: node.name,
        }
    }
}

impl Command {
    pub fn displayable_children(self: &Command) -> Vec<CommandDisplay> {
        match self {
            Command::Leaf(command_leaf) => vec![command_leaf.clone().into()],
            Command::Node(command_node) => command_node
                .children
                .iter()
                .map(|child| match child {
                    Command::Leaf(child_leaf) => child_leaf.clone().into(),
                    Command::Node(child_node) => child_node.clone().into(),
                })
                .collect(),
        }
    }
}
