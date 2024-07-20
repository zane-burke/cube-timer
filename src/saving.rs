use std::cmp::Ordering;

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const KEY: &str = "zane-burke.cube-timer.self";

#[derive(Serialize, Deserialize, Eq, Clone)]
pub struct Solve {
    pub timestamp: String,
    pub solvetime: u64,
    pub shuffle: String,
}

impl Ord for Solve {
    fn cmp(&self, other: &Self) -> Ordering {
        self.solvetime.cmp(&other.solvetime)
    }
}

impl PartialOrd for Solve {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Solve {
    fn eq(&self, other: &Self) -> bool {
        self.solvetime == other.solvetime
    }
}

#[derive(Serialize, Deserialize)]
pub struct History {
    pub history: Vec<Solve>,
}

impl History {
    pub fn add(&mut self, solve: Solve) {
        self.history.push(solve);
    }

    pub fn get_pb(&self) -> u64 {
        match self.history.iter().min() {
            Some(s) => s.solvetime,
            None => u64::MIN,
        }
    }
}

pub fn get_solve_history() -> History {
    let history: Vec<Solve> = LocalStorage::get(KEY).unwrap_or_else(|_| Vec::new());
    
    /*let last_five = history
        .iter()
        .rev()
        .take(5)
        .map(|sv| sv.solvetime)
        .collect();*/

    
    History { history }
}

pub fn update_solve_history(history: History) {
    LocalStorage::set(KEY, history).expect("Failed to update");
}