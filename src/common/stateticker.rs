use crate::common::states::{InputValue, VolatileValue};
use std::fmt;
use std::ops::{BitOrAssign, SubAssign};

#[derive(Clone, Default)]
pub struct InputValueTicker {
    t0: InputValue,
    t1: InputValue,
}
impl InputValueTicker {
    pub fn new(init: InputValue) -> Self {
        InputValueTicker {
            t0: init,
            t1: InputValue::empty(),
        }
    }
    pub fn update(&mut self) {
        self.t0 = self.t1;
    }
    fn activate(&mut self, o: InputValue) {
        self.t1 |= o;
    }
    fn deactivate(&mut self, o: InputValue) {
        self.t1 &= !o;
    }
    pub fn is_on(&self, o: InputValue) -> bool {
        self.t0.contains(o)
    }
    pub fn only(&self, o: InputValue) -> bool {
        self.t0 == o
    }
    pub fn is_empty(&self) -> bool {
        self.t0.is_empty()
    }
    pub fn rising(&self, o: InputValue) -> bool {
        !self.t0.contains(o) && self.t1.contains(o)
    }
    pub fn falling(&self, o: InputValue) -> bool {
        self.t0.contains(o) && !self.t1.contains(o)
    }
}
impl BitOrAssign<InputValue> for InputValueTicker {
    fn bitor_assign(&mut self, o: InputValue) {
        self.activate(o);
    }
}
impl SubAssign<InputValue> for InputValueTicker {
    fn sub_assign(&mut self, o: InputValue) {
        self.deactivate(o);
    }
}
impl fmt::Display for InputValueTicker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:021b}", self.t0)
    }
}
impl fmt::Debug for InputValueTicker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}|{:032b}", self.t0, self.t1)
    }
}

#[derive(Clone, Default)]
pub struct VolatileValueTicker {
    t0: VolatileValue,
    t1: VolatileValue,
}
impl VolatileValueTicker {
    pub fn new(init: VolatileValue) -> Self {
        VolatileValueTicker {
            t0: init,
            t1: VolatileValue::empty(),
        }
    }
    pub fn update(&mut self) {
        self.t0 = self.t1;
    }
    fn activate(&mut self, o: VolatileValue) {
        self.t1 |= o;
    }
    fn deactivate(&mut self, o: VolatileValue) {
        self.t1 &= !o;
    }
    pub fn is_on(&self, o: VolatileValue) -> bool {
        self.t0.contains(o)
    }
    pub fn only(&self, o: VolatileValue) -> bool {
        self.t0 == o
    }
    pub fn is_empty(&self) -> bool {
        self.t0.is_empty()
    }
    pub fn rising(&self, o: VolatileValue) -> bool {
        !self.t0.contains(o) && self.t1.contains(o)
    }
    pub fn falling(&self, o: VolatileValue) -> bool {
        self.t0.contains(o) && !self.t1.contains(o)
    }
}
impl BitOrAssign<VolatileValue> for VolatileValueTicker {
    fn bitor_assign(&mut self, o: VolatileValue) {
        self.activate(o);
    }
}
impl SubAssign<VolatileValue> for VolatileValueTicker {
    fn sub_assign(&mut self, o: VolatileValue) {
        self.deactivate(o);
    }
}
impl fmt::Display for VolatileValueTicker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:021b}", self.t0)
    }
}
impl fmt::Debug for VolatileValueTicker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}|{:032b}", self.t0, self.t1)
    }
}
