use super::{HEIGHT, WIDTH};
use nannou::prelude::*;
pub struct Particle {
    pub position: Vec2,
    velocity: Vec2,
    pub acceleration: Vec2,
    // mass: f32,
}

impl Particle {
    pub fn new() -> Particle {
        let position = {
            let random_x = (random_f32() * 2.0 - 1.0) * (WIDTH as f32 / 4.0);
            let random_y = (random_f32() * 2.0 - 1.0) * (HEIGHT as f32 / 4.0);
            vec2(random_x, random_y)
        };

        Particle {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .wh(vec2(20.0, 20.0))
            .color(CRIMSON);
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration * 2.0;
        self.position += self.velocity;
        self.acceleration = Vec2::ZERO;

        // speed limit
        self.velocity.clamp(vec2(-0.1, -0.1), vec2(0.1, 0.1));
    }
}
