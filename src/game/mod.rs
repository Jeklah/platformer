use macroquad::prelude::*;

use crate::entities::{Entity, Platform, Player};
use crate::input::InputHandler;
use crate::physics::Physics;

pub mod states;

use states::GameState;

pub struct Game {
    pub state: GameState,
    pub player: Player,
    pub platforms: Vec<Platform>,
    pub physics: Physics,
    pub input: InputHandler,
    pub camera_offset: Vec2,
}

impl Game {
    pub fn new() -> Self {
        let mut platforms = Vec::new();

        // Create ground platforms
        platforms.push(Platform::new(
            0.0,
            screen_height() - 40.0,
            screen_width(),
            40.0,
        ));
        platforms.push(Platform::new(200.0, screen_height() - 120.0, 200.0, 20.0));
        platforms.push(Platform::new(500.0, screen_height() - 200.0, 150.0, 20.0));
        platforms.push(Platform::new(750.0, screen_height() - 280.0, 200.0, 20.0));

        Self {
            state: GameState::Playing,
            player: Player::new(100.0, screen_height() - 100.0),
            platforms,
            physics: Physics::new(),
            input: InputHandler::new(),
            camera_offset: Vec2::ZERO,
        }
    }

    pub fn handle_input(&mut self) {
        self.input.update();

        match self.state {
            GameState::Playing => {
                if self.input.is_key_down(KeyCode::A) || self.input.is_key_down(KeyCode::Left) {
                    self.player.move_left();
                }
                if self.input.is_key_down(KeyCode::D) || self.input.is_key_down(KeyCode::Right) {
                    self.player.move_right();
                }
                if self.input.is_key_pressed(KeyCode::Space)
                    || self.input.is_key_pressed(KeyCode::W)
                    || self.input.is_key_pressed(KeyCode::Up)
                {
                    self.player.jump();
                }
                if self.input.is_key_pressed(KeyCode::R) {
                    self.reset_game();
                }
            }
            GameState::GameOver => {
                if self.input.is_key_pressed(KeyCode::Space)
                    || self.input.is_key_pressed(KeyCode::Enter)
                {
                    self.reset_game();
                }
            }
        }
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::Playing => {
                // Apply physics to player
                self.physics.apply_gravity(&mut self.player);
                self.physics.update_position(&mut self.player);

                // Check collisions with platforms
                for platform in &self.platforms {
                    self.physics.check_collision(&mut self.player, platform);
                }

                // Update camera to follow player
                self.update_camera();

                // Check if player fell off the world
                if self.player.position().y > screen_height() + 100.0 {
                    self.state = GameState::GameOver;
                }
            }
            GameState::GameOver => {
                // Game over state - waiting for input to restart
            }
        }
    }

    pub fn render(&self) {
        // Apply camera offset
        let cam_x = -self.camera_offset.x;
        let cam_y = -self.camera_offset.y;

        // Render platforms
        for platform in &self.platforms {
            platform.render(cam_x, cam_y);
        }

        // Render player
        self.player.render(cam_x, cam_y);

        // Render UI
        self.render_ui();
    }

    fn update_camera(&mut self) {
        // Simple camera that follows the player horizontally
        let target_x = self.player.position().x - screen_width() / 2.0;
        self.camera_offset.x = target_x;

        // Keep camera above ground
        self.camera_offset.y = 0.0;
    }

    fn render_ui(&self) {
        match self.state {
            GameState::Playing => {
                draw_text("Use A/D or Arrow Keys to move", 10.0, 60.0, 20.0, WHITE);
                draw_text("Use SPACE/W/Up to jump", 10.0, 80.0, 20.0, WHITE);
                draw_text("Press R to reset", 10.0, 100.0, 20.0, WHITE);
            }
            GameState::GameOver => {
                let text = "GAME OVER";
                let font_size = 50.0;
                let text_width = measure_text(text, None, font_size as u16, 1.0).width;
                let x = (screen_width() - text_width) / 2.0;
                let y = screen_height() / 2.0;

                draw_text(text, x, y, font_size, RED);
                draw_text(
                    "Press SPACE or ENTER to restart",
                    x - 50.0,
                    y + 50.0,
                    20.0,
                    WHITE,
                );
            }
        }
    }

    fn reset_game(&mut self) {
        self.state = GameState::Playing;
        self.player = Player::new(100.0, screen_height() - 100.0);
        self.camera_offset = Vec2::ZERO;
    }
}
