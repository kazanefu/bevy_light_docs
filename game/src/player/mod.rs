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

#[derive(Component)]
pub struct Player {
    speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: 200.0 }
    }
}

impl Player {
    pub fn spawn(commands: &mut Commands, position: Vec3, texture: Handle<Image>) -> Entity {
        commands
            .spawn((
                Player::default(),
                Transform::from_translation(position),
                Collider::cuboid(25.0, 25.0),
                RigidBody::Dynamic,
                GravityScale(0.0),
                LockedAxes::ROTATION_LOCKED,
                ActiveEvents::COLLISION_EVENTS,
                Velocity::default(),
                Ccd::enabled(),
                Sprite {
                    image: texture.clone(),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
            ))
            .id()
    }
}
