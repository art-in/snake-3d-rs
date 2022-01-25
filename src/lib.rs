mod actions;
mod drawers;
mod game;
mod helpers;
mod models;

use game::GameRc;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    GameRc::start();

    Ok(())
}
