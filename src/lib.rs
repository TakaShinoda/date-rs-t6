use chrono::prelude::*;
use wasm_bindgen::prelude::*;

// PC のタイムゾーンを使って現在の日付を yyyy/mm/dd 形式で返す
#[wasm_bindgen]
pub fn local_now() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y/%m/%d").to_string()
}
