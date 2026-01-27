use std::{collections::HashMap};

fn main() {
    println!("Warzard - He that does not wield magic in a magic world");

    // Init lore data structure.
    let lore: HashMap<i16, String> = get_lore_hash_set();
    let str1 = lore.get(&1).expect("Default");
    let str2 = lore.get(&2).expect("Default");

    println!("{str1}\n{str2}");
}

fn get_lore_hash_set() -> HashMap<i16, String> {
    let mut lore = HashMap::new();

    lore.insert(1, "In a world where magic is everything, Vib was born with nothing.".to_string());
    lore.insert(2, "Vib also wondered if this lore implementation is cool. He was not that convinced.".to_string());
    
    return lore
}
