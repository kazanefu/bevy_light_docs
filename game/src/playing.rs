use crate::{enemy::Enemy, goal::Goal, player::Player, state::GameState, wall::Wall};
use bevy::prelude::*;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(OnEnter(GameState::Playing), setup);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// ゲームプレイ状態に入ったときに呼び出されるシステム
fn setup(
    mut commands: Commands,
    textures: Res<crate::textures::TextureHandles>,
) {
    let player = Player::spawn(
        &mut commands,
        Vec3::new(-300.0, 0.0, 0.0),
        textures.player_texture.clone(),
    );
    commands
        .entity(player)
        .insert(DespawnOnExit(GameState::Playing));
    let goal = Goal::spawn(
        &mut commands,
        Vec3::new(200.0, 0.0, 0.0),
        textures.goal_texture.clone(),
    );
    commands
        .entity(goal)
        .insert(DespawnOnExit(GameState::Playing));
    let wall_1 = Wall::spawn(&mut commands, Vec3::new(-200.0, 100.0, 0.0));
    commands
        .entity(wall_1)
        .insert(DespawnOnExit(GameState::Playing));
    let wall_2 = Wall::spawn(&mut commands, Vec3::new(100.0, -100.0, 0.0));
    commands
        .entity(wall_2)
        .insert(DespawnOnExit(GameState::Playing));
    let enemy = Enemy::spawn(
        &mut commands,
        Vec3::new(0.0, 0.0, 0.0),
        textures.enemy_texture.clone(),
        -100.0..50.0,
    );
    commands
        .entity(enemy)
        .insert(DespawnOnExit(GameState::Playing));
}
