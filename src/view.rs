use std::process::Command as CliCommand;

use conrod::backend::glium::glium;
use conrod::backend::glium::glium::backend::glutin::glutin::Event;
use conrod::backend::glium::glium::backend::glutin::Display;
use conrod::Ui;
use conrod::{color, widget_ids};

use crate::bindings::Shortcut;
use crate::commands::Command;
use crate::state::State;
use crate::window_position::WindowPosition;

widget_ids! {
    pub struct Ids {
        canvas,
        head_canvas,
        head_breadcrumbs,
        list_canvas,
        command_list,
        command_list_item_canvas[],
        command_list_item_shortcut_canvas[],
        command_list_item_name_canvas[],
        command_list_item_shortcut_widget[],
        command_list_item_name_widget[],
    }
}

pub enum SpacerunEvent {
    SelectCommand(Command),
    PrevLevelCommand,
    FocusLost,
    CloseApplication,
}

static DEFAULT_FONT_SIZE: u32 = 14;

pub fn handle_event(event: &Event, state: &State) -> Option<SpacerunEvent> {
    match event {
        glium::glutin::Event::WindowEvent { event, .. } => match event {
            // Break from the loop upon `Escape`.
            glium::glutin::WindowEvent::CloseRequested
            | glium::glutin::WindowEvent::KeyboardInput {
                input:
                    glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => return Some(SpacerunEvent::CloseApplication),
            glium::glutin::WindowEvent::Focused(false) => return Some(SpacerunEvent::FocusLost),
            glium::glutin::WindowEvent::KeyboardInput { input, .. } => {
                if let Some(virtual_keycode) = input.virtual_keycode {
                    if input.state == glium::glutin::ElementState::Pressed {
                        if virtual_keycode == glium::glutin::VirtualKeyCode::Back {
                            return Some(SpacerunEvent::PrevLevelCommand);
                        }
                        let pressed_shortcut = Shortcut {
                            modifiers: input.modifiers,
                            key_code: virtual_keycode.into(),
                        };
                        let found_child = state
                            .selected_command
                            .find_child_for_shortcut(&pressed_shortcut);
                        if let Some(found_child) = found_child {
                            return select_command(&found_child);
                        }
                    }
                }
            }
            _ => (),
        },
        _ => (),
    }
    None
}

fn select_command(command: & Command) -> Option<SpacerunEvent> {
    match command {
        command @ Command::Node(_) => return Some(SpacerunEvent::SelectCommand(command.clone())),
        Command::Leaf(child_leaf) => {
            CliCommand::new("sh")
                .arg("-c")
                .arg(&child_leaf.cmd)
                .spawn()
                .expect("process failed to execute");
            return Some(SpacerunEvent::CloseApplication);
        }
    }
}

/**
 * Calculate the "real" height of our rendered UI.
 *
 * The canvasses we render fit to the window's height, so we can't get the
 * "real" rendered height of our content from them.
 * Instead, we need to combine the height of our more static known elements to
 * get the new window height.
 *
 * TODO (LinuCC) We probably need a max height? Same as window height?
 */
pub fn rendered_elements_height(ui: &Ui, ids: &Ids) -> Option<f64> {
    if let Some(list_render_rect) = ui.kids_bounding_box(ids.command_list) {
        if let Some(head_render_rect) = ui.kids_bounding_box(ids.head_canvas) {
            return Some(list_render_rect.h() + head_render_rect.h());
        }
    }
    None
}

pub fn update_initial_window_state(ui: &mut Ui, state: &mut State, ids: &mut Ids) {
    // Generate Ui to get its height
    //
    // FIXME LinuCC For some reason `ui.kids_bounding_box()` accesses the
    //     `ui.prev_updated_widgets`, which only exists after generating the Ui
    //     a second time.
    set_ui(ui.set_widgets(), &state, &state.selected_command, ids);
    set_ui(ui.set_widgets(), &state, &state.selected_command, ids);

    if let Some(height) = rendered_elements_height(ui, ids) {
        state.window_dimensions.height = height;
    }
}

pub fn update_window_and_window_state(
    new_window_height: f64,
    state: &mut State,
    display: &Display,
    force_update: bool,
) -> () {
    if new_window_height != state.window_dimensions.height || force_update {
        eprintln!("Updating window size.");

        let mut new_window_position = None;
        match state.config.position {
            Some(WindowPosition::Top) => {
                let current_monitor = display.gl_window().get_current_monitor();
                state.window_dimensions.width = current_monitor.get_dimensions().width;
                new_window_position = Some(current_monitor.get_position().to_logical(1.0));
            }
            Some(WindowPosition::Bottom) => {
                let current_monitor = display.gl_window().get_current_monitor();
                state.window_dimensions.width = current_monitor.get_dimensions().width;
                let monitor_height = current_monitor.get_dimensions().height;
                new_window_position = Some((0.0, monitor_height - new_window_height as f64).into());
            }
            _ => {}
        };

        if let Some(new_window_position) = new_window_position {
            println!("Setting windows position");
            state.window_position = new_window_position;
            display.gl_window().set_position(new_window_position);
        }
        state.window_dimensions.height = new_window_height;
        display.gl_window().set_inner_size(state.window_dimensions);
    }
}

// Declare the `WidgetId`s and instantiate the widgets.
pub fn set_ui(ref mut ui: conrod::UiCell, state: &State, command: &Command, ids: &mut Ids) {
    use conrod::{widget, Colorable, Positionable, Sizeable, Widget};


    let displayed_leafs = command.displayable_children();

    // Make sure we have enough Ids for the displayed items
    if displayed_leafs.len() != ids.command_list_item_canvas.len() {
        ids.command_list_item_canvas
            .resize(displayed_leafs.len(), &mut ui.widget_id_generator());
        ids.command_list_item_shortcut_canvas
            .resize(displayed_leafs.len(), &mut ui.widget_id_generator());
        ids.command_list_item_name_canvas
            .resize(displayed_leafs.len(), &mut ui.widget_id_generator());
        ids.command_list_item_shortcut_widget
            .resize(displayed_leafs.len(), &mut ui.widget_id_generator());
        ids.command_list_item_name_widget
            .resize(displayed_leafs.len(), &mut ui.widget_id_generator());
    }


    let child_canvas = [
        (
            ids.head_canvas,
            widget::Canvas::new()
                .length(30.0)
                .pad_left(10.0)
                .color(color::ORANGE),
        ),
        (
            ids.list_canvas,
            widget::Canvas::new()
                .color(color::BLUE),
        ),
    ];
    // let canvas = widget::Canvas::new()
    widget::Canvas::new()
        .color(color::DARK_CHARCOAL)
        .flow_down(&child_canvas)
        .set(ids.canvas, ui);

    let breadcrumb_text = state.selection_path.iter().fold("Root".into(), |acc, selection| {
        format!("{} > {}", acc, selection.name)
    });
    widget::Text::new(&breadcrumb_text)
        .mid_left_of(ids.head_canvas)
        .color(color::WHITE)
        .h_of(ids.head_canvas)
        .font_size(state.config.font_size.unwrap_or(DEFAULT_FONT_SIZE))
        .set(ids.head_breadcrumbs, ui);


    // Generate list displaying the commands
    let (mut items, scrollbar) = widget::List::flow_down(displayed_leafs.len())
        .item_size(
            item_height_by_font_size(state.config.font_size.unwrap_or(DEFAULT_FONT_SIZE)).into(),
        )
        .scrollbar_on_top()
        .mid_top_of(ids.list_canvas)
        .w_of(ids.list_canvas)
        .h_of(ids.list_canvas)
        .set(ids.command_list, ui);

    // Generate each command item
    while let Some(item) = items.next(ui) {
        let i = item.i;

        let text_container_canvas = widget::Canvas::new().pad(5.0);
        let child_canvas = [
            (
                ids.command_list_item_shortcut_canvas[i],
                text_container_canvas
                    .clone()
                    .length_weight(0.2)
                    .color(color::ORANGE),
            ),
            (
                ids.command_list_item_name_canvas[i],
                text_container_canvas.color(color::CHARCOAL),
            ),
        ];
        let canvas = widget::Canvas::new().flow_right(&child_canvas);

        item.set(canvas, ui);

        widget::Text::new(&displayed_leafs[i].shortcut.to_string())
            .middle_of(ids.command_list_item_shortcut_canvas[i])
            .color(color::WHITE)
            .font_size(state.config.font_size.unwrap_or(DEFAULT_FONT_SIZE))
            .set(ids.command_list_item_shortcut_widget[i], ui);

        widget::Text::new(&displayed_leafs[i].name)
            .mid_left_of(ids.command_list_item_name_canvas[i])
            .color(color::WHITE)
            .font_size(state.config.font_size.unwrap_or(DEFAULT_FONT_SIZE))
            .set(ids.command_list_item_name_widget[i], ui);
    }

    if let Some(s) = scrollbar {
        s.set(ui)
    }
}

/// Calculate the items height by the given font size
fn item_height_by_font_size(font_size: u32) -> u32 {
    font_size + 20
}
