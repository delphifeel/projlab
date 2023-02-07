struct NextTurnState {
    
}

impl NextTurnState {
    pub fn move_player(&mut self, direction: Direction) -> bool {
        let player_room = self.get_player_room();
        let new_room = match direction {
            Direction::Top => player_room.connections.top,
            Direction::Right => player_room.connections.right,
            Direction::Bottom => player_room.connections.bottom,
            Direction::Left => player_room.connections.left,
        };
        if let Some(new_room_id) = new_room {
            self.player_room_id = new_room_id;
            true
        } else {
            false
        }
    }

}
