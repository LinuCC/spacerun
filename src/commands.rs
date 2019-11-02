use std::collections::HashMap;
use std::fmt::{self, Display};
use std::str::FromStr;

use regex::{Regex, RegexBuilder};
use serde::de;
use serde_derive::Deserialize;

use crate::bindings::Shortcut;

#[derive(Debug, Clone, Deserialize)]
pub struct CommandNode {
    pub shortcut: Shortcut,
    pub name: String,
    pub cmd: Option<CommandTask>,
    pub children: Vec<Command>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandLeaf {
    pub shortcut: Shortcut,
    pub name: String,
    pub cmd: CommandTask,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Node(CommandNode),
    Leaf(CommandLeaf),
}

#[derive(Copy, Clone)]
pub struct CommandTaskParseError;
impl Display for CommandTaskParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse a command task")
    }
}
impl fmt::Debug for CommandTaskParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse a command task")
    }
}

/**
 * The executable part of a command.
 */
#[derive(Debug, Clone)]
pub struct CommandTask {
    pub base: String,
    pub variables: Vec<CommandTaskVariable>,
}

impl Display for CommandTask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.base)
    }
}

#[derive(Debug)]
struct CommandTaskReplaceValuesError;
impl std::error::Error for CommandTaskReplaceValuesError {}
impl fmt::Display for CommandTaskReplaceValuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not replace the placeholders in the CommandTask string"
        )
    }
}

impl FromStr for CommandTask {
    type Err = CommandTaskParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let template_regex = Regex::new(r"\{\{(.+?)\}\}").unwrap();

        let variables = template_regex
            .captures_iter(value)
            .map(|template_data| {
                CommandTaskVariable {
                    name: template_data[1].into(),
                    // TODO Add default value parsing
                    default_value: None,
                }
            })
            .collect();

        Ok(CommandTask {
            base: value.into(),
            variables: variables,
        })
    }
}

impl CommandTask {
    pub fn to_executable_string(
        &self,
        variables: &HashMap<String, String>,
    ) -> Result<String, Box<std::error::Error>> {
        let mut output = self.base.clone();
        for task_variable in &self.variables {
            let value = variables
                .get(&task_variable.name)
                .ok_or(CommandTaskReplaceValuesError)?;
            let match_string = format!("\\{{\\{{{}\\}}\\}}", task_variable.name);
            let re = RegexBuilder::new(&match_string).build()?;
            output = re.replace_all(&output, value.as_str()).to_string();
        }
        Ok(output)
    }
}

impl<'de> de::Deserialize<'de> for CommandTask {
    fn deserialize<D>(deserializer: D) -> Result<CommandTask, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

/**
 * A variable value in a command task, to be filled in e.g. by a form
 */
#[derive(Debug, Clone, Deserialize)]
pub struct CommandTaskVariable {
    pub name: String,
    pub default_value: Option<String>,
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
            name: node.name.to_string(),
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
            node.children
                .iter()
                .find(|&child| child.shortcut() == shortcut)
        } else {
            None
        }
    }
}
