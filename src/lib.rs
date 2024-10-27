use chrono::prelude::*;
use wasm_bindgen::prelude::*;

// Date in local time yyyy/mm/dd
#[wasm_bindgen]
pub fn today() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y/%m/%d").to_string()
}
