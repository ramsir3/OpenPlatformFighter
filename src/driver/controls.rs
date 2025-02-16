#![allow(dead_code)]
use crate::common::states::InputValue;
use piston::input::Key;
use std::collections::HashMap;

pub type Controls = HashMap<Key, InputValue>;

pub fn controls0() -> Controls {
    [].iter().cloned().collect()
}
pub fn controls1() -> Controls {
    [
        (Key::A, InputValue::L),
        (Key::D, InputValue::R),
        (Key::S, InputValue::D),
        (Key::W, InputValue::J | InputValue::U),
        (Key::L, InputValue::J),
        (Key::O, InputValue::B),
        (Key::I, InputValue::A),
        (Key::Space, InputValue::S),
        (Key::J, InputValue::Z),
    ]
    .iter()
    .cloned()
    .collect()
}

pub fn controls2() -> Controls {
    [
        (Key::Left, InputValue::L),
        (Key::Right, InputValue::R),
        (Key::Down, InputValue::D),
        (Key::Up, InputValue::J),
        // (Key::W,     InputValue::U),
    ]
    .iter()
    .cloned()
    .collect()
}
