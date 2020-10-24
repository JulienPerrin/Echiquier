#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;

extern crate find_folder;
extern crate glium;

mod utils;

mod echiquier;
mod outside;
mod piece;

use conrod_core::image::Map;
use conrod_core::Ui;
use conrod_core::UiBuilder;
use conrod_glium::Renderer;
use glium::glutin::event_loop::EventLoop;
use glium::Display;
use glium::Surface;

use glium::glutin::event::Event::WindowEvent;
use glium::glutin::event::KeyboardInput;
use glium::glutin::event::VirtualKeyCode::Escape;

use outside::all_widgets;

use echiquier::case::Case;
use echiquier::echiquier::Echiquier;
use piece::piece::Piece;
use piece::pion::Pion;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    build_chessboard();
    run_conrod();
}

fn build_chessboard() {
    let echiquier: Echiquier = Echiquier::new();
    echiquier.to_string();
    let e2: Case = echiquier.get_case(1, 4);
    let e4: Case = echiquier.get_case(3, 4);
    let mut pion_e: Pion = Pion::new(e2, echiquier);
    pion_e.move_piece(e4);
}

fn run_conrod() {
    let event_loop: EventLoop<()> = EventLoop::new();
    let display: Display = build_display(&event_loop);
    let mut user_interface: Ui = construct_user_interface();
    let assets = find_folder::Search::KidsThenParents(2, 3)
        .for_folder("assets")
        .unwrap();
    add_fonts(&mut user_interface, assets);

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = Map::<glium::texture::Texture2d>::new();

    // Instantiate the generated list of widget identifiers.
    let mut ids = all_widgets::Ids::new(user_interface.widget_id_generator());

    run_event_loop(
        event_loop,
        display,
        user_interface,
        ids,
        renderer,
        image_map,
    );
}

fn build_display(event_loop: &EventLoop<()>) -> Display {
    let window = glium::glutin::window::WindowBuilder::new()
        .with_title("Echiquier")
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(WIDTH, HEIGHT));
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    return glium::Display::new(window, context, event_loop).unwrap();
}

fn construct_user_interface() -> Ui {
    return UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
}

fn add_fonts(ui: &mut Ui, assets: std::path::PathBuf) {
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();
}

fn run_event_loop(
    event_loop: EventLoop<()>,
    display: Display,
    mut user_interface: Ui,
    mut ids: all_widgets::Ids,
    mut renderer: Renderer,
    image_map: Map<glium::texture::Texture2d>,
) {
    // Poll events from the window.
    utils::run_loop(display, event_loop, move |request, display| {
        match request {
            utils::Request::Event {
                event,
                should_update_ui,
                should_exit,
            } => {
                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = utils::convert_event(&event, &display.gl_window().window()) {
                    user_interface.handle_event(event);
                    *should_update_ui = true;
                }

                match event {
                    WindowEvent { event, .. } => match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::event::WindowEvent::CloseRequested
                        | glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(Escape),
                                    ..
                                },
                            ..
                        } => *should_exit = true,
                        _ => {}
                    },
                    _ => {}
                }
            }
            utils::Request::SetUi { needs_redraw } => {
                // Instantiate all widgets in the GUI.
                all_widgets::set_widgets(user_interface.set_widgets(), &mut ids);

                *needs_redraw = user_interface.has_changed();
            }
            utils::Request::Redraw => {
                // Render the `Ui` and then display it on the screen.
                let primitives = user_interface.draw();

                renderer.fill(display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    })
}
