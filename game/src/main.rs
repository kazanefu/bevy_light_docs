#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // リリースビルドではコンソールを非表示にする
// BevyとBevy Rapierのプリリュードをインポート
use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

// 各モジュールをインポート
mod ending;
mod enemy;
mod fonts;
mod game_play_set;
mod goal;
mod player;
mod playing;
mod settings;
mod state;
mod textures;
mod wall;

fn main() {
    // settings/settings.tomlから設定を読み込む
    let settings = settings::get_settings();
    // Bevyアプリケーションを作成
    let mut app = App::new();
    // デフォルトプラグインを追加し、ウィンドウ設定を上書きする
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
    // 設定をリソースとしてアプリに挿入
    app.insert_resource(settings);
    // 埋め込みフォントを追加
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf");

    app.init_state::<state::GameState>()
        .add_plugins(textures::TexturePlugin)
        .add_plugins(fonts::FontsPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(game_play_set::GamePlaySetPlugin)
        .add_plugins(player::PlayrePlugin)
        .add_plugins(playing::PlayingPlugin)
        .add_plugins(ending::EndingPlugin)
        .add_plugins(enemy::EnemyPlugin);

    // アプリの起動
    app.run();
}
