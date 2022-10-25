extern crate fps_counter;
extern crate glutin_window;
extern crate graphics;
extern crate libproc;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate vecmath;

mod asteroid;
mod collider;
mod movable_body;
mod player;
mod proc;
mod projectile;
mod traits;

use glutin_window::GlutinWindow as GWindow;
use movable_body::MovableBody;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::*;

use asteroid::*;
use collider::*;
use player::*;
use proc::*;
use projectile::*;
use traits::*;

const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

pub struct GameState {
    pub player: Player,
    pub asteroid_manager: AsteroidManager,
    pub projectile_manager: ProjectileManager,
    pub proc_manager: ProcManager,
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    fps: fps_counter::FPSCounter,
}

impl App {
    fn render(&mut self, args: &RenderArgs, game: &GameState) {
        use graphics::*;

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            game.player.render(&c, gl);
            game.player.render_debug(&c, gl);

            for projectile in game.projectile_manager.projectiles.iter() {
                projectile.render(&c, gl);
                projectile.render_debug(&c, gl);
            }

            for asteroid in game.asteroid_manager.asteroids.iter() {
                asteroid.render(&c, gl);
                asteroid.render_debug(&c, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, game: &mut GameState) {
        game.projectile_manager.purge();

        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        game.player.update(args.dt);

        for projectile in game.projectile_manager.projectiles.iter_mut() {
            projectile.update(args.dt);
        }

        for asteroid in game.asteroid_manager.asteroids.iter_mut() {
            asteroid.update(args.dt)
        }

        let mut collisions: Vec<(&mut Projectile, &mut Asteroid)> = Vec::new();
        for projectile in game.projectile_manager.projectiles.iter_mut() {
            for asteroid in game.asteroid_manager.asteroids.iter_mut() {
                if asteroid.collides_with(projectile) {
                    // println!("Player COLLISION");
                    // collisions.push((projectile, asteroid));
                    // let res = asteroid.explode_asteroid();
                    if let Some(new_asteroids) = asteroid.explode_asteroid() {
                        // push new_asteroids somewhere to be added outside of this iter_mut
                    };
                    projectile.destroy();
                };
            }
        }

        game.proc_manager.poll_pids();

        // TODO resolve collisions here ( remove projectiles/asteroids or end game)

        // for &mut (p, a) in collisions.iter_mut() {
        //     // destory projectile
        //     // game.asteroid_manager.explode_asteroid(a)
        //     a.explode_asteroid();
        // }
    }

    fn handle_input(&mut self, args: &ButtonArgs, game: &mut GameState) {
        match args.state {
            ButtonState::Press => match args.button {
                Button::Keyboard(Key::W) => game.player.set_thrust(true),
                Button::Keyboard(Key::A) => {
                    game.player.set_rotation(player::RotationState::Positive)
                }
                Button::Keyboard(Key::D) => {
                    game.player.set_rotation(player::RotationState::Negative)
                }
                Button::Keyboard(Key::Space) => {
                    game.player.shoot_projectile(&mut game.projectile_manager)
                }
                _ => {}
            },
            ButtonState::Release => match args.button {
                Button::Keyboard(Key::W) => game.player.set_thrust(false),
                Button::Keyboard(Key::A) | Button::Keyboard(Key::D) => {
                    game.player.set_rotation(player::RotationState::None)
                }
                _ => {}
            },
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: GWindow = WindowSettings::new("spinning-square", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        fps: fps_counter::FPSCounter::new(),
    };

    let initial_position = [window.size().width / 2.0, window.size().height / 2.0];
    let player_radius = 10.0;

    let collider = Collider::new(initial_position, player_radius);
    let mb = MovableBody::new(
        initial_position,
        [window.size().width, window.size().height],
    );

    let pm = ProjectileManager::new([window.size().width, window.size().height]);
    let am = AsteroidManager::new([window.size().width, window.size().height]);
    let procm = proc::get_proc_manager();

    let mut game = GameState {
        player: Player::new(mb, collider),
        asteroid_manager: am,
        projectile_manager: pm,
        proc_manager: procm,
    };

    println!("{:?}", get_procs());

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &game);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, &mut game);
        }

        if let Some(args) = e.button_args() {
            app.handle_input(&args, &mut game);
        }
    }
}
