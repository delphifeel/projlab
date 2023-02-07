mod game_state;
mod plugins;

use bevy::prelude::*;
use game_state::GameSession;
use plugins::{Doors, Player};

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
        .add_plugin(Doors)
        .add_plugin(Player)
        .add_startup_system(setup)
        .run();
}
fn setup(mut commands: Commands) {
    commands.insert_resource(GameSession::new());
    commands.spawn(Camera2dBundle::default());
}
