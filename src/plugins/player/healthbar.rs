use bevy::prelude::*;

use crate::game_state::player::PLAYER_MAX_HEALTH;
use crate::game_state::GameSession;

pub struct Healthbar;

impl Plugin for Healthbar {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(update);
    }
}

const WIDTH: f32 = 100.0;
const HEIGHT: f32 = 15.0;
const POS_X: f32 = -250.0;
const POS_Y: f32 = -270.0;

#[derive(Component)]
struct HealthbarFilled;

fn update(
    game_session: Res<GameSession>,
    mut query: Query<(&mut Visibility, &mut Transform, &mut Sprite), With<HealthbarFilled>>,
) {
    let Ok((mut visibility, mut transform, mut sprite)) = query.get_single_mut() else {
        panic!("Can't find healthbar");
    };

    let health = game_session.get_player().health;
    let percentage: f32 = health as f32 / PLAYER_MAX_HEALTH as f32;

    let new_size = Vec2::new(WIDTH * percentage as f32, HEIGHT);
    sprite.custom_size = Some(new_size);

    transform.translation = Vec3::new(POS_X - ((WIDTH - new_size.x) / 2.0), POS_Y, 2.0);

    if !visibility.is_visible {
        visibility.is_visible = true;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
            ..default()
        },
        transform: Transform::from_xyz(POS_X, POS_Y, 1.0),
        ..default()
    });
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.8, 0.1),
                custom_size: Some(Vec2::new(WIDTH / 2.0, HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(POS_X, POS_Y, 2.0),
            visibility: Visibility { is_visible: false },
            ..default()
        },
        HealthbarFilled,
    ));
}
