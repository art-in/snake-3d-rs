use super::CubeSideType;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct CubePosition {
    pub side: CubeSideType,
    pub row: i32,
    pub col: i32,
}
