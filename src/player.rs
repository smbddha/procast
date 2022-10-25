use graphics::*;
use opengl_graphics::GlGraphics;
use std::f64::consts::PI;
use std::rc::Rc;
use vecmath::*;

use asteroid::*;
use collider::*;
use movable_body::*;
use projectile::*;
use traits::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RotationState {
    Positive,
    Negative,
    None,
}

pub struct Player {
    pub c: Collider,
    pub b: MovableBody,
    pub thrust: bool,
    pub rotating: RotationState,
    // pm: Rc<ProjectileManager>,
}

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const C2: [f32; 4] = [1.0, 1.0, 1.0, 0.4];
const C3: [f32; 4] = [0.0, 0.0, 0.0, 0.4];
const ROTATION_VELOCITY: f64 = 2.4;
const THRUST_FORCE: f64 = 100.0;

impl GameObject for Player {
    fn update(&mut self, dt: f64) {
        match self.rotating {
            RotationState::Positive => self.b.rv = ROTATION_VELOCITY,
            RotationState::Negative => self.b.rv = -1.0 * ROTATION_VELOCITY,
            RotationState::None => self.b.rv = 0.0,
        }

        let mut force_vector: Vector2<f64> = [0.0, 0.0];
        if self.thrust {
            force_vector = [THRUST_FORCE * self.b.r.sin(), THRUST_FORCE * self.b.r.cos()]
        }

        self.b.apply_force(force_vector);
        self.b.update(dt);
        self.c.p = self.b.p;

        // println!("{}, {}", self.b.p[0], self.b.p[1])
        // println!("{}", self.thrust);
    }
    fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
        let verts: [Vector2<f64>; 3] = [[0.0, 0.0], [20.0, 0.0], [10.0, 20.0]];
        let transform = ctx
            .transform
            .trans(self.b.p[0], self.b.p[1])
            .rot_rad(-self.b.r)
            .trans(-10.0, -10.0);

        // Draw a box rotating around the middle of the screen.
        // rectangle(RED, square, transform, gl);

        polygon(RED, &verts[..], transform, gl)
    }
    fn render_debug(&self, ctx: &Context, gl: &mut GlGraphics) {
        let transform = ctx.transform.trans(self.b.p[0], self.b.p[1]);
        line(
            RED,
            0.5,
            [0.0, 0.0, self.b.v[0], self.b.v[1]],
            transform,
            gl,
        );

        line(
            C2,
            0.5,
            [0.0, 0.0, 20.0 * self.b.r.sin(), 20.0 * self.b.r.cos()],
            transform,
            gl,
        );

        circle_arc(
            C3,
            0.5,
            0.0,
            2.0 * PI,
            [0.0, 0.0, 20.0, 20.0],
            transform.trans(-10.0, -10.0),
            gl,
        )
    }
}

impl Collides for Player {
    fn collides_with<C: Collides>(&self, other: &C) -> bool {
        self.c.are_colliding(other.get_collider())
    }

    fn get_collider(&self) -> &Collider {
        &self.c
    }

    fn on_collision<C: Collides>(&mut self, other: &C) {
        let t = other.collider_type();
        match t {
            ColliderType::Asteroid => (), // trigger end game,
            _ => (),
        }
    }

    fn collider_type(&self) -> ColliderType {
        ColliderType::Player
    }
}

impl Player {
    pub fn new(
        body: MovableBody,
        collider: Collider,
        // projectile_manager: Rc<ProjectileManager>,
    ) -> Player {
        Player {
            // b: MovableBody::new(x, y, window_size),
            // c: Collider::new([x + 10.0, y + 10.0], 10.0),
            b: body,
            c: collider,
            thrust: false,
            rotating: RotationState::None,
            // pm: projectile_manager,
        }
    }

    pub fn set_thrust(&mut self, b: bool) {
        self.thrust = b;
    }

    pub fn set_rotation(&mut self, state: RotationState) {
        self.rotating = state;
        println!("{:?}", self.rotating)
    }

    pub fn shoot_projectile(&self, projectile_manager: &mut ProjectileManager) {
        projectile_manager.spawn_projectile(self.b.p, self.b.r);
    }
}
