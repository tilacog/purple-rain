extern crate ggez;
extern crate rand;
use ggez::graphics::{Color, Point2};
use ggez::*;

// TODO: use height and width values from a config file
const HEIGHT: f32 = 600.0;
const WIDTH: f32 = 800.0;

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
    y_speed: f32,
}

impl Drop {
    fn fall(&mut self) {
        self.y = self.y % HEIGHT + self.y_speed;
    }
}

struct MainState {
    rain: Vec<Drop>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let drops = vec![Drop {
            x: 400.0,
            y: 0.0,
            y_speed: 1.0,
        }];
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
            let p2 = Point2::new(drop.x, drop.y + 10.0);
            let points: [Point2; 2] = [p1, p2];
            graphics::line(ctx, &points, 5.0)?;
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
