# 家を配置、当たり判定を付けよう

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
