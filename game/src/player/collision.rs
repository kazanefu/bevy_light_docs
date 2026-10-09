use super::*;
use crate::{ending::EndingMessage, enemy::Enemy, goal::Goal};
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

fn find_entity<Q: QueryData, F: QueryFilter>(
    first: Entity,
    second: Entity,
    query: &Query<Q, F>,
) -> Option<Entity> {
    [first, second]
        .into_iter()
        .find(|entity| query.get(*entity).is_ok())
}

fn check_collisions(
    players: Query<Entity, With<Player>>,
    goals: Query<Entity, With<Goal>>,
    enemies: Query<Entity, With<Enemy>>,
    mut collision_events: MessageReader<CollisionEvent>,
    mut ending_message: MessageWriter<EndingMessage>,
) {
    for event in collision_events.read() {
        let CollisionEvent::Started(first, second, _) = event else {
            continue;
        };

        let Some(player_entity) = find_entity(*first, *second, &players) else {
            continue;
        };

        if let Some(goal_entity) = find_entity(*first, *second, &goals) {
            println!(
                "Player {:?} collided with Goal {:?}",
                player_entity, goal_entity
            );
            ending_message.write(EndingMessage::Clear);
        } else if let Some(enemy_entity) = find_entity(*first, *second, &enemies) {
            println!(
                "Player {:?} collided with Enemy {:?}",
                player_entity, enemy_entity
            );
            ending_message.write(EndingMessage::GameOver);
        }
    }
}
