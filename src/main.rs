mod force_field;
mod grid;
mod particle;

use grid::Grid;
use nannou::prelude::*;
use particle::Particle;

fn main() {
    nannou::app(setup).update(update).run();
}
const HEIGHT: u32 = 1200;
const WIDTH: u32 = 1200;
const GRID_SIZE: (usize, usize) = (32, 32);

struct Model {
    grid: Grid,
    particles: Vec<Particle>,
}

fn setup(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .expect("failed to build window");

    Model {
        grid: Grid::new(GRID_SIZE, app.window_rect()),
        particles: vec![Particle::new()],
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // apply force to particles
    for force_field in &model.grid.fields {
        for mut particle in model.particles.iter_mut() {
            if force_field.rect.contains(particle.position) {
                force_field.apply_force(&mut particle);
            }
        }
    }
    for particle in model.particles.iter_mut() {
        particle.update()
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    model.grid.draw(&draw);
    for particle in &model.particles {
        particle.draw(&draw)
    }

    // set background to blue
    draw.background().color(BLUE);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
