use crate::{
    bars::{healthbar_new, Bar},
    game_state::{Enemy, GameSession},
    ui::Progressbar,
};
use bevy::prelude::*;
use std::sync::Mutex;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(update);
    }
}

const SPAWN_POINTS: [Vec2; 3] = [
    Vec2::new(150.0, 100.0),
    Vec2::new(150.0, 0.0),
    Vec2::new(150.0, -100.0),
];

#[derive(Component)]
struct EnemyComponent {
    spawn_point: Vec2,
    sprite_entity: Entity,
    healthbar_entity: Entity,
}

static PREV_ROOM_ID: Mutex<Option<usize>> = Mutex::new(None);

fn room_changed(room_id: usize) -> bool {
    let id = PREV_ROOM_ID.lock().unwrap();
    match *id {
        None => true,
        Some(prev_id) if prev_id != room_id => true,
        Some(_) => false,
    }
}

fn change_visibility_according_to_enemy(
    q_visibility: &mut Query<&mut Visibility>,
    q_heathbar: &mut Query<&mut Progressbar>,
    enemy_component: &EnemyComponent,
    enemy: Option<&Enemy>,
) {
    let mut visibility = q_visibility.get_mut(enemy_component.sprite_entity).unwrap();
    let Ok(mut healthbar) = q_heathbar
        .get_mut(enemy_component.healthbar_entity) else {
            panic!("Can't find enemy healthbar");
        };

    match enemy {
        Some(enemy_v) => {
            healthbar.max = enemy_v.health as f32;
            healthbar.value = enemy_v.health as f32;
            visibility.is_visible = true;
            healthbar.visible = true;
        }
        None => {
            visibility.is_visible = false;
            healthbar.visible = false;
        }
    }
}

fn update(
    game_session: Res<GameSession>,
    query: Query<&EnemyComponent>,
    mut q_visibility: Query<&mut Visibility>,
    mut q_heathbar: Query<&mut Progressbar>,
) {
    let room = game_session.get_player_room();
    if !room_changed(room.id) {
        return;
    }

    {
        *PREV_ROOM_ID.lock().unwrap() = Some(room.id);
    }

    for enemy_component in query.iter() {
        change_visibility_according_to_enemy(
            &mut q_visibility,
            &mut q_heathbar,
            enemy_component,
            None,
        );
    }

    let mut enemies_iter = room.enemies.iter();
    for enemy_component in query.iter() {
        let Some(enemy) = enemies_iter.next() else {
            return;
        };
        change_visibility_according_to_enemy(
            &mut q_visibility,
            &mut q_heathbar,
            enemy_component,
            Some(enemy),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_handle = asset_server.load("door.png");

    for spawn_point in SPAWN_POINTS {
        // sprite
        let sprite_entity = commands
            .spawn(SpriteBundle {
                texture: enemy_handle.clone(),
                transform: Transform::from_translation(Vec3 {
                    x: spawn_point.x,
                    y: spawn_point.y,
                    z: 1.0,
                }),
                visibility: Visibility { is_visible: false },
                ..default()
            })
            .id();

        // healthbar
        let healthbar_entity = Progressbar::spawn(
            &mut commands,
            healthbar_new(Bar {
                position: Vec2::new(spawn_point.x, spawn_point.y + 30.0),
                ..default()
            }),
        );

        commands.spawn(EnemyComponent {
            spawn_point,
            sprite_entity,
            healthbar_entity,
        });
    }
}
