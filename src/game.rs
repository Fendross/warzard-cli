use crate::dialogue::Dialogue;
use crate::location::Location;

pub struct Game {
    pub dialogues: Dialogue,
    pub visited_locations: Vec<Location>
}

impl Game {
    pub fn new() -> Game {
        Self {
            dialogues: Dialogue::load_dialogues().unwrap(),
            visited_locations: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        init_terminal();
        // Intro dialogues roll.
        for dialogue in &self.dialogues.intro {
            println!("{}", dialogue);
        }
    }
}

fn init_terminal() {
    clear_terminal();
    println!("=== Warzard - Conquer your future ===\n");
}

fn clear_terminal() {
    print!("\x1b[H\x1b[J");
}
