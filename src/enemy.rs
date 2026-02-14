use serde::Deserialize;

const STARTING_LEVEL: i8 = 1;
const STARTING_HP: i16 = 10;
const STARTING_MP: i16 = 5;
const STARTING_SPEED: i16 = 4;

#[derive(Debug, Deserialize)]
pub struct Enemy {
    pub class: String, // TODO Make this a Class type.
    pub rarity: String,

    pub level: i8,

    pub total_hp: i16,
    pub current_hp: i16,
    pub total_mp: i16,
    pub current_mp: i16,
    pub speed: i16,
}

impl Enemy {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            class: String::from("Enemy"),
            rarity: String::from("Placeholder"),

            level: STARTING_LEVEL,

            total_hp: STARTING_HP,
            current_hp: STARTING_HP,
            total_mp: STARTING_MP,
            current_mp: STARTING_MP,
            speed: STARTING_SPEED,
        }
    }

    /// Currently ignores death.
    pub fn take_damage(&mut self, dmg: i16) {
        self.current_hp -= dmg;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_damage() {
        let mut enemy = Enemy::new();
        assert_eq!(enemy.current_hp, STARTING_HP);

        let dmg: i16 = 5;
        enemy.take_damage(dmg);

        assert_eq!(enemy.current_hp, STARTING_HP - dmg);
    }
}