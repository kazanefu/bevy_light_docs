use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub window: WindowSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub fullscreen: bool,
    pub vsync: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window: WindowSettings {
                width: 800,
                height: 600,
                title: "Game".to_string(),
                fullscreen: false,
                vsync: true,
            },
        }
    }
}

use std::fs;

use anyhow::Error;

fn load_settings() -> Result<Settings, Error> {
    let settings_str = fs::read_to_string("settings/settings.toml")?;
    let settings: Settings = toml::from_str(&settings_str)?;
    Ok(settings)
}

pub fn get_settings() -> Settings {
    match load_settings() {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!("Failed to load settings: {err}");
            Settings::default()
        }
    }
}
