use std::cmp;

use vecmath::Vector2;

#[derive(Copy, Clone)]
enum ColliderState {
    Enabled,
    Disabled,
}

#[derive(Copy, Clone)]
pub struct Collider {
    state: ColliderState,
    pub p: Vector2<f64>,
    pub r: f64,
}

impl Collider {
    pub fn new(p: Vector2<f64>, r: f64) -> Collider {
        Collider {
            state: ColliderState::Enabled,
            p,
            r,
        }
    }

    // TODO also return points of collision
    pub fn are_colliding(&self, other: &Collider) -> bool {
        match (self.state, other.state) {
            (ColliderState::Disabled, _) => false,
            (_, ColliderState::Disabled) => false,
            (_, _) => {
                let [x1, y1] = self.p;
                let [x2, y2] = other.p;

                // println!(
                //     "{} : {}",
                //     (x2 - x1).powf(2.0) + (y2 - y1).powf(2.0),
                //     self.r.max(other.r).powf(2.0)
                // );
                (x2 - x1).powf(2.0) + (y2 - y1).powf(2.0) <= self.r.max(other.r).powf(2.0)
            }
        }
    }

    pub fn on(&mut self) {
        self.state = ColliderState::Enabled;
    }

    pub fn off(&mut self) {
        self.state = ColliderState::Disabled;
    }
}
