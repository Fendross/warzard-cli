use game::Game;

mod game;
mod character;

fn main() {
    let mut game = Game::new();
    let mut idx_dialogue = 0;

    game.print_dialogue(idx_dialogue);
    idx_dialogue += 1;
    println!("Lore index value: {}", idx_dialogue);

    println!(
        "{} has been initialized! Our beloved non-hero! It starts out with: ", 
        game.character.name, 
    );
    println!("{} HP, Health Points", game.character.hp);
    println!("{} RP, Rage Points", game.character.rp);
    println!("{} XP, Experience Points", game.character.xp);
    println!("\x1b[32mHe's also at level {}.\x1b[0m", game.character.level);

    // Simulate xp gain.
    let gained_xp: u32 = 10;
    game.character.gain_xp(gained_xp);
    println!("{} gained {} XP, now it has a total of: {} XP.", game.character.name, gained_xp, game.character.xp);

    // Simulate level up.
    game.character.level_up();
    println!("{} has leveled up! His level is now: {}.", game.character.name, game.character.level);
}
