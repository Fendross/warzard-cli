const STARTING_LEVEL: i8 = 1;
const STARTING_XP: i16 = 0;
const STARTING_HP: i16 = 10;
const STARTING_RP: i16 = 5;
const STARTING_SPEED: i16 = 4;
const STARTING_GOLD: i16 = 100;

#[derive(Debug)]
pub struct Hero {
    pub name: String,
    pub class: String, // TODO Make this a Class type.

    pub level: i8,
    xp: i16,

    pub total_hp: i16,
    pub current_hp: i16,
    pub total_rp: i16,
    pub current_rp: i16,
    pub speed: i16,

    pub equip: String, // TODO Make this an Equipment type.
    pub gold: i16,
}

impl Hero {
    pub fn new() -> Self {
        Self {
            name: String::from("Vib"),
            class: String::from("Hero"),

            level: STARTING_LEVEL,
            xp: STARTING_XP,

            total_hp: STARTING_HP,
            current_hp: STARTING_HP,
            total_rp: STARTING_RP,
            current_rp: STARTING_RP,
            speed: STARTING_SPEED,

            equip: String::from("Basic"), // TODO Make this an Equipment type.
            gold: STARTING_GOLD,
        }
    }

    /// Currently ignores death.
    pub fn take_damage(&mut self, dmg: i16) {
        self.current_hp -= dmg;
    }

    /// When in `Location::Hideout`, fully restores stats.
    pub fn full_restore(&mut self) {
        self.current_hp = self.total_hp;
        self.current_rp = self.total_rp;
    }

    pub fn level_up() {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_damage() {
        let mut hero = Hero::new();
        assert_eq!(hero.current_hp, STARTING_HP);

        let dmg: i16 = 5;
        hero.take_damage(dmg);

        assert_eq!(hero.current_hp, STARTING_HP - dmg);
    }

    #[test]
    fn test_full_restore() {
        let mut hero = Hero::new();
        let dmg: i16 = 5;
        hero.take_damage(dmg);
        hero.full_restore();

        assert_eq!(hero.current_hp, STARTING_HP);
    }
}