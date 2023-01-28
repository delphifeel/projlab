use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 800.0,
                        height: 600.0,
                        ..default()
                    },
                    ..default()
                }),
        ) // prevents blurry sprites
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}

#[derive(Component /*, Deref, DerefMut*/)]
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

    // ogre
    let ogre_textures = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32.0, 32.0),
        4,
        1,
        None,
        Some(Vec2::new(144.0, 320.0)),
    );
    let ogre_handle = atlases_res.add(ogre_textures);

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: elf_female_handle,
            transform: Transform {
                translation: Vec3 {
                    z: 2.0,
                    ..default()
                },
                scale: Vec3::new(3.0, 3.0, 3.0),
                ..default()
            },
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: ogre_handle,
            transform: Transform {
                translation: Vec3 {
                    z: 1.0,
                    ..default()
                },
                scale: Vec3::new(6.0, 6.0, 6.0),
                ..default()
            },
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
