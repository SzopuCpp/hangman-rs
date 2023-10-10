use crate::hangman::Game;

mod hangman;

fn main() {
    Game::new(String::from("Example word"), String::from("Example category")).run();
}
