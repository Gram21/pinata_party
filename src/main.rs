extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

pub mod game;
pub mod constants;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Fiesta Pinata", [480, 272])
        .opengl(opengl)
        .fullscreen(false)
        .exit_on_esc(true)
        .build()
        .unwrap();
    // TODO: try to make cursor invisible or give it another style

    // Create a new game and run it.
    let mut game = game::Game::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(ren) => game.render(&ren),
            Input::Update(upd) => game.update(&upd),
            Input::Press(Button::Mouse(m)) => game.process_mouse(&m),
            _ => game.process_else(&e),
        }
    }
}
