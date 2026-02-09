use game::Game;

mod game;
mod dialogue;

fn main() {
    println!("Starting from scratch after a design session!");

    let mut game = Game::new();
    game.run();
}
