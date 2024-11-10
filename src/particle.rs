use speedy2d::dimen::Vec2;

pub struct Particle {
    pub position: Vec2,
    previous_position: Vec2,
    acceleration: Vec2,
    pub is_pinned: bool,
}

impl Particle {
    pub fn from_xy(x: f32, y: f32, is_pinned: bool) -> Self {
        Self {
            position: Vec2::new(x, y),
            previous_position: Vec2::new(x, y),
            acceleration: Vec2::ZERO,
            is_pinned,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        if !self.is_pinned {
            self.acceleration += force;
        }
    }

    pub fn update(&mut self, time_step: f32) {
        // verlet integration
        if !self.is_pinned {
            let velocity = self.position - self.previous_position;
            self.previous_position = self.position;
            self.position += velocity + self.acceleration * time_step * time_step;
            self.acceleration = Vec2::ZERO; // reset after update
        }
    }

    pub fn constrain_to_bounds(&mut self, width: f32, height: f32) {
        if self.position.x < 0. {
            self.position.x = 0.;
        }
        if self.position.x > width {
            self.position.x = width;
        }
        if self.position.y < 0. {
            self.position.y = 0.;
        }
        if self.position.y > height {
            self.position.y = height;
        }
    }
}
