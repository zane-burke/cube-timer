//! Handles retrieving and updating solve history

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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

/// Sets the user solve history to the provided value
pub fn set_history(history: History) {
    LocalStorage::set(HISTORY_KEY, history).expect("Failed to update");
}

/// Retrieves the user solve history.
/// Creates a new entry if there isn't one available.
pub fn retrieve_history() -> History {
    LocalStorage::get(HISTORY_KEY).unwrap_or_else(|_| History::new())
}
