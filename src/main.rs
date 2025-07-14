use macroquad::prelude::*;

mod config;
mod entities;
mod game;
mod graphics;
mod input;
mod physics;

use game::Game;

#[macroquad::main("Platformer")]
async fn main() {
    let mut game = Game::new();

    loop {
        // Handle input
        game.handle_input();

        // Update game state
        game.update();

        // Clear screen
        clear_background(SKYBLUE);

        // Render game
        game.render();

        // Show FPS
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 30.0, 20.0, WHITE);

        next_frame().await
    }
}
