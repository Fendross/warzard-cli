use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Dialogue {
    pub intro: Vec<String>,
    pub wandville: Vec<String>,
    pub hareena: Vec<String>,
    pub training: Vec<String>,
}

impl Dialogue {
    pub fn load_dialogues() -> Result<Dialogue, Box<dyn std::error::Error>> {
        let data = fs::read_to_string("src/resources/dialogues.json")?;
        let dialogues = serde_json::from_str(&data)?;
        Ok(dialogues)
    }
}
