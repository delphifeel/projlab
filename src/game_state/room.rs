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
    pub connections: Connections,
}

impl Default for Room {
    fn default() -> Self {
        Self {
            connections: Connections::default(),
        }
    }
}
