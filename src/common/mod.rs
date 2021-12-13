#![allow(dead_code)]

#[macro_use]
pub mod constants;

pub mod frame;
pub mod animation;
pub mod states;
pub mod stateticker;
pub mod fighter;
pub mod stage;

pub trait Drawable {
    fn draw(&self);
}