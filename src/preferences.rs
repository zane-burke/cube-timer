//! Handles retrieving user preferences
//! For now, that is limited to the shuffle length and whether to use dark mode or not.

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const PREFERENCES_KEY: &str = "zane-burke.cube-timer.preferences.self";

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Preferences {
    pub shuffle_length: u64,
    pub dark_mode: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            shuffle_length: 25,
            dark_mode: false,
        }
    }
}

/// setter for the theme
/// Also updates the preferences in local storage
pub fn set_theme(dark: bool) {
    LocalStorage::set(PREFERENCES_KEY, dark).expect("Failed to update theme preference");
}

/// getter for the theme
pub fn get_theme() -> bool {
    let prefs = LocalStorage::get(PREFERENCES_KEY).unwrap_or_else(|_| Preferences::default());
    prefs.dark_mode
}

/// setter for the shuffle length
/// Also updates the preferences in local storage
pub fn set_length(length: u64) {
    LocalStorage::set(PREFERENCES_KEY, length).expect("Failed to update length preference");
}

/// getter for the shuffle length
pub fn get_length() -> u64 {
    let prefs = LocalStorage::get(PREFERENCES_KEY).unwrap_or_else(|_| Preferences::default());
    prefs.shuffle_length
}

/// retrieves preferences from local storage
pub fn get_preferences() -> Preferences {
    LocalStorage::get(PREFERENCES_KEY).unwrap_or_else(|_| Preferences::default())
}
