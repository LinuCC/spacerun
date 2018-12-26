use crate::commands::Command;
use crate::config::SpacerunConfig;

const DEFAULT_WINDOW_WIDTH: u32 = 500;
const DEFAULT_WINDOW_HEIGHT: u32 = 400;

pub struct State {
    pub window_width: u32,
    pub window_height: u32,
    pub config: SpacerunConfig,
    pub selected_command: Command,
}

impl State {
    pub fn new(config: SpacerunConfig) -> State {
        State {
            window_width: DEFAULT_WINDOW_WIDTH,
            window_height: DEFAULT_WINDOW_HEIGHT,
            selected_command: config.commands.to_owned(),
            config: config,
        }
    }

    pub fn select_command(self: &mut State, new_command: Command) -> () {
        self.selected_command = new_command;
    }

    pub fn set_window_height(self: &mut State, window_height: u32) -> () {
        self.window_height = window_height;
    }
}
