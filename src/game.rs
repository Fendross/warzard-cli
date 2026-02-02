pub struct Game {
    lore: Vec<String>,
}

impl Game {
    pub fn new() -> Self {
        Self { lore: build_lore() }
    }

    pub fn read_lore(&self, i: usize) {
        if let Some(phrase) = self.lore.get(i) {
            println!("{phrase}");
        } else {
            println!("Lore item does not exist.");
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
