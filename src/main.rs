use std::process::Command as CliCommand;

use conrod::backend::glium::glium::glutin::os::unix::WindowBuilderExt;
use conrod::backend::glium::glium::{self, Surface};
use conrod::backend::glium::Renderer;
use structopt::StructOpt;

use crate::bindings::Shortcut;
use crate::commands::Command;
use crate::event_loop::EventLoop;
use crate::state::{init_variables_form_input, State};
use crate::view::SpacerunEvent::{
    CloseApplication, ExecuteCommandTask, FocusLost, PrevLevelCommand, SelectCommand,
};
use crate::view::{
    handle_event, rendered_elements_height, set_ui, update_initial_window_state,
    update_window_and_window_state, Ids,
};

mod bindings;
mod commands;
mod config;
mod event_loop;
mod state;
mod view;
mod window_position;

static FONT: &[u8] = include_bytes!("../assets/fonts/NotoSans/NotoSans-Regular.ttf");

#[derive(Debug, StructOpt)]
#[structopt(name = "spacerun")]
pub struct Options {
    #[structopt(short = "s", long = "shortcut", parse(try_from_str))]
    initial_shortcut: Option<Shortcut>,
}

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

    // --- Setup Conrod UI
    let mut ui = conrod::UiBuilder::new([
        state.window_dimensions.width,
        state.window_dimensions.height,
    ])
    .build();
    let mut ids = Ids::new(ui.widget_id_generator());
    // Add a `Font` to the `Ui`'s `font::Map` from file.
    ui.fonts
        .insert(conrod::text::Font::from_bytes(FONT).unwrap());
    update_initial_window_state(&mut ui, &mut state, &mut ids);

    // --- Setup Conrod
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Spacerun")
        .with_dimensions(state.window_dimensions)
        // Mainly so that the window gets opened nicely in i3, but there are
        // probably better fitting window types.
        .with_x11_window_type(glium::glutin::os::unix::XWindowType::Utility)
        // Untested as i3 ignores always_on_top, should help usage with other WMs
        .with_always_on_top(true)
        .with_visibility(false)
        .with_decorations(false);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    display.gl_window().show();

    // WindowBuilder has no `with_position`, so we should update the window
    // with its dimensions directly after it was created.
    update_window_and_window_state(state.window_dimensions.height, &mut state, &display, true);

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
                    state.selected_command = new_selected_command.clone();
                    state
                        .selection_path
                        .push(state.selected_command.clone().into());
                    if let Command::Leaf(command_leaf) = &state.selected_command {
                        let x = &command_leaf.cmd.variables;
                        init_variables_form_input(&mut state.form_command_task_variables, x);
                    }
                }
                Some(PrevLevelCommand) => {
                    if state.selection_path.pop().is_some() {
                        let new_command = state.selection_path.iter().try_fold(
                            &state.config.commands,
                            |acc, command_display| {
                                acc.find_child_for_shortcut(&command_display.shortcut)
                            },
                        );
                        if let Some(new_command) = new_command {
                            state.selected_command = new_command.clone();
                        }
                    }
                }
                Some(FocusLost) => {
                    // FIXME LinuCC Find out how Rofi does not lose focus, and implement it here.
                    display
                        .gl_window()
                        .set_cursor_position((0, 0).into())
                        .unwrap();
                }
                Some(ExecuteCommandTask(cmd, variables)) => {
                    let cmd_string = cmd.to_executable_string(&variables);
                    match cmd_string {
                        Ok(cmd_string) => {
                            CliCommand::new("sh")
                                .arg("-c")
                                .arg(cmd_string)
                                .spawn()
                                .expect("process failed to execute");
                            break 'main;
                        },
                        Err(error) => eprintln!("Error replacing cmd task vars! {}", error),
                    }
                }
                Some(CloseApplication) => break 'main,
                None => (),
            }
        }
        // FIXME (PE) To use conrods own events or not?
        //   all its examples use the glium backend events, not the conrod ones for custom event
        //   handling in user code.
        //
        // ui.global_input().events().for_each(|event| {
        //     use conrod::event::Text;
        //     use conrod::event::{Event, Ui};
        //     match event {
        //         Event::Ui(x) => {
        //             match x {
        //                 Ui::Text(widget_id, text_value) => {
        //                     if let Some(widget_id) = widget_id {
        //                         println!("Text Evnet | Input WidgetId {:?} ; {:?}", widget_id, x);
        //                         println!("{:?}", ids
        //                                 .command_variables_form_inputs
        //                                 .iter().any(|input_widget_id| {
        //                                     println!("{:?}", input_widget_id);
        //                                     input_widget_id[widget_id]
        //                                 } ));
        //                         println!("Text Evnet | Input WidgetId {:?} ; {:?}", widget_id, x);
        //                         if state.form_command_task_variables.len() > 0
        //                             && ids
        //                                 .command_variables_form_inputs
        //                                 .iter().any(|input_widget_id| input_widget_id == widget_id)
        //                         {
        //                             println!("Input widget event: ||| {:?}", text_value);
        //                         }
        //                         // Some text input on some widget
        //                     }
        //                 }
        //                 _ => {}
        //             }
        //         }
        //         _ => {}
        //     }
        // });
        render(
            &mut state,
            &mut ui,
            &mut ids,
            &mut renderer,
            &display,
            &image_map,
        );
    }
}

fn render(
    state: &mut State,
    ui: &mut conrod::Ui,
    ids: &mut Ids,
    renderer: &mut Renderer,
    display: &glium::Display,
    image_map: &conrod::image::Map<glium::texture::Texture2d>,
) {
    set_ui(ui.set_widgets(), state, ids);

    // Render the `Ui` and then display it on the screen.
    if let Some(primitives) = ui.draw_if_changed() {
        if let Some(height) = rendered_elements_height(ui, ids) {
            let new_window_height = height;
            update_window_and_window_state(new_window_height, state, &display, false);
        }
        renderer.fill(&display, primitives, &image_map);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer.draw(display, &mut target, &image_map).unwrap();
        target.finish().unwrap();
    }
}
