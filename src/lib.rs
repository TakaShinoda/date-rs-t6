use chrono::prelude::*;
use wasm_bindgen::prelude::*;

// Date in local time yyyy/mm/dd
#[wasm_bindgen]
pub fn today() -> String {
    let now: DateTime<Local> = Local::now();
    let day_of_week = match now.weekday() {
        Weekday::Mon => "月",
        Weekday::Tue => "火",
        Weekday::Wed => "水",
        Weekday::Thu => "木",
        Weekday::Fri => "金",
        Weekday::Sat => "土",
        Weekday::Sun => "日",
    };

    format!("{}（{}）", now.format("%Y/%m/%d").to_string(), day_of_week)
}
