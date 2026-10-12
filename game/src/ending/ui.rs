use crate::{ending::EndingMessage, state::GameState};
use bevy::prelude::*;

#[derive(Component)]
pub struct RestartButton;

pub struct EndingUiPlugin;

impl Plugin for EndingUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, restart_button_system);
    }
}

// リスタートボタンが押されたときにゲーム状態をPlayingに変更するシステム
fn restart_button_system(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), With<RestartButton>>,
    mut state: ResMut<NextState<GameState>>,
    mut ending_state: ResMut<super::EndingState>,
) {
    for (interaction, mut color) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                color.0 = Color::srgb(0.35, 0.75, 0.35);
                println!("Restart button clicked!");
                state.set(GameState::Playing);
                ending_state.message = None;
            }
            Interaction::Hovered => {
                color.0 = Color::srgb(0.25, 0.25, 0.25);
            }
            Interaction::None => {
                color.0 = Color::srgb(0.15, 0.15, 0.15);
            }
        }
    }
}

// ゲーム終了時のUIを生成するシステム
pub fn spawn_ending_ui(
    mut commands: Commands,
    jp_font: Res<crate::fonts::JpFont>,
    ending_state: Res<super::EndingState>,
) {
    let ending = match ending_state.message {
        Some(message) => message,
        None => {
            println!("No ending message found, defaulting to GameOver");
            EndingMessage::GameOver
        }
    };
    let canvas = commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(ending.background_color()),
            DespawnOnExit(GameState::Ending),
        ))
        .id();
    let text = commands
        .spawn((
            Node {
                max_width: percent(100.0),
                ..default()
            },
            Text::new(ending.text()),
            TextFont {
                font: jp_font.0.clone(),
                font_size: 50.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::WHITE),
        ))
        .id();
    let restart_button = commands
        .spawn((
            Button,
            RestartButton,
            Node {
                width: px(200.0),
                height: px(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::default()),
            children![
                Node {
                    max_width: percent(100.0),
                    ..default()
                },
                Text::new("Restart"),
                TextFont {
                    font: jp_font.0.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Center),
                TextColor(Color::WHITE),
            ],
        ))
        .id();
    commands
        .entity(canvas)
        .add_children(&[text, restart_button]);
}
