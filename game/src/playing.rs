use crate::{enemy::Enemy, goal::Goal, player::Player, state::GameState, wall::Wall};
use bevy::prelude::*;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    textures: Option<Res<crate::textures::TextureHandles>>,
    cameras: Query<Entity, With<Camera2d>>,
) {
    let (player_texture, goal_texture, enemy_texture) = match textures {
        Some(textures) => (
            textures.player_texture.clone(),
            textures.goal_texture.clone(),
            textures.enemy_texture.clone(),
        ),
        None => (
            asset_server.load("embedded://game/textures/player.png"),
            asset_server.load("embedded://game/textures/goal.png"),
            asset_server.load("embedded://game/textures/enemy.png"),
        ),
    };
    if cameras.is_empty() {
        commands.spawn(Camera2d);
    }
    let player = Player::spawn(&mut commands, Vec3::new(-300.0, 0.0, 0.0), player_texture);
    commands
        .entity(player)
        .insert(DespawnOnExit(GameState::Playing));
    let goal = Goal::spawn(&mut commands, Vec3::new(200.0, 0.0, 0.0), goal_texture);
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
        enemy_texture,
        -100.0..50.0,
    );
    commands
        .entity(enemy)
        .insert(DespawnOnExit(GameState::Playing));
}
