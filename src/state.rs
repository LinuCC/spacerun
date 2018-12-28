use conrod::glium::glutin::dpi::{LogicalPosition, LogicalSize};

use crate::commands::Command;
use crate::config::SpacerunConfig;
use crate::view::guess_initial_window_height;

const DEFAULT_WINDOW_WIDTH: f64 = 500.0;

pub struct State {
    pub window_position: LogicalPosition,
    pub window_dimensions: LogicalSize,
    pub config: SpacerunConfig,
    pub selected_command: Command,
}

impl State {
    pub fn new(config: SpacerunConfig) -> State {
        let state = State {
            window_dimensions: (DEFAULT_WINDOW_WIDTH, guess_initial_window_height(&config)).into(),
            window_position: (0, 0).into(),
            selected_command: config.commands.to_owned(),
            config: config,
        };
        return state;
    }
}
