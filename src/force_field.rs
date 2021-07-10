use nannou::prelude::*;

#[derive(Debug)]
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
    }
}
