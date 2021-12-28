#![allow(dead_code)]

#[macro_use]
pub mod constants;

pub mod frame;
pub mod animation;
pub mod states;
pub mod stateticker;
pub mod fighter;
pub mod stage;

use graphics::math::Matrix2d;
use graphics::Graphics;

pub trait Drawable {
    fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G);
}

pub trait StageCollideable {}
