#[macro_use] extern crate conrod;
extern crate find_folder;

mod commands;

use conrod::{Ui};
use conrod::backend::glium::glium::{self, Surface};
use commands::{Command, CommandNode, CommandLeaf};


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

/**
 * Generates a vector of CommandLeafs from a command to display it as a list.
 */
fn flatten_command_to_leafs(command: &Command) -> Vec<CommandLeaf> {
  match command {
    Command::Leaf(command_leaf) => {
      vec![command_leaf.clone()]
    }
    Command::Node(command_node) => {
      match command_node.children {
        Some(ref children) => {
          children.iter().map(|child| {
            match child {
              Command::Leaf(child_leaf) => child_leaf.clone(),
              Command::Node(child_node) => CommandLeaf::from(child_node)
            }
          }).collect()
        }
        None => {
          vec![CommandLeaf::from(command_node)]
        }
      }
    }
  }
}

fn main() {
  const WIDTH: u32 = 150;
  const HEIGHT: u32 = 600;

  // --- Setup Commands
  let loaded_commands = commands::get_commands().unwrap();
  println!("Commands Loaded!");

  // --- Setup Conrod
  let mut events_loop = glium::glutin::EventsLoop::new();
  let window = glium::glutin::WindowBuilder::new()
                  .with_title("Hello Conrod")
                  .with_dimensions((WIDTH, HEIGHT).into());
  let context = glium::glutin::ContextBuilder::new()
                  .with_vsync(true)
                  .with_multisampling(4);
  let display = glium::Display::new(window, context, &events_loop).unwrap();

  let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
  let ids = Ids::new(ui.widget_id_generator());

  let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

  // Add a `Font` to the `Ui`'s `font::Map` from file.
  let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
  let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
  ui.fonts.insert_from_file(font_path).unwrap();


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
                  _ => (),
              },
              _ => (),
          }
      }

      set_ui(ui.set_widgets(), &loaded_commands, &ids);


      // Render the `Ui` and then display it on the screen.
      if let Some(primitives) = ui.draw_if_changed() {
          renderer.fill(&display, primitives, &image_map);
          let mut target = display.draw();
          target.clear_color(0.0, 0.0, 0.0, 1.0);
          renderer.draw(&display, &mut target, &image_map).unwrap();
          target.finish().unwrap();
      }
  }


  // let mut vbox = VerticalBox::new(&ui);
  // vbox.set_padded(&ui, true);
  //
  // let mut button = Button::new(&ui, "Button");
  // button.on_clicked(&ui, {
  //   let ui = ui.clone();
  //   move |btn| {
  //     btn.set_text(&ui, "Clicked!");
  //   }
  // });
  //
  // let mut quit_button = Button::new(&ui, "Quit");
  // quit_button.on_clicked(&ui, {
  //   let ui = ui.clone();
  //   move |_| {
  //     ui.quit();
  //   }
  // });
  //
  // vbox.append(&ui, button, LayoutStrategy::Compact);
  // vbox.append(&ui, quit_button, LayoutStrategy::Compact);
  //

  // let command_labels: Vec<Label> = loaded_commands.iter().map(|command| {
  //   Label::new(&ui, command[1])
  // }).collect::<Vec<_>>();

  // let command_labels: Vec<Label> = example_commands.iter().map(|command| {
  //   Label::new(&ui, command[1])
  // }).collect::<Vec<_>>();

  // let command_labels = labels_from_command(&ui, loaded_commands);
  // for label in command_labels {
  //   vbox.append(&ui, label, LayoutStrategy::Compact);
  // }
  //
  // win.set_child(&ui, vbox);
  // win.show(&ui);
  // ui.main();
}

// Declare the `WidgetId`s and instantiate the widgets.
fn set_ui(ref mut ui: conrod::UiCell, command: &Command, ids: &Ids) {
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    widget::Canvas::new().color(conrod::color::DARK_CHARCOAL).set(ids.canvas, ui);

    let displayed_leafs = flatten_command_to_leafs(command);
    println!("displayed leafs {}", displayed_leafs.len());

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

