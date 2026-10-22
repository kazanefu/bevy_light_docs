# プレイヤーの移動と表示

まず、Updateの実行順序を制御するために、`src/game_play_set.rs`というファイルを作成し、以下のコードを追加します。

```rust
use bevy::prelude::*;

// 実行順を制御するためのシステムセットを定義
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GamePlaySet {
    Input,
    Detection,
    Movement,
    Rendering,
}

pub struct GamePlaySetPlugin;

impl Plugin for GamePlaySetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GamePlaySet::Input,
                GamePlaySet::Detection.after(GamePlaySet::Input),
                GamePlaySet::Movement.after(GamePlaySet::Detection),
                GamePlaySet::Rendering.after(GamePlaySet::Movement),
            )
                .chain(),
        );
    }
}
```

そしたら、`src/player/mod.rs`というファイルを作成し、PlayerについてのものをまとめるプラグインとPlayerコンポーネントとPlayerをスポーンするための関数を作成します。

```rust
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

// プレイヤーのコンポーネント
// 移動速度を保持する
#[derive(Component)]
pub struct Player {
    speed: f32,
}

// デフォルトトレイトを実装
impl Default for Player {
    fn default() -> Self {
        // デフォルトの移動速度を設定
        Self { speed: 200.0 }
    }
}

impl Player {
    // プレイヤーをスポーンしてエンティティを返す関数
    // エンティティはidとして返され、後で参照するために使用できる
    pub fn spawn(commands: &mut Commands, position: Vec3, texture: Handle<Image>) -> Entity {
        commands
            .spawn((
                // 速度を持つプレイヤーコンポーネントを追加
                Player::default(),
                // Transformコンポーネントを追加して位置を設定
                Transform::from_translation(position),
                // 長方形のコライダーを追加
                Collider::cuboid(25.0, 25.0),
                // 2D物理エンジンの剛体を追加
                RigidBody::Dynamic,
                // 重力の影響を受けないように設定
                GravityScale(0.0),
                // 回転をロックして、プレイヤーが回転しないように設定
                LockedAxes::ROTATION_LOCKED,
                // 衝突時のイベントを有効化
                ActiveEvents::COLLISION_EVENTS,
                // 速度を初期化
                Velocity::default(),
                // Continuous Collision Detectionを有効化
                Ccd::enabled(),
                // スプライトを追加して見た目を設定
                Sprite {
                    image: texture.clone(),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
            ))
            .id()
    }
}
```

次にプレイヤーの入力を処理するシステムを作成します。`src/player/input.rs`というファイルを作成し、以下のコードを追加します。

```rust
use bevy::prelude::*;

// 入力情報を2次元ベクトルで表すリソース
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct MoveDirection(pub Vec2);

// 入力に応じてMoveDirectionを更新するシステム
pub fn set_move_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key_config: Res<KeyConfig>,
    mut move_direction: ResMut<MoveDirection>,
) {
    let mut direction = Vec2::ZERO;
    if key_config.up.pressed(&keyboard_input) {
        direction.y += 1.0;
    }
    if key_config.down.pressed(&keyboard_input) {
        direction.y -= 1.0;
    }
    if key_config.left.pressed(&keyboard_input) {
        direction.x -= 1.0;
    }
    if key_config.right.pressed(&keyboard_input) {
        direction.x += 1.0;
    }
    move_direction.0 = direction.normalize_or_zero();
}

// キーの組み合わせを表す構造体
// 複数キーの組み合わせをサポートするため、最大4つのキーを保持できるようにする
#[derive(Clone, Copy, Debug)]
pub struct KeyBinding {
    keys: [Option<KeyCode>; KeyBinding::MAX_KEYS],
}

impl KeyBinding {
    // 最大キー数を定義する定数
    pub const MAX_KEYS: usize = 4;
    // 新しいKeyBindingを作成するコンストラクタ
    pub const fn new(keys: [Option<KeyCode>; KeyBinding::MAX_KEYS]) -> Self {
        Self { keys }
    }
    // 今のフレームで押されているかどうかを判定する
    pub fn pressed(&self, input: &ButtonInput<KeyCode>) -> bool {
        self.keys.iter().flatten().any(|&key| input.pressed(key))
    }
}

// プレイヤーの移動に使用するキー設定を保持するリソース
#[derive(Resource, Debug, Clone, Copy)]
pub struct KeyConfig {
    pub up: KeyBinding,
    pub down: KeyBinding,
    pub left: KeyBinding,
    pub right: KeyBinding,
}

// デフォルトのキーコンフィグ
impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            up: KeyBinding::new([Some(KeyCode::KeyW), Some(KeyCode::ArrowUp), None, None]),
            down: KeyBinding::new([Some(KeyCode::KeyS), Some(KeyCode::ArrowDown), None, None]),
            left: KeyBinding::new([Some(KeyCode::KeyA), Some(KeyCode::ArrowLeft), None, None]),
            right: KeyBinding::new([Some(KeyCode::KeyD), Some(KeyCode::ArrowRight), None, None]),
        }
    }
}
```

そしたら、この入力情報を使ってプレイヤーを動かすシステムを作成します。`src/player/control.rs`というファイルを作成し、以下のコードを追加します。

```rust
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
```

`src/player/mod.rs`のPlayerPluginにここまでのシステムやリソースを追加します。

```rust
mod control;
mod input;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            input::set_move_direction.in_set(crate::game_play_set::GamePlaySet::Input),
        )
        .init_resource::<input::MoveDirection>()
        .init_resource::<input::KeyConfig>()
        .add_plugins(control::PlayerControlPlugin);
    }
}
```

これでプレイヤーを動かす仕組みは完成しました。次はプレイヤーの見た目をつけれるようにします。`src/textures/mod.rs`というファイルを作成し、また、``src/textures/player.png``という画像を用意します。`src/textures/mod.rs`には以下のコードを追加します。

```rust
use bevy::prelude::*;

// テクスチャのリソースを定義
#[derive(Resource)]
pub struct TextureHandles {
    pub player_texture: Handle<Image>,
}

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        // 埋め込みアセットを登録
        bevy::asset::embedded_asset!(app, "player.png");
        app.add_systems(Startup, load_textures);
    }
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_texture_handle = asset_server.load("embedded://game/textures/player.png");

    commands.insert_resource(TextureHandles {
        player_texture: player_texture_handle,
    });
}
```

ではいよいよPlayerをゲーム上にスポーンさせます。Playerは`GameState`が`Playing`になったときにスポーンさせます。`src/playing.rs`というファイルを作成し、以下のコードを追加します。
```rust
use crate::{player::Player, state::GameState};
use bevy::prelude::*;


pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

// ゲームプレイ状態に入ったときに呼び出されるシステム
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    textures: Option<Res<crate::textures::TextureHandles>>,
    cameras: Query<Entity, With<Camera2d>>,
) {
    let player_texture = match textures {
        Some(textures) => (
            textures.player_texture.clone(),
        ),
        None => (
            asset_server.load("embedded://game/textures/player.png"),
        ),
    };
    // カメラが存在しない場合は新しくカメラをスポーンする
    if cameras.is_empty() {
        commands.spawn(Camera2d);
    }
    let player = Player::spawn(&mut commands, Vec3::new(-300.0, 0.0, 0.0), player_texture);
    commands
        .entity(player)
        .insert(DespawnOnExit(GameState::Playing));
}
```

あとは、`src/main.rs`に作ったプラグインとRapierPhysicsPluginを追加して、ゲームを実行してみましょう。(各モジュールを`mod`で宣言するのを忘れないようにしてください。また、bevy_rapier2d::prelude::*をuseするのも忘れないようにしてください。)

```rust
app.init_state::<state::GameState>()
        .add_plugins(textures::TexturePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(game_play_set::GamePlaySetPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(playing::PlayingPlugin)
```
