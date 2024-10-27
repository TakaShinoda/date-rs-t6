use chrono::prelude::*;
use js_sys::Date;
use wasm_bindgen::prelude::*;

// Date in local time yyyy/mm/dd
#[wasm_bindgen]
pub fn local_now() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y/%m/%d").to_string()
}

// format local time yyyy/mm/dd hh:mm
#[wasm_bindgen]
pub fn format(js_date: &Date) -> String {
    let timestamp = js_date.get_time() as i64;
    let datetime_utc: DateTime<Utc> = match DateTime::<Utc>::from_timestamp(timestamp / 1000, 0) {
        Some(dt) => dt,
        None => {
            panic!("Error");
        }
    };

    let date: DateTime<Local> = datetime_utc.with_timezone(&Local);

    date.format("%Y/%m/%d %H:%M").to_string()
}
