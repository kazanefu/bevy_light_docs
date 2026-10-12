use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Wall;

impl Wall {
    pub fn spawn(commands: &mut Commands, position: Vec3) -> Entity {
        commands
            .spawn((
                Wall,
                Transform::from_translation(position),
                RigidBody::Fixed,
                Collider::cuboid(25.0, 200.0),
                Sprite {
                    color: Color::srgb(0.5, 0.25, 0.0),
                    custom_size: Some(Vec2::new(50.0, 400.0)),
                    ..default()
                },
            ))
            .id()
    }
}
