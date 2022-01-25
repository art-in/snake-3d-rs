use crate::models::CubeSideType;

const FRONT: f32 = CubeSideType::Front as i32 as f32;
const BACK: f32 = CubeSideType::Back as i32 as f32;
const UP: f32 = CubeSideType::Up as i32 as f32;
const DOWN: f32 = CubeSideType::Down as i32 as f32;
const LEFT: f32 = CubeSideType::Left as i32 as f32;
const RIGHT: f32 = CubeSideType::Right as i32 as f32;

#[rustfmt::skip]
pub const CUBE_VERTEX_COORDS: [f32; 144] = [
    FRONT, -0.5, -0.5,   0.5,
    FRONT,  0.5, -0.5,   0.5,
    FRONT, -0.5,  0.5,   0.5,
    FRONT, -0.5,  0.5,   0.5,
    FRONT,  0.5, -0.5,   0.5,
    FRONT,  0.5,  0.5,   0.5,

    BACK, -0.5,  -0.5,  -0.5,
    BACK, -0.5,   0.5,  -0.5,
    BACK,  0.5,  -0.5,  -0.5,
    BACK, -0.5,   0.5,  -0.5,
    BACK,  0.5,   0.5,  -0.5,
    BACK,  0.5,  -0.5,  -0.5,

    UP,   -0.5,   0.5,  -0.5,
    UP,   -0.5,   0.5,   0.5,
    UP,    0.5,   0.5,  -0.5,
    UP,   -0.5,   0.5,   0.5,
    UP,    0.5,   0.5,   0.5,
    UP,    0.5,   0.5,  -0.5,

    DOWN, -0.5,  -0.5,  -0.5,
    DOWN,  0.5,  -0.5,  -0.5,
    DOWN, -0.5,  -0.5,   0.5,
    DOWN, -0.5,  -0.5,   0.5,
    DOWN,  0.5,  -0.5,  -0.5,
    DOWN,  0.5,  -0.5,   0.5,

    LEFT, -0.5,  -0.5,  -0.5,
    LEFT, -0.5,  -0.5,   0.5,
    LEFT, -0.5,   0.5,  -0.5,
    LEFT, -0.5,  -0.5,   0.5,
    LEFT, -0.5,   0.5,   0.5,
    LEFT, -0.5,   0.5,  -0.5,

    RIGHT, 0.5,  -0.5,  -0.5,
    RIGHT, 0.5,   0.5,  -0.5,
    RIGHT, 0.5,  -0.5,   0.5,
    RIGHT, 0.5,  -0.5,   0.5,
    RIGHT, 0.5,   0.5,  -0.5,
    RIGHT, 0.5,   0.5,   0.5,
];
