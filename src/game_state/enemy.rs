pub enum EnemyType {
    Mage,
    Warrior,
}

pub struct Enemy {
    pub health: u16,
    pub enemy_type: EnemyType,
}

impl Enemy {
    pub fn new_mage() -> Self {
        Self {
            health: 10,
            enemy_type: EnemyType::Mage,
        }
    }
    pub fn new_warrior() -> Self {
        Self {
            health: 20,
            enemy_type: EnemyType::Warrior,
        }
    }
}
