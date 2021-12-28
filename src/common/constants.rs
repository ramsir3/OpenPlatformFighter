use graphics::types::{Color, Radius, Vec2d};

pub const WINDOW_SIZE: (f64, f64) = (640.0, 480.0);
pub const FRAMES_PER_SECOND: u64 = 30;

pub const ENTITY_COLLISION_BOX_COLOR: Color = [1.0, 0.1, 1.0, 0.4];
pub const HURT_BOX_COLOR: Color             = [1.0, 0.0, 0.0, 0.8];
pub const HIT_BOX_COLOR:  Color             = [0.0, 0.0, 1.0, 1.0];
pub const GRAB_BOX_COLOR: Color             = [1.0, 0.0, 1.0, 1.0];
pub const LEDGE_GRAB_BOX_COLOR: Color       = [1.0, 0.0, 1.0, 1.0];
pub const SHEILD_BOX_COLOR: Color           = [1.0, 0.0, 1.0, 1.0];

pub const PLATFORM_COLOR: Color = [0.4, 0.4, 0.4, 1.0];
pub const LIGHTPLATFORM_COLOR: Color = [0.0, 1.0, 1.0, 1.0];
pub const LIGHTPLATFORM_RADIUS: Radius = 1.0;

pub const N_SIDES: u32 = 12;

pub const RVEC : Vec2d = [  1.0,  0.0 ];
pub const LVEC : Vec2d = [ -1.0,  0.0 ];
pub const UVEC : Vec2d = [  0.0, -1.0 ];
pub const DVEC : Vec2d = [  0.0,  1.0 ];

type BoxId = u32;
type BoneId = u32;
type FighterId = u32;