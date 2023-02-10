mod day3;
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

#[derive(Serialize, Deserialize, Debug)]

struct Ans {
    pt1: Option<usize>,
    pt2: Option<usize>,
}

#[wasm_bindgen]
pub fn day3(input_str: &str) -> JsValue {
    let a = match (day3::pt1(input_str), day3::pt2(input_str)) {
        (Ok(ans1), Ok(ans2)) => Ans {
            pt1: Some(ans1),
            pt2: Some(ans2),
        },
        _ => Ans {
            pt1: None,
            pt2: None,
        },
    };

    serde_wasm_bindgen::to_value(&a).unwrap()
}
