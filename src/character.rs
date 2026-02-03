/// DISCLAIMER - This is a dummy implementation, I'm still fiddling around with Rust.
/// 
/// ---
/// 
/// Defines the [`Character`] data, alongside its main stats.
pub struct Character {
    pub name: String,

    max_hp: u32,
    pub hp: u32,

    max_rp: u32,
    pub rp: u32,

    pub xp: u32,
    pub level: u32
}

impl Character {
    pub fn new(_name: String, _max_hp: u32, _max_rp: u32, _xp: u32) -> Self {
        Self {
            name: _name,
            max_hp: _max_hp,
            hp: _max_hp,
            max_rp: _max_rp,
            rp: _max_rp,
            xp: _xp,
            level: 1,
        }
    }
    
    pub fn gain_xp(&mut self, _amount: u32) {
        self.xp += _amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_character_stats() {
        let char = Character::new("test".to_string(), 1, 2, 3);

        assert_eq!("test".to_string(), char.name);
        assert_eq!(1, char.hp);
        assert_eq!(2, char.rp);
        assert_eq!(3, char.xp);
        assert_eq!(1, char.level);
    }

    #[test]
    fn test_xp_gain() {
        let mut char = Character::new("test".to_string(), 1, 2, 3);

        assert_eq!(3, char.xp);
        char.gain_xp(10);
        assert_eq!(13, char.xp);
    }
}
