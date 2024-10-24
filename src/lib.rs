use chrono::prelude::*;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn main() {
//     // UTC で現在の日付、時刻を取得
//     let utc: DateTime<Utc> = Utc::now();
//     print!("{utc}");

//     // PC のタイムゾーンを使って取得
//     let local: DateTime<Local> = Local::now();
//     println!("{}", local);
//     println!("{}", local.format("%Y/%m/%d").to_string());
// }

// PC のタイムゾーンを使って現在の日付を yyyy/mm/dd 形式で返す
#[wasm_bindgen]
pub fn local_now() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y/%m/%d").to_string()
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
