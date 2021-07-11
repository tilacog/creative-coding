use crate::force_field::ForceField;
use nannou::noise::*;
use nannou::prelude::*;

pub struct Grid {
    pub fields: Vec<ForceField>,
}

impl Grid {
    pub fn new(grid_size: (usize, usize), window_rect: Rect) -> Self {
        let mut fields = vec![];
        let top_left = window_rect.top_left();
        let bottom_right = window_rect.bottom_right();
        let size = window_rect.wh() / vec2(grid_size.0 as f32, grid_size.1 as f32);
        let step_x = size.x as usize;
        let step_y = size.y as usize;
        let noise = Perlin::new();

        for x in ((top_left.x.floor() as isize)..=(bottom_right.x.ceil() as isize)).step_by(step_x)
        {
            for y in
                ((bottom_right.y.floor() as isize)..=(top_left.y.ceil() as isize)).step_by(step_y)
            {
                let xy = vec2(x as f32, y as f32);

                let rect = Rect::from_xy_wh(xy, size);
                let force = {
                    let noise_x = noise.get([x as f64 * 0.4, y as f64 * 0.4]) as f32;
                    let noise_y = noise.get([0.0, x as f64 * 0.4]) as f32;
                    if let Some(f) = vec2(noise_x, noise_y).try_normalize() {
                        f
                    } else {
                        vec2(1.0, 1.0).normalize()
                    }
                };

                let force_field = ForceField::new(rect, force);
                fields.push(force_field);
            }
        }

        Grid { fields }
    }

    pub fn draw(&self, draw: &Draw) {
        for field in self.fields.iter() {
            field.draw(&draw)
        }
    }
}
