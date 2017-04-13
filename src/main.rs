extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use graphics::*;
use std::path::Path;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    cursor_position: (f64, f64),
    timer: f64,
    background: Image,
    bg_texture: Texture,
    aim_texture: Texture,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const ORANGE: [f32; 4] = [0.97, 0.39, 0.2, 1.0];

        let (x, y) = self.cursor_position;
        let bg = self.background;
        let ref bg_texture = self.bg_texture;
        let ref aim_texture = self.aim_texture;

        self.gl
            .draw(args.viewport(), |c, gl| {
                use graphics::draw_state::DrawState;
                // Clear the screen and redraw background.
                clear(ORANGE, gl);
                bg.draw(bg_texture, &DrawState::default(), c.transform, gl);

                // Draw a crosshair
                let aim = Image::new().rect(rectangle::square(0.0, 0.0, 50.0));
                let transform = c.transform
                    .trans(x, y)
                    // .rot_rad(rotation)
                    .trans(-25.0, -25.0);
                aim.draw(aim_texture, &DrawState::default(), transform, gl);
            });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.timer += args.dt;
        if self.timer >= 1.0 {
            println!("A second passed");
            self.timer = 0.0;
        }
    }

    fn process_mouse(&mut self, m: &MouseButton) {
        match m {
            &MouseButton::Left => {
                println!("Click at {:?}", self.cursor_position);
            },
            _ => {},
        }
    }

    fn process_else(&mut self, inp: &Input) {
        inp.mouse_cursor(|x,y| self.cursor_position = (x,y));
    }
}

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
    let mut app = App {
        gl: GlGraphics::new(opengl),
        cursor_position: (240.0,136.0),
        timer: 0.0,
        background: Image::new().rect([0.0, 0.0, 480.0, 272.0]),
        bg_texture: Texture::from_path(Path::new("img/desert.png")).unwrap(),
        aim_texture: Texture::from_path(Path::new("img/aim.png")).unwrap(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(ren) => app.render(&ren),
            Input::Update(upd) => app.update(&upd),
            Input::Press(Button::Mouse(m)) => app.process_mouse(&m),
            _ => app.process_else(&e),
        }
    }
}
