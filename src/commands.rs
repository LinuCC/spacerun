use serde_derive::Deserialize;

use crate::bindings::Shortcut;

#[derive(Debug, Clone, Deserialize)]
pub struct CommandNode {
    pub shortcut: Shortcut,
    pub name: String,
    pub cmd: Option<String>,
    pub children: Vec<Command>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandLeaf {
    pub shortcut: Shortcut,
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
    pub shortcut: Shortcut,
    pub name: String,
}

impl From<Command> for CommandDisplay {
    fn from(command: Command) -> Self {
        match command {
            Command::Node(node) => node.into(),
            Command::Leaf(leaf) => leaf.into(),
        }
    }
}

impl From<CommandNode> for CommandDisplay {
    fn from(node: CommandNode) -> Self {
        CommandDisplay {
            shortcut: node.shortcut,
            name: node.name,
        }
    }
}

impl From<CommandLeaf> for CommandDisplay {
    fn from(node: CommandLeaf) -> Self {
        CommandDisplay {
            shortcut: node.shortcut,
            name: node.name,
        }
    }
}

impl Command {
    pub fn shortcut(&self) -> &Shortcut {
        match self {
            Command::Leaf(command_leaf) => &command_leaf.shortcut,
            Command::Node(command_node) => &command_node.shortcut,
        }
    }

    pub fn displayable_children(&self) -> Vec<CommandDisplay> {
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

    pub fn find_child_for_shortcut(&self, shortcut: &Shortcut) -> Option<&Command> {
        if let Command::Node(node) = self {
            node.children.iter().find(|&child| match child {
                Command::Node(child_node) => &child_node.shortcut == shortcut,
                Command::Leaf(child_leaf) => &child_leaf.shortcut == shortcut,
            })
        } else {
            None
        }
    }
}
