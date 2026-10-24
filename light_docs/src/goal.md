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
```

次にプレイヤーがゴールに触れたときの処理を追加します。`src/player/collision.rs`というファイルを作ってそこに以下のコードを書きます。
```rust
use super::*;
use crate::goal::Goal;
use bevy::ecs::query::{QueryData, QueryFilter};
use bevy_rapier2d::prelude::CollisionEvent;

pub struct PlayerCollisionPlugin;

impl Plugin for PlayerCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_collisions.in_set(crate::game_play_set::GamePlaySet::Detection),
        );
    }
}

// ２つのエンティティのうち、指定されたクエリに一致する最初のエンティティを返す関数
fn find_entity<Q: QueryData, F: QueryFilter>(
    first: Entity,
    second: Entity,
    query: &Query<Q, F>,
) -> Option<Entity> {
    [first, second]
        .into_iter()
        .find(|entity| query.get(*entity).is_ok())
}

// プレイヤーとゴール、敵との衝突を検出するシステム
fn check_collisions(
    players: Query<Entity, With<Player>>,
    goals: Query<Entity, With<Goal>>,
    mut collision_events: MessageReader<CollisionEvent>,
) {
    for event in collision_events.read() {
        // 衝突イベントが開始された場合のみ処理する
        let CollisionEvent::Started(first, second, _) = event else {
            continue;
        };

        // 衝突したエンティティのうち、プレイヤーに該当するエンティティを取得する
        let Some(player_entity) = find_entity(*first, *second, &players) else {
            continue;
        };

        // 衝突したエンティティのうち、ゴールまたは敵に該当するエンティティを取得する
        if let Some(goal_entity) = find_entity(*first, *second, &goals) {
            println!(
                "Player {:?} collided with Goal {:?}",
                player_entity, goal_entity
            );
        }
    }
}
```
`PlayerCollisionPlugin`を`PlayerPlugin`に追加します。<br>

これでプレイヤーが家に触れた瞬間にコンソールに`Player プレイヤーのid collided with Goal 家のid`のように表示されるようになったと思います。

