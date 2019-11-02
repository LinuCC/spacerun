use conrod::glium::glutin::dpi::{LogicalPosition, LogicalSize};

use std::collections::HashMap;

use crate::commands::{Command, CommandDisplay, CommandTaskVariable};
use crate::config::SpacerunConfig;
use crate::Options;

const DEFAULT_WINDOW_WIDTH: f64 = 500.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 400.0;

pub struct State {
    pub window_position: LogicalPosition,
    pub window_dimensions: LogicalSize,
    pub config: SpacerunConfig,
    pub selected_command: Command,
    pub selection_path: Vec<CommandDisplay>,
    pub form_command_task_variables: HashMap<String, String>,
    pub options: Options,
}

impl State {
    pub fn new(config: SpacerunConfig, options: Options) -> State {
        let state = State {
            window_dimensions: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT).into(),
            window_position: (0, 0).into(),
            selected_command: select_initial_command(&config, &options),
            selection_path: vec![],
            form_command_task_variables: HashMap::new(),
            config,
            options,
        };
        return state;
    }
}

pub fn init_variables_form_input(
    form_command_task_variables: &mut HashMap<String, String>,
    variables: &[CommandTaskVariable],
) {
    *form_command_task_variables = variables
        .iter()
        .map(|variable| {
            (
                variable.name.clone(),
                variable.default_value.clone().unwrap_or("".to_string()),
            )
        })
        .collect();
}

fn select_initial_command(config: &SpacerunConfig, options: &Options) -> Command {
    let mut command = &config.commands;

    if let Some(shortcut) = &options.initial_shortcut {
        if let Some(child) = command.find_child_for_shortcut(shortcut) {
            command = child;
        }
    };

    command.to_owned()
}
