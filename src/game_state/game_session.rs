use bevy::prelude::*;
use std::collections::HashMap;

use super::{action::Action, direction::Direction, enemy::Enemy, player::Player, room::Room};

#[derive(Resource)]
pub struct GameSession {
    rooms_map: HashMap<usize, Room>,
    player: Player,
    player_room_id: usize,
}

fn build_rooms_map() -> HashMap<usize, Room> {
    let mut result = HashMap::new();

    // TODO: for now, build matrix with all connections 16x16
    const MATRIX_SIZE: usize = 16;

    for col in 1..MATRIX_SIZE + 1 {
        for row in 1..MATRIX_SIZE + 1 {
            let id: usize = (col - 1) * MATRIX_SIZE + row;
            let mut room = Room::new(id);
            if row != 1 {
                room.connections.left = Some(id - 1);
            }
            if row != MATRIX_SIZE {
                room.connections.right = Some(id + 1);
            }
            if col != 1 {
                room.connections.top = Some(id - MATRIX_SIZE);
            }
            if col != MATRIX_SIZE {
                room.connections.bottom = Some(id + MATRIX_SIZE);
            }
            result.insert(id, room);
        }
    }

    // add enemies in some rooms
    if let Some(room) = result.get_mut(&2) {
        room.enemies.push(Enemy::new_warrior());
        room.enemies.push(Enemy::new_mage());
    }
    if let Some(room) = result.get_mut(&17) {
        room.enemies.push(Enemy::new_mage());
        room.enemies.push(Enemy::new_mage());
    }

    result
}

impl GameSession {
    pub fn new() -> GameSession {
        GameSession {
            rooms_map: build_rooms_map(),
            player: Player::new(),
            player_room_id: 1,
        }
    }

    pub fn get_player_room(&self) -> &Room {
        match self.rooms_map.get(&self.player_room_id) {
            Some(r) => r,
            None => panic!("Room not found"),
        }
    }

    pub fn move_player(&mut self, direction: Direction) -> Option<&Action> {
        let player_room = self.get_player_room();
        let new_room = match direction {
            Direction::Top => player_room.connections.top,
            Direction::Right => player_room.connections.right,
            Direction::Bottom => player_room.connections.bottom,
            Direction::Left => player_room.connections.left,
        };
        if let Some(new_room_id) = new_room {
            self.player_room_id = new_room_id;
            self.player.action = Action::Resting(10.0);
            Some(&self.player.action)
        } else {
            None
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }
    pub fn get_player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
}
