use std::process::Command as CliCommand;

use conrod::backend::glium::glium;
use conrod::backend::glium::glium::backend::glutin::glutin::Event;
use conrod::{color, widget_ids};

use crate::commands::Command;
use crate::config::SpacerunConfig;

widget_ids! {
    pub struct Ids {
        canvas,
        command_list,
        command_list_item_canvas[],
        command_list_item_key_canvas[],
        command_list_item_name_canvas[],
        command_list_item_key_widget[],
        command_list_item_name_widget[],
    }
}

pub enum SpacerunEvent<'a> {
    SelectCommand(&'a Command),
    CloseApplication,
}

static DEFAULT_FONT_SIZE: u32 = 14;

pub fn handle_event<'a>(event: &Event, selected_command: &'a Command) -> Option<SpacerunEvent<'a>> {
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
            glium::glutin::WindowEvent::KeyboardInput { input, .. } => {
                match input.virtual_keycode {
                    Some(virtual_keycode)
                        if input.state == glium::glutin::ElementState::Pressed =>
                    {
                        if let Command::Node(command_node) = selected_command {
                            let found_child =
                                command_node.children.iter().find(|&child| match child {
                                    Command::Node(child_node) => child_node.key == virtual_keycode,
                                    Command::Leaf(child_leaf) => child_leaf.key == virtual_keycode,
                                });
                            match found_child {
                                Some(found_child @ Command::Node(_)) => {
                                    return Some(SpacerunEvent::SelectCommand(found_child))
                                }
                                Some(Command::Leaf(child_leaf)) => {
                                    CliCommand::new("sh")
                                        .arg("-c")
                                        .arg(&child_leaf.cmd)
                                        .spawn()
                                        .expect("process failed to execute");
                                    return Some(SpacerunEvent::CloseApplication);
                                }
                                None => {}
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        },
        _ => (),
    }
    None
}

// Declare the `WidgetId`s and instantiate the widgets.
pub fn set_ui(
    ref mut ui: conrod::UiCell,
    config: &SpacerunConfig,
    command: &Command,
    ids: &mut Ids,
) {
    use conrod::{widget, Colorable, Positionable, Sizeable, Widget};

    widget::Canvas::new()
        .color(color::DARK_CHARCOAL)
        .set(ids.canvas, ui);

    let displayed_leafs = command.displayable_children();

    // Make sure we have enough Ids for the displayed items
    if displayed_leafs.len() != ids.command_list_item_canvas.len() {
        ids.command_list_item_canvas
            .resize(displayed_leafs.len(), &mut ui.widget_id_generator());
        ids.command_list_item_key_canvas
            .resize(displayed_leafs.len(), &mut ui.widget_id_generator());
        ids.command_list_item_name_canvas
            .resize(displayed_leafs.len(), &mut ui.widget_id_generator());
        ids.command_list_item_key_widget
            .resize(displayed_leafs.len(), &mut ui.widget_id_generator());
        ids.command_list_item_name_widget
            .resize(displayed_leafs.len(), &mut ui.widget_id_generator());
    }

    // Generate list displaying the commands
    let (mut items, scrollbar) = widget::List::flow_down(displayed_leafs.len())
        .item_size(item_height_by_font_size(config.font_size.unwrap_or(DEFAULT_FONT_SIZE)).into())
        .scrollbar_on_top()
        .middle_of(ids.canvas)
        .w_of(ids.canvas)
        .set(ids.command_list, ui);

    // Generate each command item
    while let Some(item) = items.next(ui) {
        let i = item.i;

        let text_container_canvas = widget::Canvas::new().pad(5.0);
        let child_canvas = [
            (
                ids.command_list_item_key_canvas[i],
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

        widget::Text::new(&displayed_leafs[i].key)
            .middle_of(ids.command_list_item_key_canvas[i])
            .color(color::WHITE)
            .font_size(config.font_size.unwrap_or(DEFAULT_FONT_SIZE))
            .set(ids.command_list_item_key_widget[i], ui);

        widget::Text::new(&displayed_leafs[i].name)
            .mid_left_of(ids.command_list_item_name_canvas[i])
            .color(color::WHITE)
            .font_size(config.font_size.unwrap_or(DEFAULT_FONT_SIZE))
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
