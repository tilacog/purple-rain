extern crate ggez;
extern crate rand;

use ggez::graphics::{Color, Point2};
use ggez::*;
use rand::{thread_rng, Rng};

// TODO: use a config file
const HEIGHT: f32 = 600.0;
const WIDTH: f32 = 800.0;
const NUM_DROPS: u16 = 500;
const ACCELERATION: f32 = 0.15;
const Y_UPPER_BOUND_FACTOR: f32 = 4.0; // how much far off-screen drops can appear
const MIN_Z: f32 = 0.0;
const MAX_Z: f32 = 1.0;
const MIN_Y_SPEED: f32 = 4.0;
const MAX_Y_SPEED: f32 = 10.0;
const MIN_THICKNESS: f32 = 1.0;
const MAX_THICKNESS: f32 = 3.5;
const MIN_LENGTH: f32 = 10.0;
const MAX_LENGTH: f32 = 20.0;

static BG_COLOR: Color = Color {
    r: 0.901960784314, // 230
    g: 0.901960784314, // 230
    b: 0.980392156863, // 250
    a: 1.0,
};

static DROP_COLOR: Color = Color {
    r: 0.541176470588, // 138
    g: 0.168627450980, // 43
    b: 0.886274509804, // 226.0,
    a: 1.0,
};

struct Drop {
    x: f32,
    y: f32,
    /// distant droplets are faster, shorter and thinner.
    z: f32,
    y_speed: f32,
    thickness: f32,
    length: f32,
}

impl Drop {
    /// Create a new Drop at a random position
    fn new() -> Drop {
        let mut rng = thread_rng();
        let x: f32 = rng.gen_range(0.0, WIDTH);
        let y: f32 = {
            let upper_bound = HEIGHT / Y_UPPER_BOUND_FACTOR;
            rng.gen_range(-upper_bound, 0.0)
        };
        let z: f32 = rng.gen_range(MIN_Z, MAX_Z);
        let y_speed: f32 = rng.gen_range(MIN_Y_SPEED, MAX_Y_SPEED) * z;
        let thickness: f32 = rng.gen_range(MIN_THICKNESS, MAX_THICKNESS) * z;
        let length: f32 = rng.gen_range(MIN_LENGTH, MAX_LENGTH) * z;

        Drop {
            x,
            y,
            z,
            y_speed,
            thickness,
            length,
        }
    }

    fn fall(&mut self) {
        self.y = self.y + self.y_speed;
        self.y_speed = self.y_speed + ACCELERATION;

        // reset drop (TODO: try to reuse code from `Drop::new`
        if self.y > HEIGHT {
            let mut rng = thread_rng();
            self.x = rng.gen_range(0.0, WIDTH);
            self.y = {
                let upper_bound = HEIGHT / Y_UPPER_BOUND_FACTOR;
                rng.gen_range(-upper_bound, 0.0)
            };
            self.z = rng.gen_range(MIN_Z, MAX_Z);
            self.y_speed = rng.gen_range(MIN_Y_SPEED, MAX_Y_SPEED) * self.z;
            self.thickness = rng.gen_range(MIN_THICKNESS, MAX_THICKNESS) * self.z;
        }
    }
}

struct MainState {
    rain: Vec<Drop>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut drops: Vec<Drop> = Vec::new();
        for _ in 0..NUM_DROPS {
            drops.push(Drop::new());
        }
        let state = MainState { rain: drops };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for drop in self.rain.iter_mut() {
            drop.fall();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // set background color
        graphics::set_background_color(ctx, BG_COLOR);

        // show drops
        graphics::set_color(ctx, DROP_COLOR)?;
        for drop in &self.rain {
            let p1 = Point2::new(drop.x, drop.y);
            let p2 = Point2::new(drop.x, drop.y + drop.length);
            let points: [Point2; 2] = [p1, p2];
            graphics::line(ctx, &points, drop.thickness)?;
        }

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
