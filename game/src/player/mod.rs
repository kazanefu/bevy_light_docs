use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod collision;
mod control;
mod input;

pub struct PlayrePlugin;

impl Plugin for PlayrePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            input::set_move_direction.in_set(crate::game_play_set::GamePlaySet::Input),
        )
        .init_resource::<input::MoveDirection>()
        .init_resource::<input::KeyConfig>()
        .add_plugins(control::PlayerControlPlugin)
        .add_plugins(collision::PlayerCollisionPlugin);
    }
}

// プレイヤーのコンポーネント
// 移動速度を保持する
#[derive(Component)]
pub struct Player {
    speed: f32,
}

// デフォルトトレイトを実装
impl Default for Player {
    fn default() -> Self {
        // デフォルトの移動速度を設定
        Self { speed: 200.0 }
    }
}

impl Player {
    // プレイヤーをスポーンしてエンティティを返す関数
    // エンティティはidとして返され、後で参照するために使用できる
    pub fn spawn(commands: &mut Commands, position: Vec3, texture: Handle<Image>) -> Entity {
        commands
            .spawn((
                // 速度を持つプレイヤーコンポーネントを追加
                Player::default(),
                // Transformコンポーネントを追加して位置を設定
                Transform::from_translation(position),
                // 長方形のコライダーを追加
                Collider::cuboid(25.0, 25.0),
                // 2D物理エンジンの剛体を追加
                RigidBody::Dynamic,
                // 重力の影響を受けないように設定
                GravityScale(0.0),
                // 回転をロックして、プレイヤーが回転しないように設定
                LockedAxes::ROTATION_LOCKED,
                // 衝突時のイベントを有効化
                ActiveEvents::COLLISION_EVENTS,
                // 速度を初期化
                Velocity::default(),
                // Continuous Collision Detectionを有効化
                Ccd::enabled(),
                // スプライトを追加して見た目を設定
                Sprite {
                    image: texture.clone(),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
            ))
            .id()
    }
}
