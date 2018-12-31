use conrod::glium::glutin::dpi::{LogicalPosition, LogicalSize};

use crate::commands::Command;
use crate::config::SpacerunConfig;

const DEFAULT_WINDOW_WIDTH: f64 = 500.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 400.0;

pub struct State {
    pub window_position: LogicalPosition,
    pub window_dimensions: LogicalSize,
    pub config: SpacerunConfig,
    pub selected_command: Command,
}

impl State {
    pub fn new(config: SpacerunConfig) -> State {
        let state = State {
            window_dimensions: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT).into(),
            window_position: (0, 0).into(),
            selected_command: config.commands.to_owned(),
            config: config,
        };
        return state;
    }
}
