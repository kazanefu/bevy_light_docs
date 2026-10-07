use crate::{goal::Goal, player::Player, state::GameState, wall::Wall};
use bevy::prelude::*;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, DespawnOnExit(GameState::Playing)));
    let player = Player::spawn(&mut commands, Vec3::new(0.0, 0.0, 0.0));
    commands
        .entity(player)
        .insert(DespawnOnExit(GameState::Playing));
    let goal = Goal::spawn(&mut commands, Vec3::new(200.0, 0.0, 0.0));
    commands
        .entity(goal)
        .insert(DespawnOnExit(GameState::Playing));
}
