mod force_field;
mod grid;

use grid::Grid;
use nannou::prelude::*;

fn main() {
    nannou::app(setup).update(update).run();
}
const HEIGHT: u32 = 700;
const WIDTH: u32 = 700;
const GRID_SIZE: (usize, usize) = (8, 8);

struct Model {
    grid: Grid,
}

fn setup(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .expect("failed to build window");

    Model {
        grid: Grid::new(GRID_SIZE, app.window_rect()),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

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
