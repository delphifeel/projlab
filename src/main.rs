mod bars;
mod layers;

mod game_state;
use game_state::GameSession;

mod plugins;
use plugins::{DoorsPlugin, EnemiesPlugin, PlayerPlugin};

mod ui;
use ui::ProgressbarPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // prevents blurry sprites
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 800.0,
                        height: 600.0,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(ProgressbarPlugin)
        .add_plugin(DoorsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemiesPlugin)
        .add_startup_system(setup)
        .run();
}
fn setup(mut commands: Commands) {
    commands.insert_resource(GameSession::new());
    commands.spawn(Camera2dBundle::default());
}
