use std::collections::HashSet;

use crate::{
    helpers::cube::get_random_cube_position,
    models::{CameraMode, CubePosition, GameState, GameStatus, Snake},
};

use super::{cube_actions::auto_rotate_loop, snake_actions::move_snake_loop};

const APPLES_COUNT: usize = 10;
const STONES_COUNT: usize = 10;

pub fn init_game_state(state: &mut GameState) {
    state.status = GameStatus::Welcome;
    plant_objects(state);
}

pub fn update_game_state_loop(state: &mut GameState) {
    move_snake_loop(state);
    auto_rotate_loop(state);

    let cube = &mut state.scene.cube;

    if state.status == GameStatus::InGame {
        if state.snake.is_crashed {
            state.status = GameStatus::Fail;
            cube.camera_mode = CameraMode::Overview;
            cube.sides
                .values_mut()
                .for_each(|side| side.needs_redraw = true);
        }

        if state.apples.is_empty() {
            state.status = GameStatus::Win;
            cube.camera_mode = CameraMode::Overview;
            cube.sides
                .values_mut()
                .for_each(|side| side.needs_redraw = true);
        }
    }
}

fn plant_objects(state: &mut GameState) {
    let mut object_positions: HashSet<CubePosition> = HashSet::new();

    // plant snake
    state.snake = Snake::default();
    object_positions.insert(*state.snake.parts.get(0).unwrap());

    // plant apples
    state.apples.clear();

    while state.apples.len() < APPLES_COUNT {
        let pos = get_random_cube_position(&state.scene.cube);

        // do not plant above other objects
        if !object_positions.contains(&pos) {
            state.apples.insert(pos);
            object_positions.insert(pos);
        }
    }

    // plant stones
    state.stones.clear();

    while state.stones.len() < STONES_COUNT {
        let pos = get_random_cube_position(&state.scene.cube);

        if !object_positions.contains(&pos) {
            state.stones.insert(pos);
            object_positions.insert(pos);
        }
    }

    for side in state.scene.cube.sides.values_mut() {
        side.needs_redraw = true;
    }
}

pub fn start_or_pause_game(state: &mut GameState) {
    match state.status {
        GameStatus::Welcome | GameStatus::Paused => {
            state.status = GameStatus::InGame;
        }
        GameStatus::Win | GameStatus::Fail => {
            plant_objects(state);
            state.status = GameStatus::InGame;
        }
        GameStatus::InGame => {
            state.status = GameStatus::Paused;
        }
    }

    if state.status == GameStatus::InGame {
        state.scene.cube.camera_mode = CameraMode::FollowSnake;
    } else {
        state.scene.cube.camera_mode = CameraMode::Overview;
    }

    for side in state.scene.cube.sides.values_mut() {
        side.needs_redraw = true;
    }
}
