use num_traits::FromPrimitive;

use crate::models::{
    Cube, CubePosition, CubeSideType, Degrees, Direction, Grid, ModelRotation, Point3D, Radians,
};

use crate::drawers::cube_drawer::geometry;

use super::graphics_math::get_angle_between_vectors;

pub fn get_position_3d_for_cube_position(pos: &CubePosition, grid: &Grid) -> Point3D {
    let vert_ratio = (pos.row as f64 + 0.5) / grid.rows_count as f64;
    let horiz_ratio = (pos.col as f64 + 0.5) / grid.cols_count as f64;

    let ranges = &geometry::CUBE_SIDE_COORDS_RANGE[&pos.side];

    let dx = ranges.x.1 - ranges.x.0;
    let dy = ranges.y.1 - ranges.y.0;
    let dz = ranges.z.1 - ranges.z.0;

    match pos.side {
        CubeSideType::Front | CubeSideType::Back => Point3D {
            x: ranges.x.0 + dx * horiz_ratio,
            y: ranges.y.0 + dy * vert_ratio,
            z: ranges.z.0,
        },
        CubeSideType::Left | CubeSideType::Right => Point3D {
            x: ranges.x.0,
            y: ranges.y.0 + dy * vert_ratio,
            z: ranges.z.0 + dz * horiz_ratio,
        },
        CubeSideType::Up | CubeSideType::Down => Point3D {
            x: ranges.x.0 + dx * horiz_ratio,
            y: ranges.y.0,
            z: ranges.z.0 + dz * vert_ratio,
        },
    }
}

pub fn get_cube_rotation_for_position(pos: &CubePosition, grid: &Grid) -> ModelRotation {
    let pos_3d = get_position_3d_for_cube_position(pos, grid);

    // angle around X axis
    let x_vector = Point3D { y: 0.0, ..pos_3d };
    let x_angle = Radians(pos_3d.y.signum() * *get_angle_between_vectors(&pos_3d, &x_vector));

    // angle around Y axis
    let y_vector = Point3D { x: 0.0, ..pos_3d };
    let mut y_angle = Radians(-pos_3d.x.signum() * *get_angle_between_vectors(&pos_3d, &y_vector));
    if pos_3d.z < 0.0 {
        y_angle = Radians(std::f64::consts::PI - *y_angle);
    }

    ModelRotation {
        x: Degrees::from(x_angle).normalize().round(),
        y: Degrees::from(y_angle).normalize().round(),
    }
}

pub fn get_next_cube_position_and_direction(
    pos: &CubePosition,
    direction: Direction,
    grid: &Grid,
) -> (CubePosition, Direction) {
    let mut next_pos = *pos;
    let mut next_direction = direction;

    match direction {
        Direction::Up => next_pos.row += 1,
        Direction::Down => next_pos.row -= 1,
        Direction::Left => next_pos.col -= 1,
        Direction::Right => next_pos.col += 1,
    }

    // describe how cube sides adjust with each other when jumping from one side
    // to another

    // falling off the up edge
    if next_pos.row >= grid.rows_count {
        match next_pos.side {
            CubeSideType::Front => {
                next_pos.side = CubeSideType::Up;
                next_pos.row = 0;
            }
            CubeSideType::Back => {
                next_pos.side = CubeSideType::Up;
                next_direction = Direction::Down;
                next_pos.col = grid.cols_count - next_pos.col - 1;
                next_pos.row = grid.rows_count - 1;
            }
            CubeSideType::Up => {
                next_pos.side = CubeSideType::Back;
                next_direction = Direction::Down;
                next_pos.col = grid.cols_count - next_pos.col - 1;
                next_pos.row = grid.rows_count - 1;
            }
            CubeSideType::Down => {
                next_pos.side = CubeSideType::Front;
                next_pos.row = 0;
            }
            CubeSideType::Left => {
                next_pos.side = CubeSideType::Up;
                next_direction = Direction::Right;
                next_pos.row = grid.rows_count - next_pos.col - 1;
                next_pos.col = 0;
            }
            CubeSideType::Right => {
                next_pos.side = CubeSideType::Up;
                next_direction = Direction::Left;
                next_pos.row = next_pos.col;
                next_pos.col = grid.cols_count - 1;
            }
        }
    }

    // falling off the down edge
    if next_pos.row < 0 {
        match next_pos.side {
            CubeSideType::Front => {
                next_pos.side = CubeSideType::Down;
                next_pos.row = grid.cols_count - 1;
            }
            CubeSideType::Back => {
                next_pos.side = CubeSideType::Down;
                next_direction = Direction::Up;
                next_pos.col = grid.cols_count - next_pos.col - 1;
                next_pos.row = 0;
            }
            CubeSideType::Up => {
                next_pos.side = CubeSideType::Front;
                next_pos.row = grid.rows_count - 1;
            }
            CubeSideType::Down => {
                next_pos.side = CubeSideType::Back;
                next_direction = Direction::Up;
                next_pos.col = grid.cols_count - next_pos.col - 1;
                next_pos.row = 0;
            }
            CubeSideType::Left => {
                next_pos.side = CubeSideType::Down;
                next_direction = Direction::Right;
                next_pos.row = next_pos.col;
                next_pos.col = 0;
            }
            CubeSideType::Right => {
                next_pos.side = CubeSideType::Down;
                next_direction = Direction::Left;
                next_pos.row = grid.rows_count - next_pos.col - 1;
                next_pos.col = grid.cols_count - 1;
            }
        }
    }

    // falling off the right edge
    if next_pos.col >= grid.cols_count {
        match next_pos.side {
            CubeSideType::Front => {
                next_pos.side = CubeSideType::Right;
                next_pos.col = 0;
            }
            CubeSideType::Back => {
                next_pos.side = CubeSideType::Left;
                next_pos.col = 0;
            }
            CubeSideType::Up => {
                next_pos.side = CubeSideType::Right;
                next_direction = Direction::Down;
                next_pos.col = next_pos.row;
                next_pos.row = grid.rows_count - 1;
            }
            CubeSideType::Down => {
                next_pos.side = CubeSideType::Right;
                next_direction = Direction::Up;
                next_pos.col = grid.rows_count - next_pos.row - 1;
                next_pos.row = 0;
            }
            CubeSideType::Left => {
                next_pos.side = CubeSideType::Front;
                next_pos.col = 0;
            }
            CubeSideType::Right => {
                next_pos.side = CubeSideType::Back;
                next_pos.col = 0;
            }
        }
    }

    // falling off the left edge
    if next_pos.col < 0 {
        match next_pos.side {
            CubeSideType::Front => {
                next_pos.side = CubeSideType::Left;
                next_pos.col = grid.cols_count - 1;
            }
            CubeSideType::Back => {
                next_pos.side = CubeSideType::Right;
                next_pos.col = grid.cols_count - 1;
            }
            CubeSideType::Up => {
                next_pos.side = CubeSideType::Left;
                next_direction = Direction::Down;
                next_pos.col = grid.cols_count - next_pos.row - 1;
                next_pos.row = grid.rows_count - 1;
            }
            CubeSideType::Down => {
                next_pos.side = CubeSideType::Left;
                next_direction = Direction::Up;
                next_pos.col = next_pos.row;
                next_pos.row = 0;
            }
            CubeSideType::Left => {
                next_pos.side = CubeSideType::Back;
                next_pos.col = grid.cols_count - 1;
            }
            CubeSideType::Right => {
                next_pos.side = CubeSideType::Front;
                next_pos.col = grid.cols_count - 1;
            }
        }
    }

    (next_pos, next_direction)
}

pub fn get_random_cube_position(cube: &Cube) -> CubePosition {
    CubePosition {
        side: FromPrimitive::from_f64(js_sys::Math::random() * cube.sides.len() as f64).unwrap(),
        row: (js_sys::Math::random() * cube.grid.rows_count as f64).floor() as i32,
        col: (js_sys::Math::random() * cube.grid.cols_count as f64).floor() as i32,
    }
}
