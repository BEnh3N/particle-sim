use std::{cell::RefCell, rc::Rc};

use crate::particle::Particle;

pub struct Constraint {
    pub p1: Rc<RefCell<Particle>>,
    pub p2: Rc<RefCell<Particle>>,
    initial_length: f32,
    pub active: bool,
}

impl Constraint {
    pub fn new(p1: Rc<RefCell<Particle>>, p2: Rc<RefCell<Particle>>) -> Self {
        let initial_length = (p1.borrow().position - p2.borrow().position).magnitude();
        Self {
            p1,
            p2,
            initial_length,
            active: true,
        }
    }

    pub fn satisfy(&mut self) {
        if !self.active {
            return;
        }

        let mut p1 = self.p1.borrow_mut();
        let mut p2 = self.p2.borrow_mut();

        let delta = p2.position - p1.position;
        let current_length = delta.magnitude();
        let difference = (current_length - self.initial_length) / current_length;
        let correction = delta * 0.5 * difference;

        if !p1.is_pinned {
            p1.position += correction;
        }
        if !p2.is_pinned {
            p2.position -= correction;
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}
