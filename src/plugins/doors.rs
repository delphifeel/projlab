use bevy::prelude::*;

use crate::game_state::{Direction, GameSession};

pub struct Doors;

const _POSITION: f32 = 200.0;

impl Plugin for Doors {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(show_doors);
    }
}

#[derive(Component)]
struct Door {
    direction: Direction,
}

fn show_doors(mut query: Query<(&Door, &mut Visibility)>, game_state: Res<GameSession>) {
    let player_room = game_state.get_player_room();
    for (door, mut visibility) in query.iter_mut() {
        match door.direction {
            Direction::Top => {
                visibility.is_visible = player_room.connections.top.is_some();
            }
            Direction::Right => {
                visibility.is_visible = player_room.connections.right.is_some();
            }
            Direction::Bottom => {
                visibility.is_visible = player_room.connections.bottom.is_some();
            }
            Direction::Left => {
                visibility.is_visible = player_room.connections.left.is_some();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let door_handle = asset_server.load("door.png");
    let make_door_bundle = |x, y, direction| {
        (
            SpriteBundle {
                texture: door_handle.clone(),
                transform: Transform::from_translation(Vec3 { x, y, z: 1.0 }),
                visibility: Visibility { is_visible: false },
                ..default()
            },
            Door { direction },
        )
    };
    commands.spawn(make_door_bundle(0.0, -_POSITION, Direction::Bottom));
    commands.spawn(make_door_bundle(_POSITION, 0.0, Direction::Right));
    commands.spawn(make_door_bundle(0.0, _POSITION, Direction::Top));
    commands.spawn(make_door_bundle(-_POSITION, 0.0, Direction::Left));
}
