use bevy::prelude::*;

pub const FONT_PATH: &str = "embedded://game/fonts/NotoSansJP-Bold.ttf";

#[derive(Resource)]
pub struct JpFont(pub Handle<Font>);

pub struct FontsPlugin;

impl Plugin for FontsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fonts);
    }
}

fn setup_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let jp_font_handle = asset_server.load(FONT_PATH);
    commands.insert_resource(JpFont(jp_font_handle));
}
