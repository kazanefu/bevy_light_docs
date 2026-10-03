#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use bevy::{prelude::*, window::WindowResolution};
mod settings;

fn main() {
    // Load settings from settings.toml
    let settings = settings::get_settings();
    // Create a new Bevy app
    let mut app = App::new();
    // Add default plugins with custom window settings
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: settings.window.title.clone(),
            resolution: WindowResolution::new(settings.window.width, settings.window.height),
            mode: if settings.window.fullscreen {
                bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
            } else {
                bevy::window::WindowMode::Windowed
            },
            present_mode: if settings.window.vsync {
                bevy::window::PresentMode::AutoVsync
            } else {
                bevy::window::PresentMode::AutoNoVsync
            },
            ..default()
        }),
        ..default()
    }));
    // Insert the settings resource into the app
    app.insert_resource(settings);
    // Embed the font asset into the app
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf");
    // Run the app
    app.run();
}
