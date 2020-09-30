pub mod cyclops_game;

fn main() {
    let g = cyclops_game::Game::new();
    g.start();
}
