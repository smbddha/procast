use vecmath as vm;

pub struct MovableBody {
    pub p: vec2,
    v: vec2,
    a: vec2,
    f: vec2,
}

impl MovableBody {
    fn update(&mut self, dt: f32) {
        self.p = vm::vec2_add(self.p, vm::vec2_scale(self.v, dt));
        self.v = vm::vec2_add(self.v, vm::vec2_scale(self.a, dt));
        self.a = vm::vec2_add(self.a, vm::vec2_scale(self.f, dt));
    }

    fn apply_force(&mut self, f: vec2) {
        self.f = f
    }
}
