use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::ops::Range;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub x_range: Range<f32>,
}

impl Enemy {
    const DEFAULT_SPEED: f32 = 100.0;
    pub fn from_range(x_range: Range<f32>) -> Self {
        Enemy {
            speed: Self::DEFAULT_SPEED,
            x_range,
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        position: Vec3,
        texture: Handle<Image>,
        range: Range<f32>,
    ) -> Entity {
        spawn_enemy(commands, position, texture, range)
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_enemy.in_set(crate::game_play_set::GamePlaySet::Movement),
        );
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    position: Vec3,
    texture: Handle<Image>,
    range: Range<f32>,
) -> Entity {
    commands
        .spawn((
            Enemy::from_range(range),
            Transform::from_translation(position),
            RigidBody::Dynamic,
            Collider::cuboid(25.0, 25.0),
            ActiveEvents::COLLISION_EVENTS,
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y,
            Velocity::default(),
            Sprite {
                image: texture.clone(),
                color: Color::WHITE,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
        ))
        .id()
}

fn move_enemy(mut enemy_que: Query<(&mut Velocity, &Enemy, &Transform)>) {
    for (mut velocity, enemy, transform) in &mut enemy_que {
        if velocity.linvel.x == 0.0 {
            velocity.linvel.x = enemy.speed;
        }
        let x = transform.translation.x;
        if x < enemy.x_range.start {
            velocity.linvel.x = enemy.speed;
        } else if x > enemy.x_range.end {
            velocity.linvel.x = -enemy.speed;
        }
    }
}
