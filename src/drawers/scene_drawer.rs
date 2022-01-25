use crate::models::GameState;

use super::{
    cube_drawer::{draw_cube_loop, init_cube_drawer},
    cube_side_drawer::{draw_cube_side_loop, init_cube_side_drawer},
};

pub fn init_scene_drawer(state: &mut GameState, canvas: web_sys::HtmlCanvasElement) {
    state.scene.canvas = Some(canvas);

    for side_type in state.scene.cube.side_types {
        init_cube_side_drawer(state, side_type);
    }

    init_cube_drawer(state);
}

pub fn draw_scene_loop(state: &mut GameState) {
    for side_type in state.scene.cube.side_types {
        draw_cube_side_loop(state, side_type);
    }

    draw_cube_loop(state);
}
