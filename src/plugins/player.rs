use crate::game_state::GameSession;
use bevy::prelude::*;

mod healthbar;
use healthbar::Healthbar;

pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(animate_sprite)
            .add_system(keyboard_input_system)
            .add_plugin(Healthbar);
    }
}

#[derive(Component)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            sprite.index += 1;
            if sprite.index == 3 {
                sprite.index = 0;
            }
        }
    }
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<GameSession>) {
    use crate::game_state::Direction;

    let mut direction = None;
    if keyboard_input.just_pressed(KeyCode::Up) {
        direction = Some(Direction::Top);
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        direction = Some(Direction::Right);
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        direction = Some(Direction::Bottom);
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        direction = Some(Direction::Left);
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        game_state.get_player_mut().health -= 10;
    }

    if let Some(d) = direction {
        if !game_state.move_player(d) {
            info!("Can't move player there");
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases_res: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("dungeon.png");

    // elf female
    let elf_female_textures = TextureAtlas::from_grid(
        texture_handle.clone(),
        Vec2::new(16.0, 28.0),
        4,
        1,
        None,
        Some(Vec2::new(192.0, 4.0)),
    );
    let elf_female_handle = atlases_res.add(elf_female_textures);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: elf_female_handle,
            transform: Transform {
                translation: Vec3 {
                    z: 1.0,
                    ..default()
                },
                scale: Vec3::new(3.0, 3.0, 3.0),
                ..default()
            },
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
