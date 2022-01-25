use std::collections::HashSet;

use super::{CubePosition, GameStatus, Scene, Snake};

pub struct GameState {
    pub scene: Scene,
    pub snake: Snake,
    pub apples: HashSet<CubePosition>,
    pub stones: HashSet<CubePosition>,
    pub status: GameStatus,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            scene: Scene::default(),
            snake: Snake::default(),
            apples: HashSet::new(),
            stones: HashSet::new(),
            status: GameStatus::Welcome,
        }
    }
}
