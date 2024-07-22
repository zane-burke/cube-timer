//! Several useful functions that don't require direct access to anything

pub const INSPECTION_TIME: u64 = 15_000;

const SECOND: u64 = 1_000;
const MINUTE: u64 = 60 * SECOND;
const HOUR: u64 = 60 * MINUTE;

pub fn get_current_time() -> u64 {
    js_sys::Date::new_0().get_time() as u64
}

pub fn saturating_sub(lhs: u64, rhs: u64) -> u64 {
    if rhs > lhs {
        0
    } else {
        lhs - rhs
    }
}

pub fn saturating_div(lhs: u64, rhs: u64) -> u64 {
    if rhs == 0 {
        0
    } else {
        lhs / rhs
    }
}

// decreases time
pub fn dec_time(start_time: Option<u64>) -> u64 {
    let c = get_current_time();
    let s = start_time.unwrap_or(0) + INSPECTION_TIME;

    saturating_sub(s, c)
}

// increases time
pub fn inc_time(start_time: Option<u64>) -> u64 {
    let c = get_current_time();
    let s = start_time.unwrap_or(0);

    c - s
}

/// splits the shuffle sequence into vecs of length 5
pub fn chunk_vec(shuffle: &Vec<String>) -> Vec<Vec<String>> {
    shuffle.chunks(5).map(|s| s.into()).collect()
}

/// formats a `u64` time into a pretty string
pub fn time_string(time: u64) -> String {
    let min = (time / MINUTE) % 60; // convert time to the right unit then mod 60 to prevent overflow
    let s = (time / SECOND) % 60;
    let ms = (time % SECOND) / 10;

    if time >= HOUR {
        let hr = time / HOUR;
        return format!("{:0>2}:{:0>2}:{:0>2}.{:0>2}", hr, min, s, ms).to_string();
    }

    format!("{:0>2}:{:0>2}.{:0>2}", min, s, ms).to_string()
}

/// handles `Option<u64>` - `Option<u64>`
pub fn saturating_unwrap_sub(lhs: Option<u64>, rhs: Option<u64>) -> u64 {
    saturating_sub(lhs.unwrap_or(0), rhs.unwrap_or(0))
}

/// Converts the provided timestamp to a time and date string
pub fn date_string(timestamp: u64) -> String {
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(timestamp as f64));
    let dd = date.get_date();
    let mm = date.get_month();
    let yyyy = date.get_full_year();
    
    format!("{dd}/{mm}/{yyyy}")
}