use super::enemy::Enemy;

pub struct Connections {
    pub top: Option<usize>,
    pub right: Option<usize>,
    pub bottom: Option<usize>,
    pub left: Option<usize>,
}

impl Default for Connections {
    fn default() -> Self {
        Self {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }
}

pub struct Room {
    pub id: usize,
    pub connections: Connections,
    pub enemies: Vec<Enemy>,
}

impl Room {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            connections: Connections::default(),
            enemies: Vec::default(),
        }
    }
}
