use conrod::backend::glium::glium::{self, Surface};
use conrod::widget_ids;
use conrod::backend::glium::glium::glutin::os::unix::WindowBuilderExt;

use crate::commands::{Command};
use crate::config::{SpacerunConfig};
use crate::view::{handle_event};
use crate::event_loop::EventLoop;
use crate::view::SpacerunEvent::{SelectCommand, CloseApplication};

mod config;
mod commands;
mod bindings;
mod view;
mod event_loop;

widget_ids! {
    struct Ids { canvas, list }
}

static FONT: &[u8] = include_bytes!("../assets/fonts/NotoSans/NotoSans-Regular.ttf");

fn main() {

    const WIDTH: u32 = 500;
    const HEIGHT: u32 = 400;

    // --- Setup Commands
    let config = config::load_config()
      .expect("Error loading the config. Check your configuration for inconsistencies.");
    let mut selected_command: &Command = &config.commands;
    println!("Commands Loaded!");
    println!("{:?}", config.commands);

    // --- Setup Conrod
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Spacerun")
        .with_dimensions((WIDTH, HEIGHT).into())
        // Mainly so that the window gets opened nicely in i3, but there are
        // probably better fitting window types.
        .with_x11_window_type(glium::glutin::os::unix::XWindowType::Toolbar)
        // Untested as i3 ignores it, should help usage with other WMs
        .with_always_on_top(true)
        .with_decorations(false);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    let ids = Ids::new(ui.widget_id_generator());

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    ui.fonts
        .insert(conrod::text::Font::from_bytes(FONT).unwrap());

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
          match handle_event(&event, &selected_command) {
              Some(SelectCommand(new_selected_command)) => {
                selected_command = new_selected_command;
              },
              Some(CloseApplication) => break 'main,
              None => ()
          }
        }

        set_ui(ui.set_widgets(), &config, &selected_command, &ids);

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

// Declare the `WidgetId`s and instantiate the widgets.
fn set_ui(ref mut ui: conrod::UiCell, config: &SpacerunConfig, command: &Command, ids: &Ids) {
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use crate::commands::displayable_command_children;

    widget::Canvas::new()
        .color(conrod::color::DARK_CHARCOAL)
        .set(ids.canvas, ui);

    let displayed_leafs = displayable_command_children(command);

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
            .label_font_size(config.font_size.unwrap_or(14))
            .color(conrod::color::LIGHT_BLUE);
        item.set(toggle, ui);
    }

    if let Some(s) = scrollbar {
        s.set(ui)
    }
}
