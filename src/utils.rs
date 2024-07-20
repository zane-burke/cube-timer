//! Several useful functions that don't require direct access to anything

use crate::timer::INSPECTION_TIME;

pub fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    date.get_time().to_string()
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
    let c = js_sys::Date::new_0().get_time() as u64;
    let s = start_time.unwrap_or(0) + INSPECTION_TIME;

    saturating_sub(s, c)
}

// increases time
pub fn inc_time(start_time: Option<u64>) -> u64 {
    let c = js_sys::Date::new_0().get_time() as u64;
    let s = start_time.unwrap_or(0);

    c - s
}

/// splits the shuffle sequence into vecs of length 5
pub fn chunk_vec(shuffle: &Vec<String>) -> Vec<Vec<String>> {
    shuffle.chunks(5).map(|s| s.into()).collect()
}