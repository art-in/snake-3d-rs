use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn create_shader(
  context: &web_sys::WebGlRenderingContext,
  shader_type: u32,
  source: &str,
) -> Option<web_sys::WebGlShader> {
  let shader = context.create_shader(shader_type).unwrap();

  context.shader_source(&shader, source);

  context.compile_shader(&shader);

  if context.get_shader_parameter(&shader, web_sys::WebGlRenderingContext::COMPILE_STATUS)
    == JsValue::FALSE
  {
    context.delete_shader(Some(&shader));
    return None;
  }

  Some(shader)
}

fn create_program(
  context: &web_sys::WebGlRenderingContext,
  vertex_shader_src: &str,
  fragment_shader_src: &str,
) -> Option<web_sys::WebGlProgram> {
  let vertex_shader = create_shader(
    &context,
    web_sys::WebGlRenderingContext::VERTEX_SHADER,
    vertex_shader_src,
  )
  .unwrap();
  let fragment_shader = create_shader(
    &context,
    web_sys::WebGlRenderingContext::FRAGMENT_SHADER,
    fragment_shader_src,
  )
  .unwrap();

  let program = context.create_program().unwrap();
  context.attach_shader(&program, &vertex_shader);
  context.attach_shader(&program, &fragment_shader);
  context.link_program(&program);

  if context.get_program_parameter(&program, web_sys::WebGlRenderingContext::LINK_STATUS)
    == JsValue::FALSE
  {
    return None;
  }

  Some(program)
}

pub fn draw_scene(vertex_shaderder_src: &str, fragment_shader_src: &str) {
  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();
  let canvas = document.query_selector("canvas").unwrap().unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

  let context = canvas
    .get_context("webgl")
    .unwrap()
    .unwrap()
    .dyn_into::<web_sys::WebGlRenderingContext>()
    .unwrap();

  let program = create_program(&context, vertex_shaderder_src, fragment_shader_src).unwrap();

  let position_attr_location = context.get_attrib_location(&program, "a_position");

  let positions: [f32; 6] = [0.0, 0.0, 0.0, 0.5, 0.7, 0.0];
  let memory_buffer = wasm_bindgen::memory()
    .dyn_into::<js_sys::WebAssembly::Memory>()
    .unwrap()
    .buffer();
  let positions_location = positions.as_ptr() as u32 / 4;
  let positions_array = js_sys::Float32Array::new(&memory_buffer).subarray(
    positions_location,
    positions_location + positions.len() as u32,
  );

  let positions_buffer = context
    .create_buffer()
    .ok_or("failed to create buffer")
    .unwrap();
  context.bind_buffer(
    web_sys::WebGlRenderingContext::ARRAY_BUFFER,
    Some(&positions_buffer),
  );
  context.buffer_data_with_array_buffer_view(
    web_sys::WebGlRenderingContext::ARRAY_BUFFER,
    &positions_array,
    web_sys::WebGlRenderingContext::STATIC_DRAW,
  );

  // draw
  context.viewport(
    0,
    0,
    canvas.width().try_into().unwrap(),
    canvas.height().try_into().unwrap(),
  );

  context.clear_color(0.0, 0.0, 0.0, 0.0);
  context.clear(web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT);

  context.use_program(Some(&program));

  context.enable_vertex_attrib_array(position_attr_location.try_into().unwrap());

  context.bind_buffer(
    web_sys::WebGlRenderingContext::ARRAY_BUFFER,
    Some(&positions_buffer),
  );

  let size = 2; // 2 components per iteration
  let data_type = web_sys::WebGlRenderingContext::FLOAT; // the data is 32bit floats
  let normalize = false; // don't normalize the data
  let stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
  let offset = 0; // start at the beginning of the buffer
  context.vertex_attrib_pointer_with_i32(
    position_attr_location.try_into().unwrap(),
    size,
    data_type,
    normalize,
    stride,
    offset,
  );

  let primitive_type = web_sys::WebGlRenderingContext::TRIANGLES;
  let offset = 0;
  let count = 3;
  context.draw_arrays(primitive_type, offset, count);
}
