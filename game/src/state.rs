use bevy::prelude::*;

// ゲームの状態を定義
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Copy, States)]
pub enum GameState {
    #[default]
    Playing,
    Ending,
}
