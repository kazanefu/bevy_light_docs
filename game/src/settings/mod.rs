use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// 設定を表す構造体
#[derive(Resource, Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub window: WindowSettings,
}

// ウィンドウ設定を表す構造体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub fullscreen: bool,
    pub vsync: bool,
}

// デフォルトの設定を提供するための実装
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

// 設定をロードする関数
fn load_settings() -> Result<Settings, Error> {
    let settings_str = fs::read_to_string("settings/settings.toml")?;
    let settings: Settings = toml::from_str(&settings_str)?;
    Ok(settings)
}

// 設定を取得する関数。ロードに失敗した場合はデフォルト設定を返す。
pub fn get_settings() -> Settings {
    match load_settings() {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!("Failed to load settings: {err}");
            Settings::default()
        }
    }
}
