use nannou::prelude::*;

fn main() {
    nannou::app(model).simple_window(view).run();
}

const GRID_SIZE: (usize, usize) = (8, 8);

struct ForceField {
    rect: Rect,
    // force: Vec2,
}

struct Grid {
    fields: Vec<ForceField>,
}

impl Grid {
    fn new(grid_size: (usize, usize), window_rect: Rect) -> Self {
        let mut fields = vec![];
        let top_left = window_rect.top_left();
        let bottom_right = window_rect.bottom_right();
        let size = window_rect.wh() / vec2(grid_size.0 as f32, grid_size.1 as f32);

        let step_x = size.x as usize;
        let step_y = size.y as usize;

        for x in ((top_left.x.floor() as isize)..=(bottom_right.x.ceil() as isize)).step_by(step_x)
        {
            for y in
                ((bottom_right.y.floor() as isize)..=(top_left.y.ceil() as isize)).step_by(step_y)
            {
                let xy = vec2(x as f32, y as f32);

                let rect = Rect::from_xy_wh(xy, size);
                let force_field = ForceField { rect };
                fields.push(force_field);
            }
        }

        Grid { fields }
    }
}

struct Model {
    grid: Grid,
}

fn model(app: &App) -> Model {
    Model {
        grid: Grid::new(GRID_SIZE, app.window_rect()),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    for field in model.grid.fields.iter() {
        draw.rect()
            .wh(field.rect.wh())
            .xy(field.rect.xy())
            .stroke_weight(1.0)
            .stroke_color(BLACK)
            .color(WHITESMOKE);
    }

    // set background to blue
    draw.background().color(BLUE);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
