use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GamePlaySet {
    Input,
    Detection,
    Movement,
    Rendering,
}

pub struct GamePlaySetPlugin;

impl Plugin for GamePlaySetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GamePlaySet::Input,
                GamePlaySet::Detection.after(GamePlaySet::Input),
                GamePlaySet::Movement.after(GamePlaySet::Detection),
                GamePlaySet::Rendering.after(GamePlaySet::Movement),
            )
                .chain(),
        );
    }
}
