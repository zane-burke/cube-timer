//! Handles retrieving user preferences
//! For now, that is limited to the shuffle length and whether to use dark mode or not.

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const PREFERENCES_KEY: &str = "zane-burke.cube-timer.preferences.self";

#[derive(Serialize, Deserialize, Clone)]
pub struct Preferences {
    pub shuffle_length: u64,
    pub dark_mode: bool,
}

impl Preferences {
    /// this should honestly probably be done with the `Default` trait
    /// since its purpose is the same, but I'm lazy so I won't.
    pub fn new() -> Self {
        Self {
            shuffle_length: 25,
            dark_mode: false,
        }
    }

    /// setter for the theme
    /// Also updates the preferences in local storage
    pub fn set_theme(&mut self, dark: bool) {
        self.dark_mode = dark;
        LocalStorage::set(PREFERENCES_KEY, self).expect("Failed to update theme preference");
    }

    /// setter for the shuffle length
    /// Also updates the preferences in local storage
    pub fn set_length(&mut self, length: u64) {
        self.shuffle_length = length;
        LocalStorage::set(PREFERENCES_KEY, self).expect("Failed to update length preference");
    }

    /// retrieves preferences from local storage
    pub fn retrieve_preferences() -> Self {
        LocalStorage::get(PREFERENCES_KEY).unwrap_or_else(|_| Self::new())
    }
}
