#![allow(dead_code)]
use std::collections::HashMap;
use piston::input::{Key};
use common::state::IVal;

pub type Controls = HashMap<Key, u32>;

pub fn controls0() -> Controls {
    [].iter().cloned().collect()
}
pub fn controls1() -> Controls {
    [
        (Key::A,     IVal::L as u32),
        (Key::D,     IVal::R as u32),
        (Key::S,     IVal::D as u32),
        (Key::W,     IVal::J | IVal::U),
        (Key::L,     IVal::J as u32),
        (Key::O,     IVal::B as u32),
        (Key::I,     IVal::A as u32),
        (Key::Space, IVal::S as u32),
        (Key::J,     IVal::Z as u32),
    ].iter().cloned().collect()
}

pub fn controls2() -> Controls {
    [
        (Key::Left,  IVal::L as u32),
        (Key::Right, IVal::R as u32),
        (Key::Down,  IVal::D as u32),
        (Key::Up,    IVal::J as u32),
        // (Key::W,     IVal::U),
    ].iter().cloned().collect()
}