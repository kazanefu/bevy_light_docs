use bevy::prelude::*;
mod ui;
use crate::state::GameState;
pub struct EndingPlugin;

#[derive(Message, Copy, Clone, Debug)]
pub enum EndingMessage {
    Clear,
    GameOver,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct EndingState {
    pub message: Option<EndingMessage>,
}

impl EndingMessage {
    pub fn background_color(&self) -> Color {
        match self {
            EndingMessage::Clear => Color::srgb(0.0, 1.0, 0.0),
            EndingMessage::GameOver => Color::srgb(1.0, 0.0, 0.0),
        }
    }
    pub fn text(&self) -> &'static str {
        match self {
            EndingMessage::Clear => "Game Clear !!!!!!!!!!!!!!!!!!!!!!!!",
            EndingMessage::GameOver => "Game Over !!!!!!!!!!!!!!!!!!!!!!!!",
        }
    }
}

impl Plugin for EndingPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EndingMessage>()
            .insert_resource(EndingState::default())
            .add_plugins(ui::EndingUiPlugin)
            .add_systems(
                Update,
                handle_ending_message.in_set(crate::game_play_set::GamePlaySet::Detection),
            )
            .add_systems(
                OnEnter(GameState::Ending),
                ui::spawn_ending_ui,
            );
    }
}

fn handle_ending_message(
    mut ending_message: MessageReader<EndingMessage>,
    mut state: ResMut<NextState<GameState>>,
    mut ending_state: ResMut<EndingState>,
) {
    for message in ending_message.read() {
        println!("Ending message received: {:?}", message);
        ending_state.message = Some(*message);
        state.set(GameState::Ending);
    }
}
