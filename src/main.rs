use conrod::backend::glium::glium::{self, Surface};
use conrod::backend::glium::glium::glutin::os::unix::WindowBuilderExt;

use crate::view::{handle_event, set_ui, Ids};
use crate::event_loop::EventLoop;
use crate::view::SpacerunEvent::{SelectCommand, CloseApplication};
use crate::state::State;

mod config;
mod commands;
mod bindings;
mod view;
mod event_loop;
mod state;

static FONT: &[u8] = include_bytes!("../assets/fonts/NotoSans/NotoSans-Regular.ttf");

fn main() {
    // --- Setup Commands
    let config = config::load_config()
      .expect("Error loading the config. Check your configuration for inconsistencies.");
    println!("Commands Loaded!");
    println!("{:?}", config.commands);

    let mut state = State::new(config);

    // --- Setup Conrod
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Spacerun")
        .with_dimensions((state.window_width, state.window_height).into())
        // Mainly so that the window gets opened nicely in i3, but there are
        // probably better fitting window types.
        .with_x11_window_type(glium::glutin::os::unix::XWindowType::Dialog)
        // Untested as i3 ignores it, should help usage with other WMs
        .with_always_on_top(true)
        .with_decorations(false);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([state.window_width as f64, state.window_height as f64]).build();
    let mut ids = Ids::new(ui.widget_id_generator());

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
          match handle_event(&event, &state) {
              Some(SelectCommand(new_selected_command)) => {
                state.select_command(new_selected_command.to_owned());
              },
              Some(CloseApplication) => break 'main,
              None => ()
          }
        }

        set_ui(ui.set_widgets(), &state, &state.selected_command, &mut ids);

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();

            if let Some(render_rect) = ui.kids_bounding_box(ids.command_list) {
                let new_window_height = render_rect.h() as u32;
                if new_window_height != state.window_height {
                    println!("Updating window size.");
                    state.set_window_height(new_window_height);
                    display.gl_window().set_inner_size((state.window_width, state.window_height).into());
                }
            }
        }
    }
}
