use std::{cell::RefCell, rc::Rc};

use constraint::Constraint;
use particle::Particle;
use speedy2d::{
    color::Color,
    dimen::Vec2,
    window::{MouseButton, WindowHandler},
    Window,
};

mod constraint;
mod particle;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const GRAVITY: f32 = 9.81;
const TIME_STEP: f32 = 0.1;

const ROW: i32 = 20;
const COL: i32 = 20;
const REST_DISTANCE: f32 = 30.0;

const CLICK_TOLERANCE: f32 = 30.0;

struct Simulation {
    particles: Vec<Rc<RefCell<Particle>>>,
    constraints: Vec<Constraint>,
    mouse_position: Vec2,
}

impl WindowHandler for Simulation {
    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut speedy2d::Graphics2D,
    ) {
        // apply gravity and update particles
        for particle in &mut self.particles {
            let mut particle = particle.borrow_mut();
            particle.apply_force(Vec2::new_y(GRAVITY));
            particle.update(TIME_STEP);
            particle.constrain_to_bounds(WIDTH as f32, HEIGHT as f32);
        }

        for _ in 0..5 {
            for constraint in &mut self.constraints {
                constraint.satisfy();
            }
        }

        graphics.clear_screen(Color::BLACK);

        // Draw particles as points
        for particle in &self.particles {
            graphics.draw_circle(particle.borrow().position, 0.1, Color::WHITE);
        }

        // Draw constraints as lines
        for constraint in &self.constraints {
            if !constraint.active {
                continue;
            }

            graphics.draw_line(
                constraint.p1.borrow().position,
                constraint.p2.borrow().position,
                2.0,
                Color::WHITE,
            );
        }

        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _helper: &mut speedy2d::window::WindowHelper<()>, position: Vec2) {
        self.mouse_position = position;
    }

    fn on_mouse_button_down(
        &mut self,
        _helper: &mut speedy2d::window::WindowHelper<()>,
        button: speedy2d::window::MouseButton,
    ) {
        if button == MouseButton::Left {
            self.tear_cloth();
        }
    }
}

impl Simulation {
    fn tear_cloth(&mut self) {
        let nearest = self.find_nearest_constraint();
        if let Some(nearest) = nearest {
            nearest.deactivate();
        }
    }

    fn find_nearest_constraint(&mut self) -> Option<&mut Constraint> {
        let mut nearest_constraint = None;
        let mut min_distance = CLICK_TOLERANCE;

        for constraint in &mut self.constraints {
            let distance = Simulation::point_to_segment_distance(
                self.mouse_position.x,
                self.mouse_position.y,
                constraint.p1.borrow().position.x,
                constraint.p1.borrow().position.y,
                constraint.p2.borrow().position.x,
                constraint.p2.borrow().position.y,
            );
            if distance < min_distance {
                min_distance = distance;
                nearest_constraint = Some(constraint);
            }
        }
        nearest_constraint
    }

    fn point_to_segment_distance(px: f32, py: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        let abx = x2 - x1;
        let aby = y2 - y1;

        let apx = px - x1;
        let apy = py - y1;

        let bpx = px - x2;
        let bpy = py - y2;

        let ab_ap = abx * apx + aby * apy;
        let ab_ab = abx * abx + aby * aby;
        let t = ab_ap / ab_ab;

        // project point p onto the line segment ab
        if t < 0.0 {
            // p is closer to a
            (apx * apx + apy * apy).sqrt()
        } else if t > 1.0 {
            // p is closer to b
            (bpx * bpx + bpy * bpy).sqrt()
        } else {
            // projection point is on the segment
            let proj_x = x1 + t * abx;
            let proj_y = y1 + t * aby;
            ((px - proj_x) * (px - proj_x) + (py - proj_y) * (py - proj_y)).sqrt()
        }
    }
}

fn main() {
    let window = Window::new_centered("Cloth Simulation", (WIDTH, HEIGHT)).unwrap();

    let mut particles = vec![];
    let mut constraints = vec![];

    for row in 0..ROW {
        for col in 0..COL {
            let x = col as f32 * REST_DISTANCE + WIDTH as f32 / 3.0;
            let y = row as f32 * REST_DISTANCE + HEIGHT as f32 / 4.0;
            particles.push(Rc::new(RefCell::new(Particle::from_xy(x, y, row == 0))).clone());
        }
    }

    // Initialize constraints
    for row in 0..ROW {
        for col in 0..COL {
            if col < COL - 1 {
                // Horizontal constraint
                constraints.push(Constraint::new(
                    particles[(row * COL + col) as usize].clone(),
                    particles[(row * COL + col + 1) as usize].clone(),
                ));
            }
            if row < ROW - 1 {
                // Vertical constraint
                constraints.push(Constraint::new(
                    particles[(row * COL + col) as usize].clone(),
                    particles[((row + 1) * COL + col) as usize].clone(),
                ));
            }
        }
    }

    window.run_loop(Simulation {
        particles,
        constraints,
        mouse_position: Vec2::ZERO,
    })
}
