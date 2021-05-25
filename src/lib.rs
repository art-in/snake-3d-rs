use wasm_bindgen::prelude::*;

mod drawers;

// use `wee_alloc` as global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub fn init(vertex_shaderder_src: &str, fragment_shader_src: &str) {
    drawers::scene_drawer::draw_scene(vertex_shaderder_src, fragment_shader_src);
}
