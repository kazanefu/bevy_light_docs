# 家を配置、当たり判定を付けよう

プレイヤー同様に、ゴールの家も配置します。`src/gaol.rs`というファイルを作成し、以下のように書きます。

```rust
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// ゴールのコンポーネント
// タグとして使うだけなので、フィールドは持たない
#[derive(Component)]
pub struct Goal;

impl Goal {
    pub fn spawn(commands: &mut Commands, position: Vec3, texture: Handle<Image>) -> Entity {
        commands
            .spawn((
                Goal,
                Transform::from_translation(position),
                // 物理的に動かないように固定する
                RigidBody::Fixed,
                Collider::cuboid(25.0, 25.0),
                ActiveEvents::COLLISION_EVENTS,
                Sprite {
                    image: texture.clone(),
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
            ))
            .id()
    }
}
```

`src/texturers/gaol.png`という画像を用意して、ゴールの家の見た目として使います。また、プレイヤー同様に`TextureHandles`リソースにゴールのテクスチャを追加します。

`src/textures/mod.rs`を以下のように書き換えます。

```rust
use bevy::prelude::*;

// テクスチャのリソースを定義
#[derive(Resource)]
pub struct TextureHandles {
    pub player_texture: Handle<Image>,
    pub goal_texture: Handle<Image>,
}

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        // 埋め込みアセットを登録
        bevy::asset::embedded_asset!(app, "player.png");
        bevy::asset::embedded_asset!(app, "goal.png");
        app.add_systems(Startup, load_textures);
    }
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_texture_handle = asset_server.load("embedded://game/textures/player.png");
    let goal_texture_handle = asset_server.load("embedded://game/textures/goal.png");
    commands.insert_resource(TextureHandles {
        player_texture: player_texture_handle,
        goal_texture: goal_texture_handle,
    });
}
```

また、`src/playing.rs`の`setup`関数内でゴールの家をスポーンさせます。

```rust
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    textures: Option<Res<crate::textures::TextureHandles>>,
    cameras: Query<Entity, With<Camera2d>>,
) {
    let (player_texture, goal_texture) = match textures {
        Some(textures) => (
            textures.player_texture.clone(),
            textures.goal_texture.clone(),
        ),
        None => (
            asset_server.load("embedded://game/textures/player.png"),
            asset_server.load("embedded://game/textures/goal.png"),
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
}
