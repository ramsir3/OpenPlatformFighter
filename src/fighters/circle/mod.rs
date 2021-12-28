use common::fighter::{Fighter, AnimationArray};
use common::animation::{AnimationState, Animation};
use common::frame::{Frame, FrameData, FrameType};

pub fn new<'a>() -> Fighter<'a> {
    let mut aa: AnimationArray = Default::default();
    aa += Animation::new(
        AnimationState::Idle,
        vec![FrameData(
            &[
                &[Frame::Hurt(&[[0.0, -6.0, 6.0],]), Frame::EntityCollison([-12.0, 6.0, 0.0, -6.0])],
                &[Frame::Hurt(&[[0.0, -6.1, 6.1],]), Frame::EntityCollison([-12.2, 6.1, 0.0, -6.1])],
                &[Frame::Hurt(&[[0.0, -6.3, 6.3],]), Frame::EntityCollison([-12.6, 6.3, 0.0, -6.3])],
                &[Frame::Hurt(&[[0.0, -6.5, 6.5],]), Frame::EntityCollison([-13.0, 6.5, 0.0, -6.5])],
                &[Frame::Hurt(&[[0.0, -7.0, 7.0],]), Frame::EntityCollison([-14.0, 7.0, 0.0, -7.0])],
                &[Frame::Hurt(&[[0.0, -7.3, 7.3],]), Frame::EntityCollison([-14.6, 7.3, 0.0, -7.3])],
                &[Frame::Hurt(&[[0.0, -7.7, 7.7],]), Frame::EntityCollison([-15.4, 7.7, 0.0, -7.7])],
                &[Frame::Hurt(&[[0.0, -8.0, 8.0],]), Frame::EntityCollison([-16.0, 8.0, 0.0, -8.0])],
                &[Frame::Hurt(&[[0.0, -8.0, 8.0],]), Frame::EntityCollison([-16.0, 8.0, 0.0, -8.0])],
                &[Frame::Hurt(&[[0.0, -7.0, 7.0],]), Frame::EntityCollison([-14.0, 7.0, 0.0, -7.0])],
                &[Frame::Hurt(&[[0.0, -6.7, 6.7],]), Frame::EntityCollison([-13.4, 6.7, 0.0, -6.7])],
                &[Frame::Hurt(&[[0.0, -6.5, 6.5],]), Frame::EntityCollison([-13.0, 6.5, 0.0, -6.5])],
                &[Frame::Hurt(&[[0.0, -6.0, 6.0],]), Frame::EntityCollison([-12.0, 6.0, 0.0, -6.0])],
                &[Frame::Hurt(&[[0.0, -6.0, 6.0],]), Frame::EntityCollison([-12.0, 6.0, 0.0, -6.0])],
            ]
        )],
        vec![FrameType::single(14,0)],
    );
    Fighter {
        aa,
        astate: AnimationState::Idle,
        weight:  10.0,
        walkspeed:  100.0,
        init_fallspeed: 20.0,
        max_fallspeed:  50.0,
        jumpheight: 1000.0,
        jumpspeed: 14000.0,
    }
}
