use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Goal;

pub fn spawn_goal(commands: &mut Commands, position: Vec3) -> Entity {
    commands
        .spawn((
            Goal,
            Transform::from_translation(position),
            RigidBody::Fixed,
            Collider::cuboid(25.0, 25.0),
            ActiveEvents::COLLISION_EVENTS,
            Sensor,
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
        ))
        .id()
}

impl Goal {
    pub fn spawn(commands: &mut Commands, position: Vec3) -> Entity {
        spawn_goal(commands, position)
    }
}
