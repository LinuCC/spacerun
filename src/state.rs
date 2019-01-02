use conrod::glium::glutin::dpi::{LogicalPosition, LogicalSize};

use crate::Options;
use crate::commands::Command;
use crate::config::SpacerunConfig;

const DEFAULT_WINDOW_WIDTH: f64 = 500.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 400.0;

pub struct State {
    pub window_position: LogicalPosition,
    pub window_dimensions: LogicalSize,
    pub config: SpacerunConfig,
    pub selected_command: Command,
    pub options: Options,
}

impl State {
    pub fn new(config: SpacerunConfig, options: Options) -> State {
        let state = State {
            window_dimensions: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT).into(),
            window_position: (0, 0).into(),
            selected_command: select_initial_command(&config, &options),
            config,
            options,
        };
        return state;
    }
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
