mod lore;

use std::collections::HashMap;

fn main() {
    println!("Warzard - He that does not wield magic in a magic world");

    // Init lore data.
    let lore: HashMap<i16, String> = lore::init();
    
    let str1 = lore::get_lore_from_index(&lore, 1);
    let str2 = lore::get_lore_from_index(&lore, 2);

    println!("{}\n{}", str1.unwrap(), str2.unwrap());
}
