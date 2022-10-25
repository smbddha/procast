// extern crate graphics;

// use piston::*;
use graphics::*;
use opengl_graphics::GlGraphics;
use rand::Rng;
use std::f64::consts::PI;

use collider::Collider;
use movable_body::MovableBody;
use proc::*;
use traits::*;

use crate::movable_body::BodyState;

pub enum AsteroidSizeClass {
    Small,
    Medium,
    Large,
}

pub struct Asteroid {
    id: u32, // this will be the corresponding pid
    size: f64,
    size_class: AsteroidSizeClass,
    pub c: Collider,
    pub b: MovableBody,
}

const ASTEROID_SPLIT_FACTOR: u32 = 4;
const ASTEROID_SPLIT_VARIANCE: f64 = 2.0;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.8];
const C3: [f32; 4] = [0.5, 0.5, 0.0, 0.4];
impl GameObject for Asteroid {
    fn update(&mut self, dt: f64) {
        // if asteroid is outside of bounds by a certain distance
        // it should respawn as a new asteroid elsewhere

        self.b.update(dt);
        self.c.p = self.b.p;
    }
    fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
        if (self.b.state == BodyState::Dead) {
            return;
        }
        // let verts: [Vector2<f64>; 4] = [[0.0, 0.0], [0.0, 20.0], [20.0, 20.0], [20.0, 0.0]];
        let transform = ctx
            .transform
            .trans(self.b.p[0], self.b.p[1])
            .rot_rad(self.b.r)
            .trans(-20.0, -20.0);

        // Draw a box rotating around the middle of the screen.
        rectangle(BLACK, [0.0, 0.0, 40.0, 40.0], transform, gl);

        // polygon(RED, &verts[..], transform, gl)
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

impl Collides for Asteroid {
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
        ColliderType::Asteroid
    }
}

impl Asteroid {
    pub fn new(
        body: MovableBody,
        collider: Collider,
        size_class: AsteroidSizeClass,
        pid: u32,
    ) -> Asteroid {
        Asteroid {
            id: pid,
            b: body,
            c: collider,
            size: 50.0,
            size_class,
        }
    }

    pub fn explode_asteroid(&mut self) -> Option<Vec<Asteroid>> {
        // println!("HEREEEE");
        match self.size_class {
            AsteroidSizeClass::Small => {
                // should be destroyed ...
                self.b.state = BodyState::Dead;
                self.c.off();

                return None;
            }
            AsteroidSizeClass::Medium => {
                // split into several small asteroids
                // return (..ASTEROID_SPLIT_FACTOR).map(|i| {});
                // return Some((..ASTEROID_SPLIT_FACTOR).map(|i| Asteroid::new(self.window_size)));
                return Some(Vec::new());
            }
            AsteroidSizeClass::Large => {
                // split into a combination of medium and small asteroids
                // return (..ASTEROID_SPLIT_FACTOR).map(|i| {});
                return Some(Vec::new());
            }
        }

        // remove asteroid from list if necessary
    }
}

pub struct AsteroidManager {
    window_size: [f64; 2],
    pub asteroids: Vec<Asteroid>,
    available_pids: Vec<Proc>,
}

impl AsteroidManager {
    pub fn new(window_size: [f64; 2]) -> AsteroidManager {
        // TODO initially spawn several asteroids, but none should
        // should be where the player spawns (center of screen)
        let mut rng = rand::thread_rng();
        AsteroidManager {
            window_size,
            asteroids: (1..10)
                .map(|_| {
                    let p: [f64; 2] = [
                        rng.gen_range(0.0..window_size[0]),
                        rng.gen_range(0.0..window_size[1]),
                    ];
                    let mut mb = MovableBody::new(p, window_size);
                    mb.v = [rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0)];
                    mb.rv = rng.gen_range(-10.0..10.0);

                    Asteroid::new(mb, Collider::new(p, 25.0), AsteroidSizeClass::Small)
                })
                .collect(),

            available_pids: Vec::new(),
        }
    }

    pub fn add_asteroid(&mut self, a: Asteroid) {
        self.asteroids.push(a)
    }

    pub fn add_pid_asteroid(&mut self, pid: u32) {
        let p: [f64; 2] = [
            rng.gen_range(0.0..window_size[0]),
            rng.gen_range(0.0..window_size[1]),
        ];
        let mut mb = MovableBody::new(p, self.window_size);
        mb.v = [rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0)];
        mb.rv = rng.gen_range(-10.0..10.0);

        self.asteroids.push(Asteroid::new(
            mb,
            Collider::new(p, 25.0),
            AsteroidSizeClass::Small,
            pid,
        ))
    }

    pub fn update(&mut self, dt: f64) {
        // check for asteroids that have gone out of bounds
        // and give them a new position and trajectory
        let n = self.asteroids.len();
        let n_alive = self
            .asteroids
            .iter()
            .filter(|a| a.b.state == BodyState::Live)
            .count();

        // if n_alive is less than desired spawn more asteroids ...
    }

    pub fn get_available_pids(&mut self) {
        self.available_pids = get_procs();
    }
}
