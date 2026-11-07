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

        let font = app.world().resource::<AssetServer>().load(FONT_PATH);
        app.insert_resource(JpFont(font));
    }
}
