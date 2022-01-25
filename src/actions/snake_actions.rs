use std::time::Duration;

use crate::{
    helpers::{cube::get_next_cube_position_and_direction, direction::get_opposite_direction},
    models::{Direction, GameState, GameStatus},
};

const SNAKE_MOVE_PERIOD_MULTIPLIER: f64 = 0.05; // higher is faster
const MOVE_SNAKE: bool = true;

pub fn move_snake_loop(state: &mut GameState) {
    let snake = &state.snake;

    let now = web_sys::window().unwrap().performance().unwrap().now();

    if MOVE_SNAKE
        && state.status == GameStatus::InGame
        && (snake.last_move_time.is_none()
            || (now - snake.last_move_time.unwrap() >= snake.move_period.as_millis() as f64))
    {
        move_snake(state);
        state.snake.last_move_time = Some(now);
    }
}

fn move_snake(state: &mut GameState) {
    let scene = &mut state.scene;
    let snake = &mut state.snake;

    // instead of moving each snake part one step ahead, move tail to new head
    let head = snake.parts.get(0).unwrap().to_owned();
    let tail = snake.parts.get(snake.parts.len() - 1).unwrap();

    scene.cube.sides.get_mut(&tail.side).unwrap().needs_redraw = true;
    snake.parts.pop_back().unwrap();

    let (new_head, new_direction) =
        get_next_cube_position_and_direction(&head, snake.direction, &scene.cube.grid);

    snake.parts.push_front(new_head);
    snake.direction = new_direction;

    scene
        .cube
        .sides
        .get_mut(&new_head.side)
        .unwrap()
        .needs_redraw = true;

    check_for_apples(state);
    check_crash(state);
}

pub fn set_snake_direction(state: &mut GameState, direction: Direction) {
    if state.snake.direction == get_opposite_direction(direction) {
        return;
    }

    state.snake.direction = direction;
}

fn check_for_apples(state: &mut GameState) {
    let snake = &mut state.snake;
    let apples = &mut state.apples;

    let head = snake.parts.get(0).unwrap().to_owned();
    let tail = snake.parts.get(snake.parts.len() - 1).unwrap().to_owned();

    if apples.remove(&head) {
        snake.parts.push_back(tail);
        snake.move_period = Duration::from_millis(
            (snake.move_period.as_millis() as f64 * (1.0 - SNAKE_MOVE_PERIOD_MULTIPLIER)) as u64,
        );
    }
}

fn check_crash(state: &mut GameState) {
    let snake = &mut state.snake;
    let stones = &mut state.stones;

    let head = snake.parts.get(0).unwrap();

    // crash on stone
    if stones.contains(head) {
        snake.is_crashed = true;
    }

    // crash on tail
    for i in 3..snake.parts.len() {
        if head == snake.parts.get(i).unwrap() {
            snake.is_crashed = true;
            break;
        }
    }
}
