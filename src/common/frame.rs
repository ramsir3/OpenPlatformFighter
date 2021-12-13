// use piston_window::{Graphics, math::Matrix2d, types::Color, ellipse, Ellipse};
use Drawable;
use graphics::{Graphics, math::*, types::*, ellipse, Ellipse, Polygon};
use common::constants::{BOX_COLORS, N_SIDES};
use std::fmt;

#[derive(Copy, Clone, Default)]
pub struct FrameState {
    cur_frame: usize,
    end_frame: usize,
    faf_frame: usize,
}

#[derive(Copy, Clone)]
pub enum FrameType {
    Single(FrameState),
    Repeat(FrameState),
}
impl fmt::Display for FrameType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FrameType::Single(_) => write!(f, "Single"),
            FrameType::Repeat(_) => write!(f, "Repeat"),
        }
    }
}
impl FrameType {
    pub fn single(end_frame: usize, faf_frame: usize) -> Self {
        FrameType::Single(FrameState {
            cur_frame: 0, end_frame, faf_frame,
        })
    }
    pub fn repeat(end_frame: usize, faf_frame: usize) -> Self {
        FrameType::Repeat(FrameState {
            cur_frame: 0, end_frame, faf_frame,
        })
    }
    pub fn cur_frame(&self) -> usize {
        match self {
            FrameType::Single(f) => f.cur_frame,
            FrameType::Repeat(f) => f.cur_frame,
        }
    }
    pub fn reset(&mut self) {
        match self {
            FrameType::Single(f) => f.cur_frame = 0,
            FrameType::Repeat(f) => f.cur_frame = 0,
        };
    }
    pub fn tick(&mut self) {
        match self {
            FrameType::Single(ref mut a) => { a.cur_frame = (a.cur_frame + 1) % a.end_frame; },
            FrameType::Repeat(ref mut a) => { a.cur_frame = (a.cur_frame + 1) % a.end_frame; },
        }
    }
    pub fn done(&self) -> bool {
        match self {
            FrameType::Single(a) => a.cur_frame >= a.end_frame - 1,
            FrameType::Repeat(a) => a.cur_frame >= a.end_frame - 1,
        }
    }
    pub fn interruptable(&self) -> bool {
        match self {
            FrameType::Single(a) => { a.cur_frame >= a.faf_frame },
            FrameType::Repeat(a) => { a.cur_frame >= a.faf_frame },
        }
    }
}
impl Default for FrameType {
    fn default() -> FrameType {
        FrameType::Single(Default::default())
    }
}

const NUM_BOX_TYPES: usize = 6;
pub enum Frame<'a> {
    NoBox,
    EntityCollison([f64; 4]),
    Hurt(&'a [[f64; 3]]),
    Hit(&'a [[f64; 3]]),
    Grab(&'a [[f64; 3]]),
    LedgeGrab(&'a [[f64; 3]]),
    Sheild(&'a [[f64; 3]]),
}
impl<'a> Frame<'a> {
    fn draw_ecb<G: Graphics>(&self, c: Color, points: &[f64; 4], t: Matrix2d, g: &mut G) {
        Polygon::new(c)
        .draw(
            &[
                [0.0, points[0]],
                [points[1], (points[0]-points[2])/2.0],
                [0.0, points[2]],
                [points[3], (points[0]-points[2])/2.0],
            ],
            &Default::default(), t, g
        );
    }
    fn draw_box<G: Graphics>(&self, c: Color, boxes: &[[f64; 3]], t: Matrix2d, g: &mut G) {
        for b in boxes {
            Ellipse::new(c).resolution(N_SIDES)
            .draw(
                ellipse::circle(b[0], b[1], b[2]),
                &Default::default(), t, g
            );
        }
    }
}
impl<'a> Drawable for Frame<'a> {
    fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        match self {
            Frame::EntityCollison(b) => self.draw_ecb(BOX_COLORS[5], b, t, g),
            Frame::Hurt(b)           => self.draw_box(BOX_COLORS[0], b, t, g),
            Frame::Hit(b)            => self.draw_box(BOX_COLORS[1], b, t, g),
            Frame::Grab(b)           => self.draw_box(BOX_COLORS[2], b, t, g),
            Frame::LedgeGrab(b)      => self.draw_box(BOX_COLORS[3], b, t, g),
            Frame::Sheild(b)         => self.draw_box(BOX_COLORS[4], b, t, g),
            Frame::NoBox             => {}
        }
    }
}

#[derive(Copy, Clone)]
pub struct FrameData<'a>(pub &'a [&'a [Frame<'a>]]);
impl<'a> FrameData<'a> {
    pub fn draw<G: Graphics>(&self, f: usize, t: Matrix2d, g: &mut G) {
        for bt in self.0[f] {
            bt.draw(t, g);
        }
    }
}
impl<'a> Default for FrameData<'a> {
    fn default() -> Self {
        FrameData(&[
            &[Frame::Hurt(&[[0.0, 0.0, 10.0],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 10.5],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 11.0],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 12.0],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 13.5],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 14.5],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 15.0],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 15.0],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 12.0],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 12.0],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 12.0],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 10.0],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 10.0],]),],
            &[Frame::Hurt(&[[0.0, 0.0, 10.0],]),],
        ])
    }
}