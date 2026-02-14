use game::Game;

mod game;
mod hero;
mod enemy;
mod dialogue;
mod location;

fn main() {
    let mut game = Game::new();
    game.start();
}
