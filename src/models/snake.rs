use std::{collections::VecDeque, time::Duration};

use super::{CubePosition, CubeSideType, Direction};

pub struct Snake {
    pub parts: VecDeque<CubePosition>,
    pub direction: Direction,
    pub last_move_time: Option<f64>,
    pub move_period: Duration,
    pub is_crashed: bool,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            parts: VecDeque::from([CubePosition {
                side: CubeSideType::Front,
                row: 0,
                col: 0,
            }]),
            direction: Direction::Right,
            last_move_time: None,
            move_period: Duration::from_millis(150),
            is_crashed: false,
        }
    }
}
