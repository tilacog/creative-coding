use crate::particle::Particle;
use nannou::prelude::*;

pub struct ForceField {
    pub rect: Rect,
    pub force: Vec2,
}

impl ForceField {
    pub fn new(rect: Rect) -> ForceField {
        let random_x = random_f32() * 2.0 - 1.0;
        let random_y = random_f32() * 2.0 - 1.0;
        let force = vec2(random_x, random_y).normalize();
        ForceField { rect, force }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.rect()
            .wh(self.rect.wh())
            .xy(self.rect.xy())
            .stroke_weight(1.0)
            .stroke_color(BLACK)
            .color(WHITESMOKE);
        self.draw_force_vector(draw)
    }

    fn draw_force_vector(&self, draw: &Draw) {
        // arrow line
        let color = BLACK;
        let start = self.rect.xy();
        let end = start + self.force * 25.0; // 25 is magic
        draw.line()
            .start(start)
            .end(end)
            .stroke_weight(3.0)
            .color(color);

        // arrow tip
        let tip_size = self.rect.wh() / 12.0; // 12 is magic
        draw.ellipse().xy(start).wh(tip_size).color(color);
    }

    pub fn apply_force(&self, particle: &mut Particle) {
        particle.acceleration += self.force * 0.01;
    }
}
