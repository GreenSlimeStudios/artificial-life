use macroquad::prelude::{
    clear_background, draw_circle, draw_line, draw_rectangle, draw_text, next_frame, screen_height,
    screen_width, Color, Conf, BLACK, BLUE, DARKGRAY, GREEN, LIGHTGRAY, RED, YELLOW,
};
use rand::*;

const CELL_TYPE_QUANTITY: u32 = 500;

fn window_conf() -> Conf {
    Conf {
        window_title: "Artificial life".to_owned(),
        // fullscreen: true,
        window_height: 1000,
        window_width: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut particles: Vec<Cell> = Vec::new();
    particles.append(&mut create_cells(CELL_TYPE_QUANTITY, Type::RED));
    particles.append(&mut create_cells(CELL_TYPE_QUANTITY, Type::BLUE));
    particles.append(&mut create_cells(CELL_TYPE_QUANTITY, Type::GREEN));
    particles.append(&mut create_cells(CELL_TYPE_QUANTITY, Type::YELLOW));
    let mut rules: Vec<(Type, Type, f32)> = Vec::new();

    rules.push((Type::YELLOW, Type::YELLOW, get_random_value()));
    rules.push((Type::YELLOW, Type::RED, get_random_value()));
    rules.push((Type::YELLOW, Type::GREEN, get_random_value()));
    rules.push((Type::YELLOW, Type::BLUE, get_random_value()));

    rules.push((Type::RED, Type::YELLOW, get_random_value()));
    rules.push((Type::RED, Type::RED, get_random_value()));
    rules.push((Type::RED, Type::GREEN, get_random_value()));
    rules.push((Type::RED, Type::BLUE, get_random_value()));

    rules.push((Type::BLUE, Type::YELLOW, get_random_value()));
    rules.push((Type::BLUE, Type::RED, get_random_value()));
    rules.push((Type::BLUE, Type::GREEN, get_random_value()));
    rules.push((Type::BLUE, Type::BLUE, get_random_value()));

    rules.push((Type::GREEN, Type::YELLOW, get_random_value()));
    rules.push((Type::GREEN, Type::RED, get_random_value()));
    rules.push((Type::GREEN, Type::GREEN, get_random_value()));
    rules.push((Type::GREEN, Type::BLUE, get_random_value()));

    loop {
        clear_background(BLACK);

        let particles_ref = particles.clone();
        for i in 0..particles.len() {
            particles[i].update(&particles_ref, &rules, i);
        }

        for i in 0..particles.len() {
            let color: Color = match particles[i].cell_type {
                Type::RED => RED,
                Type::BLUE => BLUE,
                Type::YELLOW => YELLOW,
                Type::GREEN => GREEN,
            };
            draw_circle(particles[i].x, particles[i].y, 5.0, color);
        }
        next_frame().await
    }
}
fn get_random_value() -> f32 {
    rand::thread_rng().gen_range(-5.0..5.0)
}

#[derive(Clone, Copy)]
struct Cell {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    cell_type: Type,
}
impl Cell {
    fn new(x: f32, y: f32, cell_type: Type) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            cell_type,
        }
    }
    fn update(&mut self, cells: &Vec<Cell>, rules: &Vec<(Type, Type, f32)>, index: usize) {
        let mut fx: f32 = 0.0;
        let mut fy: f32 = 0.0;
        for rule in rules {
            if self.cell_type == rule.0 {
                for i in 0..cells.len() {
                    if cells[i].cell_type == rule.1 && i != index {
                        let dx: f32 = self.x - cells[i].x;
                        let dy: f32 = self.y - cells[i].y;
                        let d: f32 = (dx * dx + dy * dy).sqrt();
                        if d > 0.0 && d < 100.0 {
                            let f: f32 = rule.2 * 1.0 / d;
                            fx += f * dx;
                            fy += f * dy;
                        }
                    }
                }
            }
        }
        // if self.x <= 250.0 || self.x >= 750.0 {
        //     self.vx *= -1.0
        // }
        // if self.y <= 250.0 || self.y >= 750.0 {
        //     self.vy *= -1.0
        // }

        self.vx = (self.vx + fx) * 0.2;
        self.vy = (self.vy + fy) * 0.2;

        if self.x <= 100.0 {
            self.vx = self.vx.abs();
        }
        if self.y <= 100.0 {
            self.vy = self.vy.abs();
        }
        if self.x >= 900.0 {
            self.vx = self.vx.abs() * -1.0;
        }
        if self.y >= 900.0 {
            self.vy = self.vy.abs() * -1.0;
        }

        self.x += self.vx;
        self.y += self.vy;

        // if self.x <= 100.0 {
        //     self.x = 100.0;
        // }
        // if self.x >= 900.0 {
        //     self.x = 900.0;
        // }
        // if self.y <= 100.0 {
        //     self.y = 100.0;
        // }
        // if self.y >= 900.0 {
        //     self.y = 900.0;
        // }
    }
}

fn random() -> f32 {
    rand::thread_rng().gen_range(250.0..750.0)
}
fn create_cells(number: u32, cell_type: Type) -> Vec<Cell> {
    let mut group: Vec<Cell> = Vec::new();
    for i in 0..number {
        group.push(Cell::new(random(), random(), cell_type));
    }
    group
}

#[derive(Clone, Copy, PartialEq)]
enum Type {
    RED,
    BLUE,
    GREEN,
    YELLOW,
}
