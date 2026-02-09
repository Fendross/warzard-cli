use crate::dialogue::Dialogue;

pub struct Game {
    pub dialogues: Dialogue,
}

impl Game {
    pub fn new() -> Game {
        Self {
            dialogues: Dialogue::load_dialogues().unwrap(),
        }
    }

    pub fn run(&mut self) {
        println!("Ran game!");

        for dialogue in &self.dialogues.intro {
            println!("{}", dialogue);
        }
    }
}
