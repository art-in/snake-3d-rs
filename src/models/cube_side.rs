use super::CubeSideType;

pub struct CubeSide {
    pub canvas: Option<web_sys::HtmlCanvasElement>,
    pub ctx: Option<web_sys::CanvasRenderingContext2d>,

    pub side_type: CubeSideType,

    pub needs_redraw: bool,
    pub needs_update_on_cube: bool,
}

impl Default for CubeSide {
    fn default() -> Self {
        CubeSide {
            canvas: None,
            ctx: None,
            side_type: CubeSideType::Front,
            needs_redraw: true,
            needs_update_on_cube: true,
        }
    }
}

impl CubeSide {
    pub fn new(t: CubeSideType) -> Self {
        CubeSide {
            side_type: t,
            ..CubeSide::default()
        }
    }
}
