/// DISCLAIMER - This is a dummy implementation, I'm still fiddling around with Rust.
/// 
/// ---
/// 
/// Defines the [`Character`] data, which for now is composed of:
/// 
///     name -> the character name
///     hp -> health points
///     rp -> rage points
///     xp -> experience points
pub struct Character {
    pub name: String,
    hp: u32,
    rp: u32,
    xp: u32,
}

impl Character {
    pub fn new(_name: String, _hp: u32, _rp: u32, _xp: u32) -> Self {
        Self {
            name: _name,
            hp: _hp,
            rp: _rp,
            xp: _xp,
        }
    }

    pub fn get_hp(&self) -> u32 {
        self.hp
    }

    pub fn get_rp(&self) -> u32 {
        self.rp
    }

    pub fn get_xp(&self) -> u32 {
        self.xp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_character_stats() {
        let char = Character::new("test".to_string(), 1, 2, 3);

        assert_eq!("test".to_string(), char.name);
        assert_eq!(1, char.get_hp());
        assert_eq!(2, char.get_rp());
        assert_eq!(3, char.get_xp());
    }
}
