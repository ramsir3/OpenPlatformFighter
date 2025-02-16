use crate::common::animation::{Animation, AnimationState, N_ANIM_STATES};
use crate::common::frame::FrameData;
use graphics::math::{Matrix2d, Scalar};
use graphics::Graphics;
use std::ops::{AddAssign, Index, IndexMut};

#[derive(Default)]
pub struct AnimationArray<'a>([Animation<'a>; N_ANIM_STATES]);
impl<'a> AddAssign<Animation<'a>> for AnimationArray<'a> {
    fn add_assign(&mut self, rhs: Animation<'a>) {
        let i = rhs.state();
        self[i] = rhs;
    }
}
impl<'a> Index<AnimationState> for AnimationArray<'a> {
    type Output = Animation<'a>;
    fn index(&self, rhs: AnimationState) -> &Self::Output {
        &self.0[rhs as usize]
    }
}
impl<'a> IndexMut<AnimationState> for AnimationArray<'a> {
    fn index_mut(&mut self, rhs: AnimationState) -> &mut Self::Output {
        &mut self.0[rhs as usize]
    }
}

pub struct Fighter<'a> {
    pub aa: AnimationArray<'a>,
    pub astate: AnimationState,
    pub weight: Scalar,
    pub walkspeed: Scalar,
    pub init_fallspeed: Scalar,
    pub max_fallspeed: Scalar,
    pub jumpheight: Scalar,
    pub jumpspeed: Scalar,
}
impl<'a> Fighter<'a> {
    pub fn add_anim(&mut self, a: Animation<'a>) {
        self.aa += a;
    }
    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        self.aa[self.astate].draw(t, g);
    }
    pub fn update(&mut self, active: bool) {
        self.aa[self.astate].tick(active);
    }
    pub fn set_astate(&mut self, state: AnimationState, active: bool, rising: bool) -> bool {
        if state == self.astate {
            if rising {
                self.aa[self.astate].reset();
                self.astate = state;
                return true;
            }
            return false;
        } else if active && self.aa[self.astate].interruptable() || (self.aa[self.astate].done()) {
            // self.aa[self.astate].reset();
            self.astate = state;
            self.aa[self.astate].reset();
            return true;
        }
        false
    }
}
impl<'a> Default for Fighter<'a> {
    fn default() -> Self {
        let mut aa: AnimationArray = Default::default();
        aa += Animation::new(
            AnimationState::Idle,
            vec![FrameData::default()],
            vec![Default::default()],
        );
        aa += Animation::new(
            AnimationState::Walk,
            vec![FrameData::default()],
            vec![Default::default()],
        );
        // aa += Animation {state: AnimationState::Idle, grabboxes:Some(FrameData::default()), ..Default::default()};
        Fighter {
            aa,
            astate: AnimationState::Idle,
            weight: 10.0,
            walkspeed: 10.0,
            init_fallspeed: 5.0,
            max_fallspeed: 5.0,
            jumpheight: 25.0,
            jumpspeed: 25.0,
        }
    }
}
