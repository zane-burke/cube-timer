use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use std::time::Duration;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Solve {
    pub timestamp: DateTime<Local>, 
    pub solve_time: Duration, 
    pub shuffle_used: String,
}

impl Solve {
    pub fn new(timestamp: DateTime<Local>, solve_time: Duration, shuffle_used: String) -> Self {
        Solve {
            timestamp,
            solve_time,
            shuffle_used,
        }
    }
}

pub fn read_save() -> Vec<Solve> {
    let savefile = fs::read_to_string("save.json").expect("Couldn't read the JSON save file");
    let json = serde_json::from_str(&savefile);
    match json {
        Ok(j) => j,
        Err(e) => {
            if e.is_eof() {
                Vec::new()
            } else {
                panic!("Something really weird happened!");
            }
        }
    }
}

pub fn save_solves(cache: &Vec<Solve>) {
    let json = serde_json::to_string(cache).expect("Couldn't serialize into JSON");
    fs::write("save.json", json).expect("Couldn't write to file.");
}

// This should probably be changed to return the median, rather than the mean. 
// That would unfortunately require a clone, so I am hesitant to do that.
pub fn all_time_average(cache: &[Solve]) -> Duration {
    let itered = cache.iter().map(|slv| slv.solve_time);
    itered.sum::<Duration>() / cache.len() as u32
}