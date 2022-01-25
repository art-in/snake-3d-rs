use crate::{
    helpers::{cube::get_cube_rotation_for_position, ranges::project_to_range},
    models::{CameraMode, Degrees, GameState, Range},
};

const AUTO_ROTATION_STEP_RANGE: Range = Range(0.5, 10.0);
const AUTO_ROTATION_ANGLE_RANGE: Range = Range(0.0, 180.0);

pub fn auto_rotate_loop(state: &mut GameState) {
    let cube = &mut state.scene.cube;

    let current_rotation = &mut cube.current_rotation;
    let target_rotation = &mut cube.target_rotation;

    if cube.camera_mode == CameraMode::Overview {
        target_rotation.y = Degrees(*target_rotation.y - 0.3).normalize();
    }

    if cube.camera_mode == CameraMode::FollowSnake {
        let head = state.snake.parts.get(0).unwrap();
        *target_rotation = get_cube_rotation_for_position(head, &cube.grid);
    }

    if current_rotation != target_rotation {
        cube.needs_redraw = true;
    }

    if current_rotation.x != target_rotation.x {
        current_rotation.x = make_rotation_step(current_rotation.x, target_rotation.x);
    }

    if current_rotation.y != target_rotation.y {
        current_rotation.y = make_rotation_step(current_rotation.y, target_rotation.y);
    }
}

fn make_rotation_step(current_angle: Degrees, target_angle: Degrees) -> Degrees {
    let angle_diff = ((*current_angle - *target_angle).abs())
        .min((*current_angle - *target_angle - 360.0).abs())
        .min((*current_angle - *target_angle + 360.0).abs());

    if angle_diff < AUTO_ROTATION_STEP_RANGE.0 {
        return target_angle;
    }

    let rotation_step = project_to_range(
        angle_diff,
        AUTO_ROTATION_ANGLE_RANGE,
        AUTO_ROTATION_STEP_RANGE,
    );

    let next_current_rotation = Degrees(
        *current_angle + rotation_step * get_rotation_direction(current_angle, target_angle) as f64,
    );

    next_current_rotation.normalize()
}

fn get_rotation_direction(from: Degrees, to: Degrees) -> i32 {
    if from == to {
        return 0;
    }

    if *from >= 0.0 && *to >= 0.0 {
        return if *from > *to { -1 } else { 1 };
    }

    if *from <= 0.0 && *to <= 0.0 {
        return if *from > *to { -1 } else { 1 };
    }

    if *from >= 0.0 && *to <= 0.0 {
        return if *from - *to <= 180.0 { -1 } else { 1 };
    }

    if *from <= 0.0 && *to >= 0.0 {
        return if -*from + *to < 180.0 { 1 } else { -1 };
    }

    panic!("unreachable");
}
