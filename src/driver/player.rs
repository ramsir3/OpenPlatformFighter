use graphics::math::*;
use graphics::Graphics;
use piston::input::{Button, ButtonArgs, ButtonState};

use driver::controls::*;
use common::{states::*, stateticker::*, animation::AnimationState, constants::*, fighter::*, Drawable};

// use std::fmt;

pub struct Player<'a> {
    f: Fighter<'a>,
    c: Controls,
    istate: InputValueTicker,
    vstate: VolatileValueTicker,
    pos: Vec2d,
    vel: Vec2d,
    fvel: Vec2d,
    // acc: Vec2d,
    jt: (f64, bool),
}

impl<'a> Player<'a> {
    pub fn new(f: Fighter<'a>, c: Controls, pos: Vec2d) -> Self {
        Player {
            f,
            c,
            istate: InputValueTicker::new(InputValue::empty()),
            vstate: VolatileValueTicker::new(VolatileValue::GROUNDED),
            pos,
            vel:  [  0.0,   0.0],
            fvel: [  0.0,   0.0],
            // acc:  [  0.0,   0.0],
            jt:   (0.0, false),
        }
    }
    pub fn update(&mut self, dt: f64) {
        // println!("{:?}", self.is);

        self.f.update(!self.istate.is_empty());
        if self.istate.is_on(InputValue::S) {
            self.pos = [100.0, 100.0];
            self.vel = [  0.0,   0.0];
            self.fvel = [  0.0,   0.0];
        }
        // let last_astate = self.f.astate;
        if self.istate.is_on(InputValue::L | (InputValue::R | InputValue::D)) {
            self.f.set_astate(AnimationState::Walk, true, self.istate.rising(InputValue::L | (InputValue::R | InputValue::D)));
        }
        if self.istate.rising(InputValue::J) {
            // println!("rising: {}", self.is);
            if self.f.set_astate(AnimationState::Jump, true, self.istate.rising(InputValue::J)) {
                self.vel = mul([1.0, 0.0], self.vel);
                self.jt.1 = true;
            }
        }
        if self.istate.rising(InputValue::A) {
            // println!("rising: {}", self.is);
            self.f.set_astate(AnimationState::Jab, true, self.istate.rising(InputValue::A));
        }
        if self.istate.is_empty() || (self.istate.is_on(InputValue::J) && !self.istate.rising(InputValue::J)) {
            self.f.set_astate(AnimationState::Idle, false, false);
        }

        self.update_vel(dt);
        self.move_pos(dt);
        // if last_astate != self.f.astate { println!("{}, {}", last_astate, self.f.astate); }
        self.update_vstate();
        self.istate.update();
        self.vstate.update();
    }
    fn move_pos(&mut self, dt: f64) {
        self.pos = add(self.pos, mul_scalar(self.vel, dt));
    }
    fn update_vel(&mut self, dt: f64) {
        self.vel = [0.0, 0.0];
        let mut wvel = [0.0, 0.0];
        if self.istate.is_on(InputValue::L) {
            wvel = add(wvel, LVEC);
            self.vstate |= VolatileValue::FACINGLEFT;
        }
        if self.istate.is_on(InputValue::R) {
            wvel = add(wvel, RVEC);
            self.vstate -= VolatileValue::FACINGLEFT;
        }
        if self.istate.is_on(InputValue::D) {
            wvel = add(wvel, DVEC);
        }
        wvel = mul_scalar(wvel, self.f.walkspeed);
        if self.jt.1 {
            self.jt.0 += dt;
            wvel = add(wvel, mul_scalar(UVEC, self.f.jumpspeed*dt));
        }
        if self.jt.0 >= self.f.jumpheight/self.f.jumpspeed {
            self.jt.0 = 0.0;
            self.jt.1 = false;
            self.fvel = mul_scalar(DVEC, self.f.init_fallspeed);
        }

        if self.vstate.is_on(VolatileValue::GROUNDED) {
            self.fvel = [0.0, 0.0];
        } else {
            if self.vstate.falling(VolatileValue::GROUNDED) || self.istate.rising(InputValue::J) {
                // println!("fell, {:?}", self.is);
                self.fvel = mul_scalar(DVEC, self.f.init_fallspeed);
            }
            if self.fvel[1] < self.f.max_fallspeed {
                self.fvel = add(self.fvel, mul_scalar(DVEC, self.f.weight*dt));
            }
        }
        self.vel = add(add(self.vel, wvel), self.fvel);
        // println!("vel:  {:?}", self.vel);
        // println!("jt:   {:?}", self.jumptime);
        // println!("fvel: {:?}", self.fall_vel);
    }
    pub fn update_inputs(&mut self, b: &ButtonArgs) {
        if let Button::Keyboard(k) = b.button {
            if self.c.contains_key(&k) {
                let u = self.c[&k];
                // println!("{: >5}: {:021b}", format!("{:?}", k), u);
                match b.state {
                    ButtonState::Press => {
                        self.istate |= u;
                        // println!("added:   {:032b}, {:?}", u, k);
                    },
                    ButtonState::Release => {
                        self.istate -= u;
                        // println!("removed: {:032b}, {:?}", u, k);
                    }
                }
            }
        }
    }
    fn update_vstate(&mut self) {
        self.vstate -= VolatileValue::GROUNDED;
    }
    pub fn get_debug_state(&mut self) -> String {
        // format!("{:?}", self.istate)
        format!("{:?}", self.f.aa[self.f.astate])
    }
}
impl<'a> Drawable for Player<'a> {
    fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        self.f.draw(multiply(t, translate(self.pos)), g);
    }
}
