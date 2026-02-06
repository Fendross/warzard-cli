use crate::character::Character;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct DialogueData {
    intro: Vec<String>,
    wandville: Vec<String>,
}

impl DialogueData {
    pub fn load_dialogues() -> Result<DialogueData, Box<dyn std::error::Error>> {
        let data = fs::read_to_string("src/resources/dialogues.json")?;
        let dialogues = serde_json::from_str(&data)?;
        Ok(dialogues)
    }
}

/// Tha main game container. It is composed of:
/// 
///     character -> a [`Character`] instance
///     lore -> a Vec<String> of lore phrases
pub struct Game {
    pub character: Character,

    pub dialogues: DialogueData,
}

impl Game {
    pub fn new() -> Self {
        Self { 
            character: Character::new("Vib".to_string(), 20, 5, 0),
            dialogues: DialogueData::load_dialogues().unwrap(),
        }
    }

    pub fn print_dialogue(&self, i: usize) {
        if let Some(phrase) = self.dialogues.intro.get(i) {
            println!("{phrase}");
        } else {
            println!("Dialogue does not exist.");
        }
    }

    #[cfg(test)]
    pub fn get_dialogue(&self, id: usize) -> String {
        match self.dialogues.intro.get(id) {
            Some(text) => text.clone(),
            None => "Dialogue does not exist.".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lore() {
        let game: Game = Game::new();
        assert_eq!(
            "In a world where magic is everything, Vib was born with nothing.".to_string(), 
            game.get_dialogue(0)
        );
    }

    #[test]
    fn test_read_lore_nonexistent() {
        let game: Game = Game::new();
        assert_eq!("Dialogue does not exist.".to_string(), game.get_dialogue(999));
    }
}
