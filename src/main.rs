use game::Game;

mod game;

fn main() {
    println!("Warzard - He that does not wield magic in a magic world");

    let game = Game::new();

    game.read_lore(0);
    game.read_lore(1);
    game.read_lore(2);
}
