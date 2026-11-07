# プレイヤーの移動と表示

まず、Updateの実行順序を制御するために、`src/game_play_set.rs`というファイルを作成し、以下のコードを追加します。システムセットを使うことで、プレイヤーの入力、検出、移動、描画の順番を制御することができます。

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

そしたら、`src/player/mod.rs`というファイルを作成し、PlayerについてのものをまとめるプラグインとPlayerコンポーネントとPlayerをスポーンするための関数を作成します。`mod.rs`は`main.rs`同様に名前が固定のファイルで、そのディレクトリのモジュールを定義するファイルです。<br>
`Player`コンポーネントは、プレイヤーの移動スピードを保持するためのコンポーネントです。

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
PlayerPluginのbuildメソッド内にはあとで、プレイヤーの入力を処理するシステムや、プレイヤーの移動を制御するシステムを追加していきます。Pluginを使うことで、Playerに関する処理をまとめて管理することができます。<br>
`Player::spawn`関数は、プレイヤーをスポーンするための関数です。`Commands`を使ってエンティティを生成し、Playerに必要なコンポーネントを追加しています。

次にプレイヤーの入力を処理するシステムを作成します。`src/player/input.rs`というファイルを作成し、以下のコードを追加します。ここにはキー入力をVec2で表すリソースと、入力に応じてその値を更新するシステムを作成します。また、キーの組み合わせを表す構造体も作成します。<br>
`MoveDirection`というリソースを作成し、矢印キーやWASDキーの入力に応じて、x軸とy軸の方向をVec2で表します。リソースはBevyの仕組みで、システム間で共有されるデータを保持するためのものです。世界に1つだけ存在するグローバルなデータとして扱われます。<br>
`set_move_direction`システムは、入力に応じてMoveDirectionを更新するシステムです。`add_systems`で`Update`で登録しているので、毎フレーム呼ばれます。引数に`Res<ButtonInput<KeyCode>>`を受け取ることで、現在押されているキーの情報を取得できます。システムからリソースにアクセスするには、`Res`や`ResMut`を使います。`Res`は読み取り専用、`ResMut`は書き込み可能なリソースを取得するためのものです。ここでは`ButtonInput<KeyCode>`と`KeyConfig`を読み取り専用で取得し、`MoveDirection`を更新するために書き込み可能で取得しています。<br>
`KeyBinding`構造体は、複数のキーの組み合わせを表すための構造体です。最大4つのキーを保持できるようにしています。`pressed`メソッドは、今のフレームで押されているかどうかを判定するためのメソッドです。<br>
`KeyConfig`構造体は、プレイヤーの移動に使用するキー設定を保持するためのリソースです。デフォルトでは、WASDキーと矢印キーの組み合わせを設定しています。<br>

```rust
use crate::game_play_set::GamePlaySet;
use bevy::prelude::*;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MoveDirection::default())
            .insert_resource(KeyConfig::default())
            .add_systems(Update, set_move_direction.in_set(GamePlaySet::Input));
    }
}

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

そしたら、この入力情報を使ってプレイヤーを動かすシステムを作成します。`src/player/control.rs`というファイルを作成し、以下のコードを追加します。ここではRapierという物理エンジンを使ってプレイヤーの速度を直接設定することで、プレイヤーを動かします。<br>
`move_player`システムは、`MoveDirection`リソースを使ってプレイヤーの速度を設定するシステムです。`Query<(&mut Velocity, &Player)>`を使って、プレイヤーのVelocityコンポーネントとPlayerコンポーネントを取得し、速度を更新します。<br>

```rust
use super::*;
pub struct PlayerControlPlugin;

// プレイヤーの移動を制御するプラグイン
impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            // プレイヤーの移動を制御するシステムをFixedUpdateに追加
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
        app.add_plugins(input::PlayerInputPlugin)
            .add_plugins(control::PlayerControlPlugin);
    }
}
```

これでプレイヤーを動かす仕組みは完成しました。次はプレイヤーの見た目をつけれるようにします。`src/textures/mod.rs`というファイルを作成し、また、`src/textures/player.png`という画像を用意します。`src/textures/mod.rs`には以下のコードを追加します。`TextureHandles`というリソースを作成し、そこにプレイヤーのテクスチャを保持します。毎回``asset_server.load``でテクスチャを読み込むのは非効率なので、最初に読み込んでリソースとして保持しておきます。

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

ではいよいよPlayerをゲーム上にスポーンさせます。Playerは`GameState`が`Playing`になったときにスポーンさせます。`src/playing.rs`というファイルを作成し、以下のコードを追加します。`setup`システムは、`GameState`が`Playing`になったときに呼ばれ、ワールドにプレイヤーやカメラをスポーンします。
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
    // テクスチャがリソースとして存在する場合はそれを使い、存在しない場合はアセットサーバーからロードする
    let player_texture = match textures {
        Some(textures) => textures.player_texture.clone(),
        None => asset_server.load("embedded://game/textures/player.png"),
    };
    // カメラが存在しない場合は新しくカメラをスポーンする
    if cameras.is_empty() {
        commands.spawn(Camera2d);
    }
    let player = Player::spawn(&mut commands, Vec3::new(-300.0, 0.0, 0.0), player_texture);
    commands
        .entity(player)
        .insert(DespawnOnExit(GameState::Playing)); // ゲームプレイ状態を抜けるときにプレイヤーを削除するためのコンポーネントを追加
}
```

`Commands`はゲームの世界(World)に対する変更を予約しておくための仕組みです。

あとは、`src/main.rs`に作ったプラグインとRapierPhysicsPluginを追加して、ゲームを実行してみましょう。(各モジュールを`mod`で宣言するのを忘れないようにしてください。また、bevy_rapier2d::prelude::*をuseするのも忘れないようにしてください。)

```rust
app.init_state::<state::GameState>()
        .add_plugins(textures::TexturePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(game_play_set::GamePlaySetPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(playing::PlayingPlugin)
```
