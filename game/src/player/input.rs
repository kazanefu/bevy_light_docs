use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct MoveDirection(pub Vec2);

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

#[derive(Clone, Copy, Debug)]
pub struct KeyBinding {
    keys: [Option<KeyCode>; KeyBinding::MAX_KEYS],
}

impl KeyBinding {
    pub const MAX_KEYS: usize = 4;
    pub fn new(keys: [Option<KeyCode>; KeyBinding::MAX_KEYS]) -> Self {
        Self { keys }
    }
    pub fn pressed(&self, input: &ButtonInput<KeyCode>) -> bool {
        self.keys.iter().flatten().any(|&key| input.pressed(key))
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct KeyConfig {
    pub up: KeyBinding,
    pub down: KeyBinding,
    pub left: KeyBinding,
    pub right: KeyBinding,
}

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
