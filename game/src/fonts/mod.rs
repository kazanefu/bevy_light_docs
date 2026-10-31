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
    let jp_font_handle = asset_server.load(FONT_PATH);
    commands.insert_resource(JpFont(jp_font_handle));
}
