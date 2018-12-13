mod commands;

use conrod::{widget_ids};
use conrod::backend::glium::glium::{self, Surface};
use std::process::{Command as CliCommand};
use crate::commands::{Command, CommandNode, CommandLeaf};


widget_ids! {
    struct Ids { canvas, list }
}

pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}

#[derive(Clone)]
struct CommandDisplay {
    key: String,
    name: String,
}

impl<'a> From<&'a CommandNode> for CommandDisplay {
    fn from(node: &'a CommandNode) -> Self {
        CommandDisplay {
            key: node.key.clone(),
            name: node.name.clone(),
        }
    }
}

impl<'a> From<&'a CommandLeaf> for CommandDisplay {
    fn from(node: &'a CommandLeaf) -> Self {
        CommandDisplay {
            key: node.key.clone(),
            name: node.name.clone(),
        }
    }
}

static FONT: &[u8] = include_bytes!("../assets/fonts/NotoSans/NotoSans-Regular.ttf");

// struct AppState {
//
// }
//
// fn key_pressed

fn main() {
  const WIDTH: u32 = 500;
  const HEIGHT: u32 = 400;

  // --- Setup Commands
  let loaded_commands = commands::get_commands().unwrap();
  let mut selected_command: &Command = &loaded_commands;
  println!("Commands Loaded!");

  // --- Setup Conrod
  let mut events_loop = glium::glutin::EventsLoop::new();
  let window = glium::glutin::WindowBuilder::new()
                  .with_title("Spacerun")
                  .with_dimensions((WIDTH, HEIGHT).into())
                  .with_decorations(false);
  let context = glium::glutin::ContextBuilder::new()
                  .with_vsync(true)
                  .with_multisampling(4);
  let display = glium::Display::new(window, context, &events_loop).unwrap();

  let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
  let ids = Ids::new(ui.widget_id_generator());

  let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

  // Add a `Font` to the `Ui`'s `font::Map` from file.
  ui.fonts.insert(conrod::text::Font::from_bytes(FONT).unwrap());


  // The image map describing each of our widget->image mappings (in our case, none).
  let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

  let mut event_loop = EventLoop::new();
  'main: loop {
      // Handle all events.
      for event in event_loop.next(&mut events_loop) {

          // Use the `winit` backend feature to convert the winit event to a conrod one.
          if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
              ui.handle_event(event);
              event_loop.needs_update();
          }

          match event {
              glium::glutin::Event::WindowEvent { event, .. } => match event {
                  // Break from the loop upon `Escape`.
                  glium::glutin::WindowEvent::CloseRequested |
                  glium::glutin::WindowEvent::KeyboardInput {
                      input: glium::glutin::KeyboardInput {
                          virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                          ..
                      },
                      ..
                  } => break 'main,
                  glium::glutin::WindowEvent::KeyboardInput { input, ..  } => {
                    match input.virtual_keycode {
                      Some(virtual_keycode) if input.state == glium::glutin::ElementState::Pressed => {
                        let maybe_string_keycode = virtual_keycode_to_string(virtual_keycode);
                          if let (Some(ref string_keycode), Command::Node(command_node)) = (maybe_string_keycode, selected_command) {
                            let found_child = command_node.children.iter().find(|&child| {
                              match child {
                                Command::Node(child_node) => child_node.key == *string_keycode,
                                Command::Leaf(child_leaf) => child_leaf.key == *string_keycode
                              }
                            });
                            match found_child {
                                Some(found_child @ Command::Node(_)) => selected_command = found_child,
                                Some(Command::Leaf(child_leaf)) => {
                                    CliCommand::new("sh")
                                        .arg("-c")
                                        .arg(&child_leaf.cmd)
                                        .spawn()
                                        .expect("process failed to execute");
                                    break 'main;
                                }
                                None => {}
                            }
                        };
                      }
                      _ => (),
                    }
                  }
                  _ => (),
              },
              _ => (),
          }
      }

      set_ui(ui.set_widgets(), &selected_command, &ids);


      // Render the `Ui` and then display it on the screen.
      if let Some(primitives) = ui.draw_if_changed() {
          renderer.fill(&display, primitives, &image_map);
          let mut target = display.draw();
          target.clear_color(0.0, 0.0, 0.0, 1.0);
          renderer.draw(&display, &mut target, &image_map).unwrap();
          target.finish().unwrap();
      }
  }
}

/**
 * Generates a vector of CommandLeafs from a command to display it as a list.
 */
fn flatten_command_to_leafs(command: &Command) -> Vec<CommandDisplay> {
  match command {
    Command::Leaf(command_leaf) => {
      vec![CommandDisplay::from(command_leaf)]
    }
    Command::Node(command_node) => {
        command_node.children.iter().map(|child| {
            match child {
                Command::Leaf(child_leaf) => CommandDisplay::from(child_leaf),
                Command::Node(child_node) => CommandDisplay::from(child_node)
            }
        }).collect()
    }
  }
}

fn virtual_keycode_to_string(virtual_keycode: glium::glutin::VirtualKeyCode) -> Option<String> {
  match virtual_keycode {
    glium::glutin::VirtualKeyCode::A => Some(String::from("a")),
    glium::glutin::VirtualKeyCode::B => Some(String::from("b")),
    glium::glutin::VirtualKeyCode::C => Some(String::from("c")),
    glium::glutin::VirtualKeyCode::D => Some(String::from("d")),
    glium::glutin::VirtualKeyCode::E => Some(String::from("e")),
    glium::glutin::VirtualKeyCode::F => Some(String::from("f")),
    glium::glutin::VirtualKeyCode::G => Some(String::from("g")),
    glium::glutin::VirtualKeyCode::H => Some(String::from("h")),
    glium::glutin::VirtualKeyCode::I => Some(String::from("i")),
    glium::glutin::VirtualKeyCode::J => Some(String::from("j")),
    glium::glutin::VirtualKeyCode::K => Some(String::from("k")),
    glium::glutin::VirtualKeyCode::L => Some(String::from("l")),
    glium::glutin::VirtualKeyCode::M => Some(String::from("m")),
    glium::glutin::VirtualKeyCode::N => Some(String::from("n")),
    glium::glutin::VirtualKeyCode::O => Some(String::from("o")),
    glium::glutin::VirtualKeyCode::P => Some(String::from("p")),
    glium::glutin::VirtualKeyCode::Q => Some(String::from("q")),
    glium::glutin::VirtualKeyCode::R => Some(String::from("r")),
    glium::glutin::VirtualKeyCode::S => Some(String::from("s")),
    glium::glutin::VirtualKeyCode::T => Some(String::from("t")),
    glium::glutin::VirtualKeyCode::U => Some(String::from("u")),
    glium::glutin::VirtualKeyCode::V => Some(String::from("v")),
    glium::glutin::VirtualKeyCode::W => Some(String::from("w")),
    glium::glutin::VirtualKeyCode::X => Some(String::from("x")),
    glium::glutin::VirtualKeyCode::Y => Some(String::from("y")),
    glium::glutin::VirtualKeyCode::Z => Some(String::from("z")),
    _ => None
  }
}


// Declare the `WidgetId`s and instantiate the widgets.
fn set_ui(ref mut ui: conrod::UiCell, command: &Command, ids: &Ids) {
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    widget::Canvas::new().color(conrod::color::DARK_CHARCOAL).set(ids.canvas, ui);

    let displayed_leafs = flatten_command_to_leafs(command);

    let (mut items, scrollbar) = widget::List::flow_down(displayed_leafs.len())
        .item_size(50.0)
        .scrollbar_on_top()
        .middle_of(ids.canvas)
        .wh_of(ids.canvas)
        .set(ids.list, ui);

    while let Some(item) = items.next(ui) {
        let i = item.i;
        let label = &displayed_leafs[i].name;
        let toggle = widget::Toggle::new(true)
            .label(label)
            .label_color(conrod::color::WHITE)
            .color(conrod::color::LIGHT_BLUE);
        item.set(toggle, ui);
    }

    if let Some(s) = scrollbar { s.set(ui) }
}

