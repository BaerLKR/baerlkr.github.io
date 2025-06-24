#![recursion_limit = "1024"]

pub mod blog;
pub mod easter;
pub mod glow;
pub mod text;
pub mod theme;
pub mod time;

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f32;
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);
}

fn main() {
    set_panic_hook();
    text::text_animation();
    text::pacman();
    easter::rust();
    wasm_bindgen_futures::spawn_local(blog::blog());
    glow::cursor_glow();
    time::render();
}
