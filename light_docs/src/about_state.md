# ゲーム全体の状態をStateで管理する

今回作るゲームでは大きく分けて、ゲームプレイ中, 終了(クリア or ゲームオーバー)の2つの状態が存在します。`src/state.rs`というファイルを作って以下のようにゲームの状態の定義をします。
```rust
use bevy::prelude::*;

// ゲームの状態を定義
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Copy, States)]
pub enum GameState {
    #[default]
    Playing,
    Ending,
}
```
また、アプリケーションにこの状態を追加するために、`src/main.rs`を以下のように書き換えます。
```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // リリースビルドではコンソールを非表示にする

use bevy::{prelude::*, window::WindowResolution};
mod settings;
mod state;


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

    app.init_state::<state::GameState>(); // ゲームの状態を追加
    
    // アプリの起動
    app.run();
}
```


