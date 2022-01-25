use super::Cube;

#[derive(Default)]
pub struct Scene {
    pub canvas: Option<web_sys::HtmlCanvasElement>,
    pub ctx: Option<web_sys::WebGlRenderingContext>,
    pub cube: Cube,
}
