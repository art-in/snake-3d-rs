use wasm_bindgen::JsCast;

use crate::{
    helpers::canvas::{get_canvas_font_string, measure_canvas_text},
    models::{CubeSideType, GameState, GameStatus},
};

// cube sides are drawn in 2D context and passed as textures to 3D cube.
// this is not very performant approach, since we need to upload entire side
// image when something small changes on it. faster wound be to upload object
// positions only and draw them as separate 3D entities, textures untouched.
// I've dodged this approach because I guess it would be harder to code, while
// I want it to be as basic as possible without diving into 3D coding hell
// (for that I would chose some 3D library) ie. need to calculate 3D positions
// for all objects, apply different textures for different objects (snake,
// stones, apples, status overlays), etc.
pub fn init_cube_side_drawer(state: &mut GameState, side_type: CubeSideType) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document
        .create_element("canvas")
        .expect("failed to create canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    canvas.set_width(2u32.pow(9u32));
    canvas.set_height(2u32.pow(9u32));

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .expect("failed to get 2D render context")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let side = state
        .scene
        .cube
        .sides
        .get_mut(&side_type)
        .unwrap_or_else(|| panic!("failed to get side: {:?}", side_type));

    side.canvas = Some(canvas);
    side.ctx = Some(ctx);
    side.needs_redraw = true;
    side.needs_update_on_cube = true;
}

pub fn draw_cube_side_loop(state: &mut GameState, side_type: CubeSideType) {
    let cube = &mut state.scene.cube;
    let side = cube.sides.get_mut(&side_type).unwrap();

    if !side.needs_redraw {
        return;
    }

    let canvas = side.canvas.as_ref().unwrap();
    let ctx = side.ctx.as_ref().unwrap();

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;

    ctx.clear_rect(0.0, 0.0, width, height);

    ctx.set_global_alpha(1.0);
    ctx.set_fill_style(&"white".into());
    ctx.fill_rect(0.0, 0.0, width, height);

    // draw grid
    let grid = &cube.grid;

    let cell_width = width / grid.cols_count as f64;
    let cell_height = height / grid.rows_count as f64;

    for i in 1..grid.cols_count {
        let x = i as f64 * cell_width;
        ctx.move_to(x, 0.0);
        ctx.line_to(x, height);
    }

    for i in 1..grid.rows_count {
        let y = i as f64 * cell_height;
        ctx.move_to(0.0, y);
        ctx.line_to(width, y);
    }

    ctx.set_line_width(1.0);
    ctx.stroke();

    // draw snake
    ctx.set_fill_style(&"red".into());
    for part in &state.snake.parts {
        if part.side == side_type {
            ctx.fill_rect(
                part.col as f64 * cell_width,
                height - part.row as f64 * cell_height - cell_height,
                cell_width,
                cell_height,
            );
        }
    }

    // draw apples
    ctx.set_fill_style(&"green".into());
    for apple in &state.apples {
        if apple.side == side_type {
            ctx.fill_rect(
                apple.col as f64 * cell_width,
                height - apple.row as f64 * cell_height - cell_height,
                cell_width,
                cell_height,
            )
        }
    }

    // draw stones
    ctx.set_fill_style(&"black".into());
    for stone in &state.stones {
        if stone.side == side_type {
            ctx.fill_rect(
                stone.col as f64 * cell_width,
                height - stone.row as f64 * cell_height - cell_height,
                cell_width,
                cell_height,
            );
        }
    }

    // draw status overlay
    if state.status != GameStatus::InGame {
        const OVERLAY_HEIGHT: f64 = 200.0;
        const OVERLAY_WIDTH: f64 = 400.0;
        const OVERLAY_PADDING: f64 = 30.0;

        let overlay_horizontal_margin = (width - OVERLAY_WIDTH) / 2.0;
        let overlay_vertical_margin = (height - OVERLAY_HEIGHT) / 2.0;

        ctx.set_global_alpha(0.7);
        ctx.set_fill_style(&"white".into());
        ctx.fill_rect(
            overlay_horizontal_margin,
            overlay_vertical_margin,
            OVERLAY_WIDTH,
            OVERLAY_HEIGHT,
        );

        ctx.set_line_width(3.0);
        ctx.set_stroke_style(&"black".into());
        ctx.stroke_rect(
            overlay_horizontal_margin,
            overlay_vertical_margin,
            OVERLAY_WIDTH,
            OVERLAY_HEIGHT,
        );

        // title
        ctx.set_fill_style(&"black".into());
        let title_font =
            get_canvas_font_string(Some(70), Some("Consolas"), Some("px"), Some("bold"));
        ctx.set_font(&title_font);

        let title = match state.status {
            GameStatus::Paused => "PAUSED",
            GameStatus::Win => "WIN",
            GameStatus::Fail => "FAIL",
            _ => "SNAKE 3D",
        };

        let title_size = measure_canvas_text(ctx, title).unwrap();
        ctx.fill_text(
            title,
            width / 2.0 - title_size.width / 2.0,
            height / 2.0 + title_size.height / 2.0,
        )
        .unwrap();

        // controls hint
        let controls_hint_font = get_canvas_font_string(Some(20), Some("Consolas"), None, None);
        ctx.set_font(&controls_hint_font);
        let constrols_hint = "WSAD/arrows to control";
        let controls_hint_size = measure_canvas_text(ctx, constrols_hint).unwrap();

        ctx.fill_text(
            constrols_hint,
            width / 2.0 - controls_hint_size.width / 2.0,
            overlay_vertical_margin + OVERLAY_PADDING + controls_hint_size.height,
        )
        .unwrap();

        // start hint
        let start_hint_font = get_canvas_font_string(Some(20), Some("Consolas"), None, None);
        ctx.set_font(&start_hint_font);
        let start_hint = "space/enter to start";
        let start_hint_size = measure_canvas_text(ctx, start_hint).unwrap();

        ctx.fill_text(
            start_hint,
            width / 2.0 - start_hint_size.width / 2.0,
            height - overlay_vertical_margin - OVERLAY_PADDING,
        )
        .unwrap();
    }

    side.needs_redraw = false;
    side.needs_update_on_cube = true;
}
