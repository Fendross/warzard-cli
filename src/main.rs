use game::Game;

mod game;

fn main() {
    println!("Warzard - He that does not wield magic in a magic world");

    let game = Game::new();

    game.print_lore(0);
    game.print_lore(1);
    game.print_lore(2);
}
