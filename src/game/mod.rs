use macroquad::prelude::*;

use crate::entities::{Collectible, Entity, Platform, Player};
use crate::input::InputHandler;
use crate::physics::Physics;

pub mod states;

use states::GameState;

pub struct Game {
    pub state: GameState,
    pub player: Player,
    pub platforms: Vec<Platform>,
    pub collectibles: Vec<Collectible>,
    pub physics: Physics,
    pub input: InputHandler,
    pub camera_offset: Vec2,
    pub score: i32,
    pub time_survived: f32,
}

impl Game {
    pub fn new() -> Self {
        let mut platforms = Vec::new();

        // Create ground platforms with better visuals
        let mut ground_platform = Platform::new(0.0, screen_height() - 40.0, screen_width(), 40.0);
        ground_platform.platform_type = crate::entities::platform::PlatformType::Ground;
        ground_platform.color = BROWN;
        platforms.push(ground_platform);
        platforms.push(Platform::new(200.0, screen_height() - 120.0, 200.0, 20.0));
        platforms.push(Platform::new(500.0, screen_height() - 200.0, 150.0, 20.0));
        platforms.push(Platform::new(750.0, screen_height() - 280.0, 200.0, 20.0));

        // Create collectibles
        let mut collectibles = Vec::new();
        collectibles.push(Collectible::new_coin(150.0, screen_height() - 160.0));
        collectibles.push(Collectible::new_coin(300.0, screen_height() - 160.0));
        collectibles.push(Collectible::new_gem(550.0, screen_height() - 240.0));
        collectibles.push(Collectible::new_coin(800.0, screen_height() - 320.0));
        collectibles.push(Collectible::new_power_up(900.0, screen_height() - 320.0));
        collectibles.push(Collectible::new_coin(1200.0, screen_height() - 80.0));

        Self {
            state: GameState::Playing,
            player: Player::new(100.0, screen_height() - 100.0),
            platforms,
            collectibles,
            physics: Physics::new(),
            input: InputHandler::new(),
            camera_offset: Vec2::ZERO,
            score: 0,
            time_survived: 0.0,
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
                // Update time survived
                self.time_survived += get_frame_time();

                // Apply physics to player
                self.physics.apply_gravity(&mut self.player);
                self.physics.update_position(&mut self.player);

                // Check collisions with platforms
                for platform in &self.platforms {
                    self.physics.check_collision(&mut self.player, platform);
                }

                // Update collectibles and check for collection
                for collectible in &mut self.collectibles {
                    collectible.update();
                    let collected_value = collectible.check_collection(&self.player.body);
                    if collected_value > 0 {
                        self.score += collected_value;
                    }
                }

                // Update camera to follow player
                self.update_camera();

                // Update score based on horizontal distance traveled
                let distance_score = (self.player.position().x / 10.0) as i32;
                self.score = distance_score + (self.time_survived as i32 * 10);

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

        // Render background
        self.render_background(cam_x, cam_y);

        // Render platforms
        for platform in &self.platforms {
            platform.render(cam_x, cam_y);
        }

        // Render collectibles
        for collectible in &self.collectibles {
            collectible.render(cam_x, cam_y);
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

                // Display score and time
                draw_text(&format!("Score: {}", self.score), 10.0, 130.0, 24.0, YELLOW);
                draw_text(
                    &format!("Time: {:.1}s", self.time_survived),
                    10.0,
                    160.0,
                    20.0,
                    LIGHTGRAY,
                );

                // Display player position for debugging
                let pos = self.player.position();
                draw_text(
                    &format!("Position: ({:.0}, {:.0})", pos.x, pos.y),
                    10.0,
                    180.0,
                    16.0,
                    DARKGRAY,
                );
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
        self.score = 0;
        self.time_survived = 0.0;

        // Reset all collectibles
        for collectible in &mut self.collectibles {
            collectible.collected = false;
        }
    }

    fn render_background(&self, cam_x: f32, cam_y: f32) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Draw distant mountains with parallax (very slow movement)
        let mountain_offset = cam_x * 0.1;
        for i in 0..5 {
            let x = mountain_offset + i as f32 * 300.0 - 200.0;
            let height = 100.0 + (i as f32 * 30.0);
            draw_triangle(
                Vec2::new(x, screen_h - 40.0),
                Vec2::new(x + 150.0, screen_h - 40.0 - height),
                Vec2::new(x + 300.0, screen_h - 40.0),
                Color::new(0.4, 0.3, 0.6, 0.8),
            );
        }

        // Draw clouds with parallax (medium movement)
        let cloud_offset = cam_x * 0.3;
        let time = get_time() as f32;
        for i in 0..6 {
            let x = cloud_offset + i as f32 * 200.0 + (time * 10.0 + i as f32 * 50.0).sin() * 20.0;
            let y = 50.0 + i as f32 * 15.0 + (time * 2.0 + i as f32).sin() * 10.0;

            // Cloud body (multiple circles)
            draw_circle(x, y, 25.0, Color::new(1.0, 1.0, 1.0, 0.8));
            draw_circle(x + 20.0, y, 30.0, Color::new(1.0, 1.0, 1.0, 0.8));
            draw_circle(x + 45.0, y, 25.0, Color::new(1.0, 1.0, 1.0, 0.8));
            draw_circle(x + 25.0, y - 15.0, 20.0, Color::new(1.0, 1.0, 1.0, 0.8));
        }

        // Draw grass details on ground
        let grass_offset = cam_x * 0.8;
        for i in 0..100 {
            let grass_x = grass_offset + i as f32 * 20.0;
            let grass_y = screen_h - 40.0;
            let grass_height = 5.0 + (i as f32 * 0.5 + time).sin() * 2.0;

            draw_line(
                grass_x,
                grass_y,
                grass_x + (i as f32 * 0.1).sin() * 2.0,
                grass_y - grass_height,
                1.0,
                Color::new(0.2, 0.8, 0.2, 0.6),
            );
        }

        // Draw distant trees
        let tree_offset = cam_x * 0.5;
        for i in 0..10 {
            let tree_x = tree_offset + i as f32 * 120.0 + 50.0;
            let tree_y = screen_h - 40.0;

            // Tree trunk
            draw_rectangle(
                tree_x,
                tree_y - 50.0,
                8.0,
                50.0,
                Color::new(0.4, 0.2, 0.1, 0.7),
            );

            // Tree crown
            draw_circle(
                tree_x + 4.0,
                tree_y - 60.0,
                20.0,
                Color::new(0.1, 0.6, 0.1, 0.8),
            );
        }
    }
}
