# クリア画面を作る

まず、`src/ending/mod.rs`というファイルを作成し、ゲーム終了時に送信するメッセージの定義と、そのメッセージを処理するシステムを作ります。
```rust
use bevy::prelude::*;
use crate::state::GameState;
pub struct EndingPlugin;

// ゲーム終了メッセージを定義する列挙型
#[derive(Message, Copy, Clone, Debug)]
pub enum EndingMessage {
    Clear,
    GameOver,
}

// ゲーム終了状態を保持するリソース
#[derive(Resource, Debug, Clone, Default)]
pub struct EndingState {
    pub message: Option<EndingMessage>,
}

impl EndingMessage {
    // リザルト画面での背景色を返すメソッド
    pub fn background_color(&self) -> Color {
        match self {
            EndingMessage::Clear => Color::srgb(0.0, 1.0, 0.0),
            EndingMessage::GameOver => Color::srgb(1.0, 0.0, 0.0),
        }
    }
    // リザルト画面でのテキストを返すメソッド
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
            .add_systems(
                Update,
                handle_ending_message.in_set(crate::game_play_set::GamePlaySet::Detection),
            );
    }
}

// ゲーム終了メッセージを処理するシステム
fn handle_ending_message(
    mut ending_message: MessageReader<EndingMessage>,
    mut state: ResMut<NextState<GameState>>,
    mut ending_state: ResMut<EndingState>,
) {
    for message in ending_message.read() {
        println!("Ending message received: {:?}", message);
        // ゲーム終了状態を更新し、ゲーム状態をEndingに設定する
        ending_state.message = Some(*message);
        // GameStateをEndingに変更する
        state.set(GameState::Ending);
    }
}

```

これで、ゲーム終了時に送信されるメッセージを処理し、ゲーム状態を`Ending`に変更するシステムが完成しました。<br>

UIで使うためのフォントを読み込んでリソースに追加するシステムを作ります。`src/fonts/mod.rs`というファイルを作成し、以下のコードを追加します。また、フォントファイルを`src/fonts/NotoSansJP-Bold.ttf`に配置します。(ここではNoto Sans JPのBoldフォントを使用していますが、別のフォントでも構いません。)
```rust
use bevy::prelude::*;

// フォントファイルのパス
pub const FONT_PATH: &str = "embedded://game/fonts/NotoSansJP-Bold.ttf";

// 日本語フォントのリソースを定義
#[derive(Resource)]
pub struct JpFont(pub Handle<Font>);

pub struct FontsPlugin;

impl Plugin for FontsPlugin {
    fn build(&self, app: &mut App) {
        // 埋め込みフォントを追加
        bevy::asset::embedded_asset!(app, "NotoSansJP-Bold.ttf");
        app.add_systems(Startup, setup_fonts);
    }
}

// 日本語フォントをロードしてリソースとして登録するシステム
fn setup_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    // フォントをロード
    let jp_font_handle = asset_server.load(FONT_PATH);
    // フォントをリソースとして登録
    commands.insert_resource(JpFont(jp_font_handle));
}
```

UIを作成します。`src/ending/ui.rs`というファイルを作成し、ゲーム終了時のUIを表示するシステムを追加します。
```rust
use crate::{ending::EndingMessage, state::GameState};
use bevy::prelude::*;

// リスタートボタンのコンポーネント
#[derive(Component)]
pub struct RestartButton;

// ゲーム終了時のUIを表示するプラグイン
pub struct EndingUiPlugin;

impl Plugin for EndingUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            restart_button_system.in_set(crate::game_play_set::GamePlaySet::Input),
        )
        .add_systems(OnEnter(GameState::Ending), spawn_ending_ui);
    }
}

// リスタートボタンが押されたときにゲーム状態をPlayingに変更するシステム
fn restart_button_system(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<RestartButton>, Changed<Interaction>),
    >,
    mut state: ResMut<NextState<GameState>>,
    mut ending_state: ResMut<super::EndingState>,
) {
    for (interaction, mut color) in &mut button_query {
        // ボタンの状態に応じて色を変更する
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
    // canvasの子としてtextとrestart_buttonを追加
    commands
        .entity(canvas)
        .add_children(&[text, restart_button]);
}
```

作ったプラグインを追加していきます。`src/ending/mod.rs`に以下のコードを追加します。
```rust
mod ui;
```
```rust
impl Plugin for EndingPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EndingMessage>()
            .insert_resource(EndingState::default())
            .add_plugins(ui::EndingUiPlugin) // UIプラグインを追加
            .add_systems(
                Update,
                handle_ending_message.in_set(crate::game_play_set::GamePlaySet::Detection),
            );
    }
}
```

`src/main.rs`に作ったプラグインを追加します。`add_plugins(fonts::FontsPlugin)`と`add_plugins(ending::EndingPlugin)`を`app`に追加してください。<br>

`src/player/collision.rs`の`check_collisions`システムを書き換えて、プレイヤーがゴールに触れたときにゲームクリアのメッセージを送信するようにします。
```rust
// プレイヤーとゴール、敵との衝突を検出するシステム
fn check_collisions(
    players: Query<Entity, With<Player>>,
    goals: Query<Entity, With<Goal>>,
    mut collision_events: MessageReader<CollisionEvent>,
    mut ending_message: MessageWriter<EndingMessage>, // ゲーム終了メッセージを送信するためのMessageWriter
) {
    for event in collision_events.read() {
        // 衝突イベントが開始された場合のみ処理する
        let CollisionEvent::Started(first, second, _) = event else {
            continue;
        };

        // 衝突したエンティティのうち、プレイヤーに該当するエンティティを取得する
        let Some(player_entity) = find_entity(*first, *second, &players) else {
            continue;
        };

        // 衝突したエンティティのうち、ゴールまたは敵に該当するエンティティを取得する
        if let Some(goal_entity) = find_entity(*first, *second, &goals) {
            println!(
                "Player {:?} collided with Goal {:?}",
                player_entity, goal_entity
            );
            // ゲームクリアのメッセージを送信する
            ending_message.write(EndingMessage::Clear);
        }
    }
}
```
