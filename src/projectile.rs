use graphics::*;
use opengl_graphics::GlGraphics;
use std::f64::consts::PI;

use collider::Collider;
use movable_body::*;
use traits::*;
use vecmath::Vector2;

#[derive(Clone, Copy)]
pub struct Projectile {
    pub c: Collider,
    pub b: MovableBody,
}

const PROJECTILE_RADIUS: f64 = 5.0;

const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 0.8];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const PROJECTILE_SPEED: f64 = 200.0;
const C3: [f32; 4] = [0.5, 0.5, 0.0, 0.4];

impl GameObject for Projectile {
    fn update(&mut self, dt: f64) {
        if (self.b.state == BodyState::Dead) {
            return;
        }
        self.b.update(dt);
        self.c.p = self.b.p;
    }
    fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
        if (self.b.state == BodyState::Dead) {
            return;
        }
        let square = rectangle::square(0.0, 0.0, 5.0);
        let transform = ctx
            .transform
            .trans(self.b.p[0], self.b.p[1])
            .rot_rad(0.0)
            .trans(-2.5, -2.5);

        rectangle(RED, square, transform, gl);
    }
    fn render_debug(&self, ctx: &Context, gl: &mut GlGraphics) {
        if (self.b.state == BodyState::Dead) {
            return;
        }
        let transform = ctx.transform.trans(self.b.p[0], self.b.p[1]);
        circle_arc(
            C3,
            0.5,
            0.0,
            2.0 * PI,
            [0.0, 0.0, self.c.r * 2.0, self.c.r * 2.0],
            transform.trans(-self.c.r, -self.c.r),
            gl,
        )
    }
}

impl Collides for Projectile {
    fn collides_with<C: Collides>(&self, other: &C) -> bool {
        self.c.are_colliding(other.get_collider())
    }

    fn get_collider(&self) -> &Collider {
        &self.c
    }

    fn on_collision<C: Collides>(&mut self, other: &C) {
        let t = other.collider_type();
        match (t) {
            ColliderType::Asteroid => (), // trigger end game,
            _ => (),
        }
    }

    fn collider_type(&self) -> ColliderType {
        ColliderType::Projectile
    }
}

impl Projectile {
    pub fn new(body: MovableBody, collider: Collider) -> Projectile {
        Projectile {
            b: body,
            c: collider,
        }
    }
    pub fn destroy(&mut self) {
        self.b.state = BodyState::Dead;
    }
}

pub struct ProjectileManager {
    window_size: [f64; 2],
    pub projectiles: Vec<Projectile>,
}

impl ProjectileManager {
    pub fn new(window_size: [f64; 2]) -> ProjectileManager {
        ProjectileManager {
            projectiles: Vec::new(),
            window_size,
        }
    }

    pub fn spawn_projectile(&mut self, p: Vector2<f64>, r: f64) {
        let mut b = MovableBody::new(p, self.window_size);
        b.v = [PROJECTILE_SPEED * r.sin(), PROJECTILE_SPEED * r.cos()];
        b.boundary_mode = BoundaryModeEnum::None;

        let p = Projectile::new(b, Collider::new(p, PROJECTILE_RADIUS));
        self.projectiles.push(p);
    }
    pub fn purge(&mut self) {
        self.projectiles.retain(|p| p.b.state == BodyState::Live);
    }
}
