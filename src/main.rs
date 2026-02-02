use game::Game;

mod game;
mod character;

fn main() {
    let game = Game::new();
    let mut lore_index = 0;

    game.print_lore(lore_index);
    lore_index += 1;
    println!("Lore index value: {}", lore_index);

    println!(
        "{} has been initialized! Our beloved non-hero! It starts out with: ", 
        game.character.name, 
    );
    println!("{} HP, Health Points", game.get_character_hp());
    println!("{} RP, Rage Points", game.get_character_rp());
    println!("{} XP, Experience Points", game.get_character_xp());
}
