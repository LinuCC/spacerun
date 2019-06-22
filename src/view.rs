use std::process::Command as CliCommand;

use azul::prelude::*;
use azul::widgets::text_input::*;

use crate::bindings::{KeyCode, ModifiersState, Shortcut};
use crate::commands::{Command, CommandLeaf, CommandNode};
use crate::state::State;
use crate::{SpacerunApp, SpacerunEvent};

static DEFAULT_FONT_SIZE: u32 = 14;

pub fn render_app(app: &SpacerunApp, layout_info: LayoutInfo<SpacerunApp>) -> Dom<SpacerunApp> {
    let mut app_container = Dom::div().with_id("app_container");

    let breadcrumb_text = app
        .state
        .selection_path
        .iter()
        .fold("Root".into(), |acc, selection| {
            format!("{} > {}", acc, selection.name)
        });

    app_container = app_container.with_child(
        Dom::div().with_id("app_heading").with_child(
            Dom::div()
                .with_id("breadcrumb_container")
                .with_child(Dom::label(breadcrumb_text)),
        ),
    );
    match app.state.selected_command {
        Command::Node(_) => app_container.with_child(render_command_select_list(app, layout_info)),
        Command::Leaf(_) => app_container.with_child(render_command_form(app, layout_info)),
    }
}

fn handle_keyboard_input(mut info: CallbackInfo<SpacerunApp>) -> Option<()> {
    let keyboard_state = info.get_keyboard_state().clone();
    if let Some(key_code) = keyboard_state.latest_virtual_keycode {
        let app = &mut info.state.data;

        if key_code == VirtualKeyCode::Escape {
            app.handle_event(SpacerunEvent::PrevLevelCommand);
            return Some(());
        }

        let pressed_shortcut = Shortcut {
            modifiers: ModifiersState {
                shift_down: keyboard_state.shift_down,
                ctrl_down: keyboard_state.ctrl_down,
                alt_down: keyboard_state.alt_down,
                super_down: keyboard_state.super_down,
            },
            key_code: KeyCode(key_code),
        };
        let found_child = app
            .state
            .selected_command
            .find_child_for_shortcut(&pressed_shortcut);
        if let Some(found_child) = found_child {
            let event = select_command(&found_child);
            if let Some(event) = event {
                app.handle_event(event);
                return Some(());
            }
        }
    }
    // info.state.handle_event(SelectComand());
    None
}

fn render_command_select_list(
    app: &SpacerunApp,
    layout_info: LayoutInfo<SpacerunApp>,
) -> Dom<SpacerunApp> {
    let mut select_list = Dom::div()
        .with_class("select_list")
        .with_callback(On::VirtualKeyDown, handle_keyboard_input);
    let displayed_leafs = app.state.selected_command.displayable_children();
    for leaf in &displayed_leafs {
        select_list = select_list.with_child(
            Dom::div()
                .with_id(format!("select_list_row_for_{}", leaf.shortcut))
                .with_class("select_list_row")
                .with_child(
                    Dom::div()
                        .with_class("shortcut")
                        .with_child(Dom::label(leaf.shortcut.to_string())),
                )
                .with_child(
                    Dom::div()
                        .with_class("description")
                        .with_child(Dom::label(leaf.name.clone())),
                ),
        );
    }
    select_list
}

fn render_command_form(
    app: &SpacerunApp,
    layout_info: LayoutInfo<SpacerunApp>,
) -> Dom<SpacerunApp> {
    Dom::div()
}

fn select_command(command: &Command) -> Option<SpacerunEvent> {
    match command {
        command @ Command::Node(_) => return Some(SpacerunEvent::SelectCommand(command.clone())),
        Command::Leaf(child_leaf) => {
            CliCommand::new("sh")
                .arg("-c")
                .arg(&child_leaf.cmd.to_string())
                .spawn()
                .expect("process failed to execute");
            return Some(SpacerunEvent::CloseApplication);
        }
    }
}
