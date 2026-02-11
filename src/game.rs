use crate::dialogue::Dialogue;
use crate::location::Location;

pub struct Game {
    pub dialogues: Dialogue,

    pub current_location: Location,
    pub visited_locations: Vec<Location>,
}

impl Game {
    pub fn new() -> Game {
        Self {
            dialogues: Dialogue::load_dialogues().unwrap(),

            current_location: Location::default(),
            visited_locations: vec![Location::default()],
        }
    }

    pub fn run(&mut self) {
        init_terminal();

        // Intro dialogues roll.
        for dialogue in &self.dialogues.intro {
            println!("{}", dialogue);
        }

        println!("Player currently in: {:?}.", self.current_location);
        self.update_location(Location::Hareena);
        println!("Player currently in: {:?}.", self.current_location);
    }

    fn has_visited(&self, location: &Location) -> bool {
        self.visited_locations.contains(location)
    }

    // Borrows to check, owns to store.
    fn update_location(&mut self, location: Location) {
        if !self.has_visited(&location) {
            self.visited_locations.push(location);
        }
        self.current_location = location
    }
}

// CLI Utils.
fn init_terminal() {
    clear_terminal();
    println!("=== Warzard - Conquer your future ===\n");
}

fn clear_terminal() {
    print!("\x1b[H\x1b[J");
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
