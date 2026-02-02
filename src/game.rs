use crate::character::Character;

/// Tha main game container. It is composed of:
/// 
///     character -> a [`Character`] instance
///     lore -> a Vec<String> of lore phrases
pub struct Game {
    pub character: Character,

    pub lore: Vec<String>,
}

impl Game {
    pub fn new() -> Self {
        Self { 
            character: Character::new("Vib".to_string(), 20, 5, 0),
            lore: build_lore()
        }
    }

    pub fn get_character_hp(&self) -> u32 {
        self.character.get_hp()
    }

    pub fn get_character_rp(&self) -> u32 {
        self.character.get_rp()
    }

    pub fn get_character_xp(&self) -> u32 {
        self.character.get_xp()
    }

    pub fn print_lore(&self, i: usize) {
        if let Some(phrase) = self.lore.get(i) {
            println!("{phrase}");
        } else {
            println!("Lore item does not exist.");
        }
    }

    #[cfg(test)]
    pub fn get_lore_message(&self, id: usize) -> String {
        match self.lore.get(id) {
            Some(text) => text.clone(),
            None => "Lore item does not exist.".to_string(),
        }
    }
}

fn build_lore() -> Vec<String> {
    let mut lore: Vec<String> = Vec::new();
    lore.push("In a world where magic is everything, Vib was born with nothing.".to_string());
    lore.push("Vib also wondered if this lore implementation is cool. He was not that convinced.".to_string());
    lore.push("Game module seems better.".to_string());

    lore
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_lore() {
        let game: Game = Game::new();
        assert_eq!(3, game.lore.len());
    }

    #[test]
    fn test_read_lore() {
        let game: Game = Game::new();
        assert_eq!(
            "In a world where magic is everything, Vib was born with nothing.".to_string(), 
            game.get_lore_message(0)
        );
    }

    #[test]
    fn test_read_lore_nonexistent() {
        let game: Game = Game::new();
        assert_eq!("Lore item does not exist.".to_string(), game.get_lore_message(999));
    }
}
