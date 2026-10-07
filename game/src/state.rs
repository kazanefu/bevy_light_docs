use bevy::prelude::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Copy, States)]
pub enum GameState {
    #[default]
    Playing,
    Ending,
}
