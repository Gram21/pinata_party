use constants::*;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use piston::input::*;
use random::MTRng32;
use std::path::Path;


pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    rand: MTRng32, // TODO other rng generator?
    cursor_position: (f64, f64),
    timer: f64,
    background: Image,
    bg_texture: Texture,
    aim_texture: Texture,
    evil_texture: Texture,
    hero_texture: Texture,
    evil_targets: Vec<Target>,
    hero_targets: Vec<Target>,
}

impl Game {
    pub fn new(opengl: OpenGL) -> Self {
        let mut rand = MTRng32::new(42); // TODO better seed

        let mut evil_targets: Vec<Target> = Vec::new();
        for _ in 0..3 {
            let t = Target::new_rnd(&mut rand);
            evil_targets.push(t);
        }

        let mut hero_targets: Vec<Target> = Vec::new();
        for _ in 0..3 {
            let t = Target::new_rnd(&mut rand);
            hero_targets.push(t);
        }

        Game {
            gl: GlGraphics::new(opengl),
            rand: rand,
            cursor_position: (WINDOW_SIZE.0 / 2.0, WINDOW_SIZE.1 / 2.0),
            timer: 0.0,
            background: Image::new().rect([0.0, 0.0, WINDOW_SIZE.0, WINDOW_SIZE.1]),
            bg_texture: Texture::from_path(Path::new(TEXTURE_BG)).unwrap(),
            aim_texture: Texture::from_path(Path::new(TEXTURE_AIM)).unwrap(),
            evil_texture: Texture::from_path(Path::new(TEXTURE_TRUMP)).unwrap(),
            hero_texture: Texture::from_path(Path::new(TEXTURE_MEXICAN)).unwrap(),
            evil_targets: evil_targets,
            hero_targets: hero_targets,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {

        let (x, y) = self.cursor_position;
        let bg = self.background;
        let ref bg_texture = self.bg_texture;
        let ref aim_texture = self.aim_texture;
        let ref hero_texture = self.hero_texture;
        let ref evil_texture = self.evil_texture;
        let ref hero_targets = self.hero_targets;
        let ref evil_targets = self.evil_targets;

        self.gl.draw(args.viewport(), |c, gl| {
            use graphics::draw_state::DrawState;

            // Clear the screen and redraw background.
            bg.draw(bg_texture, &DrawState::default(), c.transform, gl);

            // Draw targets
            for target in hero_targets {
                let target_img =
                    Image::new().rect(rectangle::square(target.x, target.y, target.width));
                let transform = c.transform.trans(0.0, 0.0).trans(-25.0, -25.0);
                target_img.draw(hero_texture, &DrawState::default(), transform, gl);
            }

            for target in evil_targets {
                let target_img =
                    Image::new().rect(rectangle::square(target.x, target.y, target.width));
                let transform = c.transform.trans(0.0, 0.0).trans(-25.0, -25.0);
                target_img.draw(evil_texture, &DrawState::default(), transform, gl);
            }

            // Draw a crosshair
            let aim = Image::new().rect(rectangle::square(0.0, 0.0, 50.0));
            let transform = c.transform.trans(x, y).trans(-50.0, -50.0);
            aim.draw(aim_texture, &DrawState::default(), transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.timer += args.dt;
        // update timer. If greater than one a second passed. Reset then.
        if self.timer >= 1.0 {
            self.timer = 0.0;
            self.update_lifetimes();
        }

        // create new targets if old ones died
        while self.hero_targets.len() < 3 {
            self.hero_targets.push(Target::new_rnd(&mut self.rand));
        }
        while self.evil_targets.len() < 3 {
            self.evil_targets.push(Target::new_rnd(&mut self.rand));
        }
    }

    fn update_lifetimes(&mut self) {
        for targets in [&mut self.evil_targets, &mut self.hero_targets].iter_mut() {
            for i in (0..targets.len()).rev() {
                if targets[i].lifetime <= 1 {
                    // Lifetime is over, remove them
                    targets.remove(i);
                } else {
                    targets[i].lifetime -= 1;
                }
            }
        }
    }

    pub fn process_mouse(&mut self, m: &MouseButton) {
        match m {
            &MouseButton::Left => {
                for i in Target::check_for_hit(&mut self.evil_targets, self.cursor_position)
                        .iter()
                        .rev() {
                    self.evil_targets.remove(*i);
                }
                for i in Target::check_for_hit(&mut self.hero_targets, self.cursor_position)
                        .iter()
                        .rev() {
                    self.hero_targets.remove(*i);
                }
            }
            _ => {}
        }
    }

    pub fn process_else(&mut self, inp: &Input) {
        inp.mouse_cursor(|x, y| self.cursor_position = (x + 25.0, y + 25.0)); // cursor offset
    }
}

pub struct Target {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub bounty: u16,
    pub lifetime: u16,
}

impl Target {
    pub fn new(x: f64, y: f64, width: f64, height: f64, bounty: u16, lifetime: u16) -> Self {
        Target {
            x: x,
            y: y,
            width: width,
            height: height,
            bounty: bounty,
            lifetime: lifetime,
        }
    }

    pub fn new_rnd(rand: &mut MTRng32) -> Self {
        //TODO
        let (x, y) = Self::get_rnd_position(rand);
        Target {
            x: x,
            y: y,
            width: 50.0,
            height: 50.0,
            bounty: 30,
            lifetime: 6,
        }
    }

    fn get_rnd_position(rand: &mut MTRng32) -> (f64, f64) {
        // TODO: check if rnd_position is okay
        let x = (rand.rand() % WINDOW_SIZE.0 as u32) as f64;
        let y = (rand.rand() % WINDOW_SIZE.1 as u32) as f64;
        (x, y)
    }

    fn coord_is_inside(&mut self, x: f64, y: f64) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    pub fn check_for_hit(targets: &mut [Target], click: (f64, f64)) -> Vec<usize> {
        let mut indices: Vec<usize> = Vec::new();
        for (i, target) in targets.iter_mut().enumerate() {
            if target.coord_is_inside(click.0, click.1) {
                indices.push(i);
            }
        }
        indices
    }
}
