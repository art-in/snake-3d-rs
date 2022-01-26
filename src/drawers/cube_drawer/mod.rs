pub mod geometry;

use wasm_bindgen::JsCast;

use crate::{
    helpers::{
        graphics_math::{self as gmath, Matrix4, Vec3},
        webgl::{get_attribute_location, get_uniform_location, init_program, init_shader},
    },
    models::{Cube, Degrees, GameState},
};

const FIELD_OF_VIEW: Degrees = Degrees(60.0);
const VERTEX_SHADER_SRC: &str = include_str!("shaders/vertex.glsl");
const FRAGMENT_SHADER_SRC: &str = include_str!("shaders/fragment.glsl");

pub fn init_cube_drawer(state: &mut GameState) {
    let program;
    let ctx;

    let scene = &mut state.scene;
    let cube = &mut scene.cube;
    let canvas = scene.canvas.as_ref().unwrap();

    ctx = canvas
        .get_context("webgl")
        .unwrap()
        .expect("failed to get webgl context")
        .dyn_into::<web_sys::WebGlRenderingContext>()
        .unwrap();

    // compile GLSL shaders for cube
    let vertex_shader = init_shader(
        &ctx,
        web_sys::WebGlRenderingContext::VERTEX_SHADER,
        VERTEX_SHADER_SRC,
    )
    .expect("failed to init vertex shader");
    let fragment_shader = init_shader(
        &ctx,
        web_sys::WebGlRenderingContext::FRAGMENT_SHADER,
        FRAGMENT_SHADER_SRC,
    )
    .expect("failed to init fragment shader");

    program = init_program(&ctx, [vertex_shader, fragment_shader]).expect("failed to init program");

    ctx.use_program(Some(&program));

    // lookup locations for attributes/uniforms
    let cube_vertex_coord_attr_location =
        get_attribute_location(&ctx, &program, "a_cube_vertex_coord").unwrap();
    let cube_vertex_side_attr_location =
        get_attribute_location(&ctx, &program, "a_cube_vertex_side").unwrap();
    let cube_texture_coord_attr_location =
        get_attribute_location(&ctx, &program, "a_cube_texture_coord").unwrap();
    cube.matrix_uniform_location = Some(get_uniform_location(&ctx, &program, "u_matrix").unwrap());

    // pass buffer with vertex coordinates
    let cube_vertex_coords_buffer = ctx.create_buffer();
    ctx.bind_buffer(
        web_sys::WebGlRenderingContext::ARRAY_BUFFER,
        cube_vertex_coords_buffer.as_ref(),
    );
    ctx.buffer_data_with_array_buffer_view(
        web_sys::WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(&geometry::CUBE_VERTEX_COORDS[..]),
        web_sys::WebGlRenderingContext::STATIC_DRAW,
    );

    // pass buffer with texture coordinates
    let cube_texture_coords_buffer = ctx.create_buffer();
    ctx.bind_buffer(
        web_sys::WebGlRenderingContext::ARRAY_BUFFER,
        cube_texture_coords_buffer.as_ref(),
    );
    ctx.buffer_data_with_array_buffer_view(
        web_sys::WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(&geometry::CUBE_TEXTURE_COORDS[..]),
        web_sys::WebGlRenderingContext::STATIC_DRAW,
    );

    // define how to extract coordinates from vertex buffer
    ctx.enable_vertex_attrib_array(cube_vertex_coord_attr_location);
    ctx.bind_buffer(
        web_sys::WebGlRenderingContext::ARRAY_BUFFER,
        cube_vertex_coords_buffer.as_ref(),
    );
    ctx.vertex_attrib_pointer_with_i32(
        cube_vertex_coord_attr_location,
        3,                                     // 3 components per iteration
        web_sys::WebGlRenderingContext::FLOAT, // the data is 32bit floats
        false,                                 // don't normalize the data
        16, // (bytes) each vertex consists of 4 x 4-byte floats (side, x, y, z)
        4,  // skip side float
    );

    // define how to extract cube side index from vertex buffer
    ctx.enable_vertex_attrib_array(cube_vertex_side_attr_location);
    ctx.bind_buffer(
        web_sys::WebGlRenderingContext::ARRAY_BUFFER,
        cube_vertex_coords_buffer.as_ref(),
    );
    ctx.vertex_attrib_pointer_with_i32(
        cube_vertex_side_attr_location,
        1,
        web_sys::WebGlRenderingContext::FLOAT,
        false,
        16,
        0,
    );

    // define how to extract coordinates from texture coordinates buffer
    ctx.enable_vertex_attrib_array(cube_texture_coord_attr_location);
    ctx.bind_buffer(
        web_sys::WebGlRenderingContext::ARRAY_BUFFER,
        cube_texture_coords_buffer.as_ref(),
    );
    ctx.vertex_attrib_pointer_with_i32(
        cube_texture_coord_attr_location,
        2,
        web_sys::WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );

    // create textures for cube sides
    let mut cube_textures: std::vec::Vec<web_sys::WebGlTexture> = Vec::new();
    cube_textures.reserve(cube.sides.len());
    for side_type in cube.sides.keys() {
        let texture = ctx.create_texture().unwrap();
        cube_textures.push(texture);

        // bind uniform with texture unit
        let side_type_idx = *side_type as i32;
        let uniform_name = format!("u_cube_texture_side_{side_type_idx}");
        let cube_texture_side_uniform_location =
            get_uniform_location(&ctx, &program, &uniform_name).unwrap();

        ctx.uniform1i(Some(&cube_texture_side_uniform_location), side_type_idx);
    }

    cube.textures = cube_textures;

    // pass texture data for the first time (update later in draw loop)
    for (side_type, side) in &mut cube.sides {
        let canvas = side.canvas.as_ref().unwrap();

        let side_type_idx = *side_type as u32;
        ctx.active_texture(web_sys::WebGlRenderingContext::TEXTURE0 + side_type_idx);
        ctx.bind_texture(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            cube.textures.get(side_type_idx as usize),
        );

        ctx.tex_image_2d_with_u32_and_u32_and_canvas(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            0,
            web_sys::WebGlRenderingContext::RGBA as i32,
            web_sys::WebGlRenderingContext::RGBA,
            web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
            canvas,
        )
        .unwrap();

        ctx.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_WRAP_S,
            web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        ctx.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_WRAP_T,
            web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        ctx.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MIN_FILTER,
            web_sys::WebGlRenderingContext::LINEAR as i32,
        );

        side.needs_update_on_cube = false;
    }

    state.scene.ctx = Some(ctx);
    state.scene.cube.program = Some(program);
}

fn should_redraw_cube(cube: &Cube) -> bool {
    cube.needs_redraw || cube.sides.iter().any(|(_, side)| side.needs_update_on_cube)
}

pub fn draw_cube_loop(state: &mut GameState) {
    let mut matrix;

    let scene = &state.scene;

    let canvas = scene.canvas.as_ref().unwrap();
    let ctx = scene.ctx.as_ref().unwrap();
    let cube = &scene.cube;

    if !should_redraw_cube(cube) {
        return;
    }

    // define how to convert from clip space to canvas pixels
    ctx.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    ctx.enable(web_sys::WebGlRenderingContext::CULL_FACE);
    ctx.enable(web_sys::WebGlRenderingContext::DEPTH_TEST);

    // clear the canvas and the depth buffer
    ctx.clear(
        web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT
            | web_sys::WebGlRenderingContext::DEPTH_BUFFER_BIT,
    );

    // compute the projection matrix
    let aspect: f32 = canvas.client_width() as f32 / canvas.client_height() as f32;
    let projection_matrix = gmath::perspective(FIELD_OF_VIEW.into(), aspect, 1.0, 2000.0);

    let camera_pos: Vec3 = [0.0, 0.0, 2.0];
    let up: Vec3 = [0.0, 1.0, 0.0];
    let target: Vec3 = [0.0, 0.0, 0.0];

    let camera_matrix = gmath::look_at(camera_pos, target, up);
    let view_matrix = gmath::inverse(camera_matrix);
    let view_projection_matrix = gmath::multiply(projection_matrix, view_matrix);

    matrix = gmath::x_rotate(view_projection_matrix, cube.current_rotation.x.into());
    matrix = gmath::y_rotate(matrix, cube.current_rotation.y.into());

    draw_cube(state, matrix);
}

fn draw_cube(state: &mut GameState, matrix: Matrix4) {
    let scene = &mut state.scene;

    let ctx = scene.ctx.as_mut().unwrap();
    let cube = &mut scene.cube;

    ctx.use_program(cube.program.as_ref());

    // update texture data if needed
    for (side_type, side) in &mut cube.sides {
        if side.needs_update_on_cube {
            let side_type_idx = *side_type as u32;
            ctx.active_texture(web_sys::WebGlRenderingContext::TEXTURE0 + side_type_idx);
            ctx.bind_texture(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                cube.textures.get(side_type_idx as usize),
            );
            ctx.tex_sub_image_2d_with_u32_and_u32_and_canvas(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                0,
                0,
                0,
                web_sys::WebGlRenderingContext::RGBA,
                web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                side.canvas.as_ref().unwrap(),
            )
            .unwrap();
            side.needs_update_on_cube = false;
        }
    }

    // pass transformation matrix
    ctx.uniform_matrix4fv_with_f32_array(cube.matrix_uniform_location.as_ref(), false, &matrix);

    // draw the geometry
    const CUBE_VERTICES_COUNT: i32 = 6  // cube sides
                                   * 2  // triangles per cube side
                                   * 3; // vertices per triangle

    ctx.draw_arrays(
        web_sys::WebGlRenderingContext::TRIANGLES,
        0,
        CUBE_VERTICES_COUNT,
    );

    cube.needs_redraw = false;
}
