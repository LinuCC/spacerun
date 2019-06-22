use azul::dom::{NodeId, TabIndex};
use azul::prelude::*;
use azul::widgets::text_input::*;
use azul::window::WinitXWindowType;
use structopt::StructOpt;

use crate::bindings::Shortcut;
use crate::commands::Command;
use crate::state::State;
use crate::view::render_app;

mod bindings;
mod commands;
mod config;
mod state;
mod view;

macro_rules! CSS_PATH {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.css")
    };
}
macro_rules! FONT_PATH {
    () => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/fonts/NotoSans/NotoSans-Regular.ttf"
        )
    };
}
const FONT: &[u8] = include_bytes!(FONT_PATH!());

#[derive(Debug, StructOpt)]
#[structopt(name = "spacerun")]
pub struct Options {
    #[structopt(short = "s", long = "shortcut", parse(try_from_str))]
    initial_shortcut: Option<Shortcut>,
}

pub enum SpacerunEvent {
    SelectCommand(Command),
    PrevLevelCommand,
    FocusLost,
    CloseApplication,
}

// struct TestCrudApp {
//     text_input: TextInputState,
//     text_input2: TextInputState,
//     text_input3: TextInputState,
//     text_input4: TextInputState,
// }
//
// impl Default for TestCrudApp {
//     fn default() -> Self {
//         Self {
//             text_input: TextInputState::new("Hover mouse over rectangle and press keys"),
//             text_input2: TextInputState::new("Hover mouse over rectangle and press keys"),
//             text_input3: TextInputState::new("Hover mouse over rectangle and press keys"),
//             text_input4: TextInputState::new("Hover mouse over rectangle and press keys"),
//         }
//     }
// }
//
fn on_text_input_focus(mut callback_info: CallbackInfo<SpacerunApp>) -> Option<()> {
    let (dom_id, node_id) = callback_info.hit_dom_node.clone();
    // let node = callback_info
    //     .get_node(&(dom_id.clone(), node_id.clone()))
    //     .unwrap()
    //     .clone();

    // println!(
    //     "{:?}",
    //     callback_info.get_node(&callback_info.hit_dom_node.clone())
    // );
    // let node_hierarchy = callback_info.get_node_hierarchy().clone();
    // let next_sibling: Option<NodeId> = node_id.following_siblings(&node_hierarchy).nth(1);
    // if let Some(next_sibling) = next_sibling {
    //     println!(
    //         "setting focus to {:?}",
    //         (
    //             dom_id.clone(),
    //             next_sibling.children(&node_hierarchy).next().unwrap()
    //         )
    //     );
    //     callback_info.set_focus_from_node_id((
    //         dom_id,
    //         next_sibling.children(&node_hierarchy).next().unwrap(),
    //     ));
    //     Some(())
    // } else {
    //     println!("No focus :(");
    //     None
    // }
    // FIXME Ugly, but for some reason code above does not work.
    if let Some(index) = callback_info.target_index_in_parent() {
        callback_info.set_focus_from_css(&format!("#text_input_{}", index + 2));
        Some(())
    } else {
        None
    }
}

pub struct SpacerunApp {
    pub state: State,
}

impl Layout for SpacerunApp {
    fn layout(&self, info: LayoutInfo<Self>) -> Dom<Self> {
        render_app(self, info)
    }
}

impl SpacerunApp {
    fn handle_event(&mut self, event: SpacerunEvent) -> () {
        use SpacerunEvent::*;
        match event {
            SelectCommand(new_selected_command) => {
                self.state.selected_command = new_selected_command.clone();
                self.state
                    .selection_path
                    .push(self.state.selected_command.clone().into());
            }
            PrevLevelCommand => {
                if self.state.selection_path.pop().is_some() {
                    let new_command = self.state.selection_path.iter().try_fold(
                        &self.state.config.commands,
                        |acc, command_display| {
                            acc.find_child_for_shortcut(&command_display.shortcut)
                        },
                    );
                    if let Some(new_command) = new_command {
                        self.state.selected_command = new_command.clone();
                    }
                }
            }
            FocusLost => {
                // FIXME LinuCC Implement
                println!("WHAAAAA");
            }
            // FIXME LinuCC Panic, lul
            CloseApplication => panic!("App closed normally :kappa:"),
        }
    }
}

// impl Layout for TestCrudApp {
//     fn layout(&self, info: LayoutInfo<Self>) -> Dom<Self> {
//         Dom::div()
//             .with_callback(
//                 On::VirtualKeyDown,
//                 |mut callback_info: CallbackInfo<TestCrudApp>| {
//                     if callback_info.get_keyboard_state().latest_virtual_keycode
//                         == Some(VirtualKeyCode::Tab)
//                     {
//                         callback_info.set_focus_from_css("#text_input_1");
//                         println!("WATTT");
//                         Some(())
//                     } else {
//                         None
//                     }
//                 },
//             )
//             .with_id("container")
//             .with_child(
//                 TextInput::new()
//                     .bind(info.window, &self.text_input, &self)
//                     .dom(&self.text_input)
//                     .with_tab_index(TabIndex::Auto)
//                     .with_callback(On::TextInput, on_text_input_focus)
//                     .with_id("text_input_1"),
//             )
//             .with_child(
//                 TextInput::new()
//                     .bind(info.window, &self.text_input2, &self)
//                     .dom(&self.text_input2)
//                     .with_tab_index(TabIndex::Auto)
//                     .with_callback(On::TextInput, on_text_input_focus)
//                     .with_id("text_input_2"),
//             )
//             .with_child(
//                 TextInput::new()
//                     .bind(info.window, &self.text_input3, &self)
//                     .dom(&self.text_input3)
//                     .with_tab_index(TabIndex::Auto)
//                     .with_callback(On::TextInput, on_text_input_focus)
//                     .with_id("text_input_3"),
//             )
//             .with_child(
//                 TextInput::new()
//                     .bind(info.window, &self.text_input4, &self)
//                     .dom(&self.text_input4)
//                     .with_tab_index(TabIndex::Auto)
//                     .with_callback(On::TextInput, on_text_input_focus)
//                     .with_id("text_input_4"),
//             )
//     }
// }
//
fn main() {
    // --- Parse command line args
    let options = Options::from_args();
    eprintln!("options: {:?}", options);
    // --- Setup Commands
    let config = config::load_config()
        .expect("Error loading the config. Check your configuration for inconsistencies.");
    eprintln!("Commands Loaded!");
    eprintln!("{:?}", config.commands);

    let mut state = State::new(config, options);
    let app = SpacerunApp { state };

    let mut app = App::new(app, AppConfig::default()).unwrap();

    let font_id = app.app_state.resources.add_css_font_id("NotoSans-Regular");
    app.app_state
        .resources
        .add_font_source(font_id, FontSource::Embedded(FONT));
    let css = css::override_native(include_str!(CSS_PATH!())).unwrap();

    let window = app
        .create_window(
            WindowCreateOptions {
                state: WindowState {
                    has_decorations: false,
                    is_always_on_top: true,
                    ..WindowState::default()
                },
                x_window_type: Some(WinitXWindowType::Utility),
                ..WindowCreateOptions::default()
            },
            css,
        )
        .unwrap();
    app.run(window).unwrap();
}
