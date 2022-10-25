use std::f64::consts::PI;

use vecmath as vm;

#[derive(Copy, Clone)]
pub enum BoundaryModeEnum {
    Wrap,
    None,
}

#[derive(Copy, Clone, PartialEq)]
pub enum BodyState {
    Live, // an active body ... keep rendering
    Dead, // has been destroyed in game ... don't render
}

#[derive(Copy, Clone)]
pub struct MovableBody {
    pub p: vm::Vector2<f64>, // position vector
    pub v: vm::Vector2<f64>, // velocity vector
    pub a: vm::Vector2<f64>, // acceleration vector
    pub r: f64,              // rotation angle
    pub rv: f64,             // rotation velocity
    pub window_size: [f64; 2],
    pub boundary_mode: BoundaryModeEnum,
    pub state: BodyState,
}

impl MovableBody {
    pub fn new(p: [f64; 2], window_size: [f64; 2]) -> MovableBody {
        MovableBody {
            p,
            v: [0.0, 0.0],
            a: [0.0, 0.0],
            r: 0.0,
            rv: 0.0,
            boundary_mode: BoundaryModeEnum::Wrap,
            window_size,
            state: BodyState::Live,
        }
    }
    pub fn update(&mut self, dt: f64) {
        let [mut px, mut py] = vm::vec2_add(self.p, vm::vec2_scale(self.v, dt));

        match self.boundary_mode {
            BoundaryModeEnum::Wrap => {
                if px > self.window_size[0] {
                    px = 0.0;
                }
                if px < 0.0 {
                    px = self.window_size[0];
                }
                if py > self.window_size[1] {
                    py = 0.0;
                }
                if py < 0.0 {
                    py = self.window_size[1];
                }

                self.p = [px, py]
            }
            BoundaryModeEnum::None => self.p = [px, py],
        }

        self.v = vm::vec2_add(self.v, vm::vec2_scale(self.a, dt));
        // self.a = vm::vec2_add(self.a, vm::vec2_scale(self.f, dt));
        self.r = (self.r + (self.rv * dt)) % (2.0 * PI)
    }

    pub fn apply_force(&mut self, f: vm::Vector2<f64>) {
        self.a = f
    }
}
