use bevy::prelude::*;

// テクスチャのリソースを定義
#[derive(Resource)]
pub struct TextureHandles {
    pub player_texture: Handle<Image>,
    pub enemy_texture: Handle<Image>,
    pub goal_texture: Handle<Image>,
}

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        // 埋め込みアセットを登録
        bevy::asset::embedded_asset!(app, "player.png");
        bevy::asset::embedded_asset!(app, "enemy.png");
        bevy::asset::embedded_asset!(app, "goal.png");

        let asset_server = app.world().resource::<AssetServer>();
        app.insert_resource(TextureHandles {
            player_texture: asset_server.load("embedded://game/textures/player.png"),
            enemy_texture: asset_server.load("embedded://game/textures/enemy.png"),
            goal_texture: asset_server.load("embedded://game/textures/goal.png"),
        });
    }
}
