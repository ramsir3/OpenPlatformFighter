#[macro_use]
extern crate enum_display_derive;
use glutin_window::GlutinWindow as Window;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{ButtonArgs, ButtonEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::{OpenGLWindow, WindowSettings};
extern crate gl;
#[macro_use]
mod common;
mod driver;
mod fighters;
use common::constants::*;
use common::stage::Stage;
use driver::controls::*;
use driver::player::Player;
use fighters::*;

use std::io::{stdout, Write};

pub struct OPF<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    players: [Option<Player<'a>>; 4],
    stage: Stage<'a>,
}

impl<'a> OPF<'a> {
    #[allow(dead_code)]
    fn setup1(opengl: OpenGL) -> OPF<'a> {
        OPF {
            gl: GlGraphics::new(opengl),
            players: [
                Some(Player::new(test::new(), controls1(), [100.0, 100.0])),
                None,
                None,
                None,
            ],
            stage: Stage::default(),
        }
    }
    #[allow(dead_code)]
    fn setup2(opengl: OpenGL) -> OPF<'a> {
        OPF {
            gl: GlGraphics::new(opengl),
            players: [
                Some(Player::new(test::new(), controls1(), [100.0, 100.0])),
                Some(Player::new(test::new(), controls2(), [175.0, 100.0])),
                None,
                None,
            ],
            stage: Stage::default(),
        }
    }
    #[allow(dead_code)]
    fn setup4(opengl: OpenGL) -> OPF<'a> {
        OPF {
            gl: GlGraphics::new(opengl),
            players: [
                Some(Player::new(test::new(), controls1(), [100.0, 100.0])),
                Some(Player::new(test::new(), controls2(), [125.0, 100.0])),
                Some(Player::new(test::new(), controls0(), [150.0, 100.0])),
                Some(Player::new(test::new(), controls0(), [175.0, 100.0])),
            ],
            stage: Stage::default(),
        }
    }
    fn render(&mut self, args: &RenderArgs) {
        let stage = &self.stage;
        let players = &self.players;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            stage.draw(c.transform, gl);

            for p in players.iter() {
                match p {
                    Some(p) => p.draw(c.transform, gl),
                    None => (),
                }
            }
        });
    }
    fn update_inputs(&mut self, args: &ButtonArgs) {
        for p in self.players.iter_mut() {
            match p {
                Some(p) => p.update_inputs(args),
                None => (),
            }
        }
    }
    fn update(&mut self, args: &UpdateArgs) {
        for p in self.players.iter_mut() {
            match p {
                Some(p) => p.update(args.dt),
                None => (),
            }
        }
    }
    #[allow(dead_code)]
    fn print_player_state(&mut self) {
        for p in self.players.iter_mut() {
            match p {
                Some(p) => {
                    print!("{:}\t", p.get_debug_state());
                }
                None => (),
            }
        }
        print!("\r");
        stdout().flush().ok();
    }
}

fn main() {
    println!("enter main");
    //init
    let opengl = OpenGL::V2_1;

    let mut window: Window = WindowSettings::new("Hello Piston!", WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    // Load the OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // OpenGL::load_with(|s| window.get_proc_address(s));
    let mut opf = OPF::setup4(opengl);
    //game loop
    let mut es = EventSettings::new();
    es.ups = FRAMES_PER_SECOND;
    let mut events = Events::new(es);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.button_args() {
            opf.update_inputs(&args);
        }
        if let Some(args) = e.render_args() {
            opf.render(&args);
        }
        if let Some(args) = e.update_args() {
            opf.update(&args);
            opf.print_player_state();
        }
    }
}
