use std::process::Command as CliCommand;

use conrod::backend::glium::glium;
use conrod::backend::glium::glium::backend::glutin::glutin::Event;

use crate::commands::Command;

pub enum SpacerunEvent<'a> {
    SelectCommand(&'a Command),
    CloseApplication,
}

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
    Some(SpacerunEvent::SelectCommand(selected_command))
}
