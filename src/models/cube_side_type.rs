use num_derive::FromPrimitive;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, FromPrimitive)]
pub enum CubeSideType {
    Front = 0,
    Back = 1,
    Up = 2,
    Down = 3,
    Left = 4,
    Right = 5,
}
