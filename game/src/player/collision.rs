use super::*;
use bevy::ecs::query::{QueryData, QueryFilter};

pub fn get_contained_entity<Q: QueryData, F: QueryFilter>(
    e1: Entity,
    e2: Entity,
    query: &Query<Q, F>,
) -> Option<Entity> {
    if query.get(e1).is_ok() {
        Some(e1)
    } else if query.get(e2).is_ok() {
        Some(e2)
    } else {
        None
    }
}

pub struct PlayerCollisionPlugin;

impl Plugin for PlayerCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_goal.in_set(crate::game_play_set::GamePlaySet::Detection),
        );
    }
}

fn check_goal(
    player: Query<Entity, With<Player>>,
    goal: Query<Entity, With<crate::goal::Goal>>,
    mut events: MessageReader<CollisionEvent>,
) {
    for event in events.read() {
        println!("a");
        if let CollisionEvent::Started(e1, e2, _) = event {
            let Some(player_entity) = get_contained_entity(*e1, *e2, &player) else {
                continue;
            };
            let Some(goal_entity) = get_contained_entity(*e1, *e2, &goal) else {
                continue;
            };
            println!(
                "Player {:?} collided with Goal {:?}",
                player_entity, goal_entity
            );
        }
    }
}
