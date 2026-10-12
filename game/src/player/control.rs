use super::*;
pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            move_player.in_set(crate::game_play_set::GamePlaySet::Movement),
        );
    }
}

// プレイヤーの移動を制御するシステム
pub fn move_player(
    move_direction: Res<input::MoveDirection>,
    mut query: Query<(&mut Velocity, &Player)>,
) {
    for (mut velocity, player) in &mut query {
        // プレイヤーの速度を設定
        velocity.linvel = move_direction.0 * player.speed;
    }
}
