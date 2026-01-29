use std::collections::HashMap;

pub fn init() -> HashMap<i16, String> {
    let mut lore = HashMap::new();

    lore.insert(1, "In a world where magic is everything, Vib was born with nothing.".to_string());
    lore.insert(2, "Vib also wondered if this lore implementation is cool. He was not that convinced.".to_string());
    
    return lore
}

pub fn get_lore_from_index(lore: &HashMap<i16, String>, i: i16) -> Option<&String> {
    return lore.get(&i)
}
