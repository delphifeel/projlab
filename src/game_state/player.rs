use super::action::Action;

pub struct Player {
    pub health: u16,
    pub action: Action,
}

pub const PLAYER_MAX_HEALTH: u16 = 100;

impl Player {
    pub fn new() -> Player {
        Player {
            health: PLAYER_MAX_HEALTH,
            action: Action::Free,
        }
    }
}
