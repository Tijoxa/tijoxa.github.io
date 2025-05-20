mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Set the panic hook to provide better error messages in the browser console.
    utils::set_panic_hook();
    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: String) {
    alert(&format!("Hello, {name}!"));
}
