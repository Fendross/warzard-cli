use crate::dialogue::Dialogue;
use crate::location::Location;

pub struct Game {
    pub dialogues: Dialogue,
    pub current_location: Location,
    pub visited_locations: Vec<Location>,
}

impl Game {
    pub fn new() -> Game {
        // User starts directly in the Hareena, for the tutorial fight.
        Self {
            dialogues: Dialogue::load_dialogues().unwrap(),
            current_location: Location::Hareena,
            visited_locations: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        init_terminal();

        // Intro dialogues roll.
        for dialogue in &self.dialogues.intro {
            println!("{}", dialogue);
        }

        self.visited_locations.push(self.current_location.clone());
        println!("Player currently in: {:?}.", self.current_location);
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
