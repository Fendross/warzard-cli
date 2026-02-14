use crate::dialogue::Dialogue;
use crate::location::Location;
use crate::hero::Hero;
use crate::enemy::Enemy;

use std::fs;

pub struct Game {
    pub hero: Hero,
    pub enemies: Vec<Enemy>,

    pub dialogues: Dialogue,

    pub current_location: Location,
    pub visited_locations: Vec<Location>,
}

impl Game {
    pub fn new() -> Game {
        Self {
            hero: Hero::new(),
            enemies: load_enemies(),

            dialogues: load_dialogues().unwrap(),

            current_location: Location::default(),
            visited_locations: vec![Location::default()],
        }
    }

    pub fn start(&mut self) {
        self.init_game();

        println!("Our hero: {:?}", self.hero);
        println!("First enemy: {:?}", self.enemies.get(0).as_slice());
        println!("Number of enemies stored: {:?}", self.enemies.len());
    }

    /// At the start of the game, clears the terminal and rolls the intro dialogues.
    /// Also, sets the location of the player in the Hareena.
    fn init_game(&mut self) {
        clear_terminal();

        print_dialogues(&self.dialogues.intro);
        self.update_location(Location::Hareena);

        println!("\n=== Warzard - Conquer your future ===\n");
        print_dialogues(&self.dialogues.hareena);
    }

    fn has_visited(&self, location: &Location) -> bool {
        self.visited_locations.contains(location)
    }

    fn update_location(&mut self, location: Location) {
        if !self.has_visited(&location) {
            self.visited_locations.push(location);
        }
        self.current_location = location
    }
}

// Data loaders.
pub fn load_dialogues() -> Result<Dialogue, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("src/resources/dialogues.json")?;
    let dialogues = serde_json::from_str(&data)?;
    Ok(dialogues)
}

pub fn load_enemies() -> Vec<Enemy> {
    let data = std::fs::read_to_string("src/resources/enemies.json").expect("File not found");
    serde_json::from_str(&data).expect("Error parsing JSON")
}

// Utils.
fn clear_terminal() {
    print!("\x1b[H\x1b[J");
}

pub fn print_dialogues(lines: &[String]) {
    for line in lines {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_visited() {
        let game: Game = Game::new();
        let location = Location::default();

        assert_eq!(game.current_location, location);
        assert!(game.has_visited(&location));
    }

    #[test]
    fn test_update_location() {
        let mut game: Game = Game::new();
        let location = Location::Hareena;
        game.update_location(location);

        assert!(game.has_visited(&location));
    }
}
