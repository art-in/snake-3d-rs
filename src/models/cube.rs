use std::collections::HashMap;

use super::{CameraMode, CubeSide, CubeSideType, Grid, ModelRotation, Point2D};

const GRID_SIZE: i32 = 16;

pub struct Cube {
    pub program: Option<web_sys::WebGlProgram>,
    pub matrix_uniform_location: Option<web_sys::WebGlUniformLocation>,
    pub textures: Vec<web_sys::WebGlTexture>,
    pub current_rotation: ModelRotation,
    pub target_rotation: ModelRotation,
    pub camera_mode: CameraMode,
    pub mouse_is_dragging: bool,
    pub mouse_pos: Option<Point2D>,
    pub needs_redraw: bool,
    pub grid: Grid,
    pub sides: HashMap<CubeSideType, CubeSide>,

    // side types can also be retreived from keys of sides map, but save them
    // separately so we can iterate over all sides without messing with borrow
    // checker, as it forces to clone iteration target beforehand if that target
    // also mutably borrowed inside each iteration, and cloning array is faster
    // than allocating vector for map keys with sides.keys.copied().collect()
    pub side_types: [CubeSideType; 6],
}

impl Default for Cube {
    fn default() -> Self {
        let side_types: [CubeSideType; 6] = [
            CubeSideType::Front,
            CubeSideType::Back,
            CubeSideType::Up,
            CubeSideType::Down,
            CubeSideType::Left,
            CubeSideType::Right,
        ];

        let mut sides: HashMap<CubeSideType, CubeSide> = HashMap::new();

        for side_type in side_types {
            sides.insert(side_type, CubeSide::new(side_type));
        }

        Cube {
            program: None,
            matrix_uniform_location: None,
            textures: Vec::new(),
            current_rotation: ModelRotation::default(),
            target_rotation: ModelRotation::default(),
            camera_mode: CameraMode::Overview,
            mouse_is_dragging: false,
            mouse_pos: None,
            needs_redraw: true,
            grid: Grid {
                rows_count: GRID_SIZE,
                cols_count: GRID_SIZE,
            },
            sides,
            side_types,
        }
    }
}
