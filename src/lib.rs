mod utils;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    pub name: String,
}

#[wasm_bindgen]
pub fn greet_json(json: &str) {
    let res: Result<Greeting, _> = serde_json::from_str(json);

    let name = match res {
        Ok(o) => o.name,
        _ => "sample".to_string(),
    };

    let greeting_str = format!("Hello from greet_json, {}!", name);
    alert(&greeting_str);
}

#[wasm_bindgen]
pub fn build_greeting(name: &str) -> JsValue {
    let g = Greeting {
        name: name.to_string(),
    };

    serde_wasm_bindgen::to_value(&g).unwrap()
}
