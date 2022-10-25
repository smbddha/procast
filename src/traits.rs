use graphics::Context;
use opengl_graphics::GlGraphics;

use collider::Collider;

pub enum ColliderType {
    Asteroid,
    Projectile,
    Player,
}

pub trait Collides {
    fn collides_with<C: Collides>(&self, other: &C) -> bool;
    fn get_collider(&self) -> &Collider;
    fn on_collision<C: Collides>(&mut self, other: &C);
    fn collider_type(&self) -> ColliderType;
}

pub trait GameObject {
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics);
    fn render_debug(&self, ctx: &Context, gl: &mut GlGraphics) {}
    fn update(&mut self, _: f64) {}
}
