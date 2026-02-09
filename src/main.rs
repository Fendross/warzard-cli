use game::Game;

mod game;
mod dialogue;
mod location;

fn main() {
    let mut game = Game::new();
    game.run();
}
