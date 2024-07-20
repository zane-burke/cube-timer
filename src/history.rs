//! Handles retrieving and updating solve history

use gloo::storage::{LocalStorage, Storage};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use crate::utils;

const HISTORY_KEY: &str = "zane-burke.cube-timer.history.self";

#[derive(Serialize, Deserialize, Eq, Clone)]
pub struct Solve {
    pub timestamp: u64,
    pub solvetime: u64,
    pub shuffle: String,
}

impl Solve {
    pub fn new(timestamp: u64, solvetime: u64, shuffle: String) -> Self {
        Self {
            timestamp,
            solvetime,
            shuffle,
        }
    }
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
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
}

/// adds a single solve to local storage
pub fn save_solve(solve: Solve) {
    let mut history = retrieve_history();
    history.history.push(solve);
    set_history(history);
}

/// Gets the personal best
pub fn get_pb() -> u64 {
    let history = retrieve_history().history;

    match history.iter().min() {
        Some(s) => s.solvetime,
        None => u64::MIN,
    }
}

/// Gets the average of the last five, minus the best and worst solves
pub fn get_ao5() -> u64 {
    let history = retrieve_history().history;
    let last_five: Vec<u64> = history
        .iter()
        .rev()
        .take(5)
        .map(|sv| sv.solvetime)
        .collect();

    let length = last_five.len();

    // prevent further calculations of there isn't enough data to go off of
    if length <= 2 {
        return 0;
    }

    let (min, max) = last_five.iter().copied().minmax().into_option().unwrap();

    let sum: u64 = last_five.iter().copied().sum();

    utils::saturating_div(sum - max - min, length as u64)
}

/// Gets the user's all-time average solve time
pub fn get_avg() -> u64 {
    let history = retrieve_history().history;
    let length = history.len();
    let sum: u64 = history.iter().map(|sv| sv.solvetime).sum();

    utils::saturating_div(sum, length as u64)
}

/// Sets the user solve history to the provided value
pub fn set_history(history: History) {
    LocalStorage::set(HISTORY_KEY, history).expect("Failed to update");
}

/// Retrieves the user solve history.
/// Creates a new entry if there isn't one available.
pub fn retrieve_history() -> History {
    LocalStorage::get(HISTORY_KEY).unwrap_or_else(|_| History::new())
}
