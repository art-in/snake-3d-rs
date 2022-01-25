use std::collections::HashMap;

use crate::models::{CubeSideType, Range};

pub struct CubeSideRangesByAxis {
    pub x: Range,
    pub y: Range,
    pub z: Range,
}

lazy_static::lazy_static! {
    pub static ref CUBE_SIDE_COORDS_RANGE: HashMap<CubeSideType, CubeSideRangesByAxis> =
        HashMap::from([
            (
                CubeSideType::Front,
                CubeSideRangesByAxis {
                    x: Range(-0.5, 0.5),
                    y: Range(-0.5, 0.5),
                    z: Range(0.5, 0.5),
                },
            ),
            (
                CubeSideType::Left,
                CubeSideRangesByAxis {
                    x: Range(-0.5, -0.5),
                    y: Range(-0.5, 0.5),
                    z: Range(-0.5, 0.5),
                },
            ),
            (
                CubeSideType::Right,
                CubeSideRangesByAxis {
                    x: Range(0.5, 0.5),
                    y: Range(-0.5, 0.5),
                    z: Range(0.5, -0.5),
                },
            ),
            (
                CubeSideType::Back,
                CubeSideRangesByAxis {
                    x: Range(0.5, -0.5),
                    y: Range(-0.5, 0.5),
                    z: Range(-0.5, -0.5),
                },
            ),
            (
                CubeSideType::Up,
                CubeSideRangesByAxis {
                    x: Range(-0.5, 0.5),
                    y: Range(0.5, 0.5),
                    z: Range(0.5, -0.5),
                },
            ),
            (
                CubeSideType::Down,
                CubeSideRangesByAxis {
                    x: Range(-0.5, 0.5),
                    y: Range(-0.5, -0.5),
                    z: Range(-0.5, 0.5),
                },
            ),
        ]);
}
