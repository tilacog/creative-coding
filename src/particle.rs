use nannou::prelude::*;

pub struct Particle {
    pub position: Vec2,
    velocity: Vec2,
    pub acceleration: Vec2,
    // mass: f32,
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            position: Vec2::ZERO,
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
        self.velocity.clamp(vec2(-0.5, -0.5), vec2(0.5, 0.5));
    }
}
