mod utils;
use serde::{Deserialize, Serialize};
use serde_json;
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
pub fn greet(name: &str) {
    let greeting_str = format!("Hello, {}!", name);
    alert(&greeting_str);
}

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    name: String,
}

#[wasm_bindgen]
pub fn greet_json(json: &str) {
    let res: Result<Greeting, _> = serde_json::from_str(json);

    let json_obj = if let Ok(obj) = res {
        obj
    } else {
        Greeting {
            name: "sample".to_string(),
        }
    };

    let name = json_obj.name;

    let greeting_str = format!("Hello from json, {}!", name);
    alert(&greeting_str);
}
