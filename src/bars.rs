use crate::ui::Progressbar;
use bevy::prelude::*;

pub struct Bar {
    pub max: u16,
    pub value: u16,
    pub position: Vec2,
    pub visible: bool,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            max: 0,
            value: 0,
            position: Vec2::ZERO,
            visible: false,
        }
    }
}

pub fn healthbar_new(config: Bar) -> Progressbar {
    Progressbar {
        max: config.max as f32,
        value: config.value as f32,
        width: 100.0,
        height: 15.0,
        position: Vec3::new(config.position.x, config.position.y, 2.0),
        filled_color: Color::rgb(0.0, 0.7, 0.0),
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.3),
        visible: config.visible,
    }
}
