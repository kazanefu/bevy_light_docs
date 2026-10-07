use super::*;
pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_player.in_set(crate::game_play_set::GamePlaySet::Movement),
        );
    }
}

pub fn move_player(
    time: Res<Time>,
    move_direction: Res<input::MoveDirection>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    let delta_time = time.delta_secs();

    for (mut transform, player) in &mut query {
        let movement = move_direction.0 * player.speed * delta_time;
        transform.translation += movement.extend(0.0);
    }
}
