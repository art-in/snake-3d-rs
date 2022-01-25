use crate::{
    helpers::direction::get_opposite_direction,
    models::{CameraMode, CubeSideType, Degrees, Direction, GameState, GameStatus, Point2D},
};

use super::{game_actions::start_or_pause_game, snake_actions::set_snake_direction};

pub fn on_keydown(state: &mut GameState, key_code: &str) {
    let mut direction = None;

    match key_code {
        "ArrowUp" | "KeyW" => direction = Some(Direction::Up),
        "ArrowDown" | "KeyS" => direction = Some(Direction::Down),
        "ArrowLeft" | "KeyA" => direction = Some(Direction::Left),
        "ArrowRight" | "KeyD" => direction = Some(Direction::Right),
        "Space" | "Enter" => start_or_pause_game(state),
        _ => {}
    }

    if let Some(mut direction) = direction {
        let head = state.snake.parts.get(0).unwrap();
        let grid = &state.scene.cube.grid;

        // adjust direction per current camera rotation
        if (head.side == CubeSideType::Up && head.row >= grid.rows_count / 2)
            || (head.side == CubeSideType::Down && head.row < grid.rows_count / 2)
        {
            direction = get_opposite_direction(direction);
        }

        set_snake_direction(state, direction);
    }
}

pub fn on_mousedown(state: &mut GameState) {
    if state.status != GameStatus::InGame {
        let cube = &mut state.scene.cube;

        cube.mouse_is_dragging = true;
        cube.camera_mode = CameraMode::ManualControl;
    }
}

pub fn on_mouseup(state: &mut GameState) {
    let cube = &mut state.scene.cube;

    cube.mouse_is_dragging = false;
    cube.mouse_pos = None;
}

pub fn on_mousemove(state: &mut GameState, mouse_pos: Point2D) {
    let cube = &mut state.scene.cube;

    if !cube.mouse_is_dragging {
        return;
    }

    if let Some(cube_mouse_pos) = &cube.mouse_pos {
        let dx = cube_mouse_pos.x - mouse_pos.x;
        let dy = cube_mouse_pos.y - mouse_pos.y;

        const ROTATION_VELOCITY: f64 = 0.25;
        cube.target_rotation.x = Degrees(*cube.target_rotation.x - dy * ROTATION_VELOCITY);
        cube.target_rotation.y = Degrees(*cube.target_rotation.y - dx * ROTATION_VELOCITY);

        cube.target_rotation.x = cube.target_rotation.x.normalize();
        cube.target_rotation.y = cube.target_rotation.y.normalize();

        cube.current_rotation = cube.target_rotation;

        cube.needs_redraw = true;
    }

    cube.mouse_pos = Some(mouse_pos);
}
