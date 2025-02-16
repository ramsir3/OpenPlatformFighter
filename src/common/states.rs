#![allow(dead_code)]

use bitflags::bitflags;

// use std::fmt;

bitflags! {
    #[derive(Copy, Clone, Default, PartialEq)]
    pub struct InputValue: u32 {
        const J    = 0b1_0000_0000_0000_0000;
        const L    = 0b0_1000_0000_0000_0000;
        const R    = 0b0_0100_0000_0000_0000;
        const U    = 0b0_0010_0000_0000_0000;
        const D    = 0b0_0001_0000_0000_0000;
        const B    = 0b0_0000_1000_0000_0000;
        const A    = 0b0_0000_0100_0000_0000;
        const S    = 0b0_0000_0010_0000_0000;
        const Z    = 0b0_0000_0001_0000_0000;
        const CR   = 0b0_0000_0000_1000_0000;
        const CL   = 0b0_0000_0000_0100_0000;
        const CD   = 0b0_0000_0000_0010_0000;
        const CU   = 0b0_0000_0000_0001_0000;
        const TR   = 0b0_0000_0000_0000_1000;
        const TL   = 0b0_0000_0000_0000_0100;
        const TD   = 0b0_0000_0000_0000_0010;
        const TU   = 0b0_0000_0000_0000_0001;
    }
    #[derive(Copy, Clone, Default, PartialEq)]
    pub struct VolatileValue: u32 {
        const GROUNDED        = 0b1_0000_0000_0000_0000;
        const ACTIVEINPUT     = 0b0_1000_0000_0000_0000;
        const INTERRUPTABLE   = 0b0_0100_0000_0000_0000;
        const HELPLESS        = 0b0_0010_0000_0000_0000;
        //                    = 0b0_0001_0000_0000_0000;
        //                    = 0b0_0000_1000_0000_0000;
        //                    = 0b0_0000_0100_0000_0000;
        //                    = 0b0_0000_0010_0000_0000;
        //                    = 0b0_0000_0001_0000_0000;
        //                    = 0b0_0000_0000_1000_0000;
        //                    = 0b0_0000_0000_0100_0000;
        //                    = 0b0_0000_0000_0010_0000;
        //                    = 0b0_0000_0000_0001_0000;
        //                    = 0b0_0000_0000_0000_1000;
        //                    = 0b0_0000_0000_0000_0100;
        //                    = 0b0_0000_0000_0000_0010;
        const FACINGLEFT      = 0b0_0000_0000_0000_0001;
    }
}
