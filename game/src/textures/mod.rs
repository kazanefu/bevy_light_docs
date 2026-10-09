use bevy::prelude::*;

#[derive(Resource)]
pub struct TextureHandles {
    pub player_texture: Handle<Image>,
    pub enemy_texture: Handle<Image>,
    pub goal_texture: Handle<Image>,
}

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        bevy::asset::embedded_asset!(app, "player.png");
        bevy::asset::embedded_asset!(app, "enemy.png");
        bevy::asset::embedded_asset!(app, "goal.png");
        app.add_systems(Startup, load_textures);
    }
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_texture_handle = asset_server.load("embedded://game/textures/player.png");
    let enemy_texture_handle = asset_server.load("embedded://game/textures/enemy.png");
    let goal_texture_handle = asset_server.load("embedded://game/textures/goal.png");

    commands.insert_resource(TextureHandles {
        player_texture: player_texture_handle,
        enemy_texture: enemy_texture_handle,
        goal_texture: goal_texture_handle,
    });
}
