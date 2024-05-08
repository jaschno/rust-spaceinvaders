mod game;

#[macroquad::main("Space Invaders")]
async fn main() {
    let mut game = game::Game::new();

    game.run().await;
}