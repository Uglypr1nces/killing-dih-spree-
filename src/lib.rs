use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn say_something() {
    console::log_1(&"Hello from WASM!".into());
}
