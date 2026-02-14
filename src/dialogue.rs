use serde::Deserialize;

#[derive(Deserialize)]
pub struct Dialogue {
    pub intro: Vec<String>,
    pub wandville: Vec<String>,
    pub hareena: Vec<String>,
    pub training: Vec<String>,
}
