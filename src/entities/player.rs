use macroquad::prelude::*;

use super::{Entity, PhysicsBody};

#[derive(Debug, Clone)]
pub struct Player {
    pub body: PhysicsBody,
    pub move_speed: f32,
    pub jump_force: f32,
    pub max_jump_count: u32,
    pub current_jump_count: u32,
    pub color: Color,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            body: PhysicsBody::new(x, y, 32.0, 32.0),
            move_speed: 200.0,
            jump_force: -400.0,
            max_jump_count: 2, // Allow double jump
            current_jump_count: 0,
            color: BLUE,
        }
    }

    pub fn move_left(&mut self) {
        self.body.velocity.x = -self.move_speed;
    }

    pub fn move_right(&mut self) {
        self.body.velocity.x = self.move_speed;
    }

    pub fn jump(&mut self) {
        if self.current_jump_count < self.max_jump_count {
            self.body.velocity.y = self.jump_force;
            self.current_jump_count += 1;
            self.body.on_ground = false;
        }
    }

    pub fn reset_jump(&mut self) {
        self.current_jump_count = 0;
    }

    pub fn set_on_ground(&mut self, on_ground: bool) {
        self.body.on_ground = on_ground;
        if on_ground {
            self.reset_jump();
        }
    }

    // Getters for physics system
    pub fn position(&self) -> Vec2 {
        self.body.position
    }

    pub fn size(&self) -> Vec2 {
        self.body.size
    }

    pub fn velocity(&self) -> Vec2 {
        self.body.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.body.velocity = velocity;
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.body.position = position;
    }

    pub fn is_on_ground(&self) -> bool {
        self.body.on_ground
    }
}

impl Entity for Player {
    fn position(&self) -> Vec2 {
        self.body.position
    }

    fn size(&self) -> Vec2 {
        self.body.size
    }

    fn render(&self, camera_x: f32, camera_y: f32) {
        let render_x = self.body.position.x + camera_x;
        let render_y = self.body.position.y + camera_y;

        // Draw player as a rectangle
        draw_rectangle(
            render_x,
            render_y,
            self.body.size.x,
            self.body.size.y,
            self.color,
        );

        // Draw eyes to show facing direction
        let eye_size = 4.0;
        let eye_y = render_y + 8.0;

        // Left eye
        draw_rectangle(render_x + 8.0, eye_y, eye_size, eye_size, WHITE);

        // Right eye
        draw_rectangle(render_x + 20.0, eye_y, eye_size, eye_size, WHITE);

        // Draw velocity indicator (for debugging)
        if self.body.velocity.length() > 0.1 {
            let vel_end = Vec2::new(
                render_x + self.body.size.x / 2.0 + self.body.velocity.x * 0.1,
                render_y + self.body.size.y / 2.0 + self.body.velocity.y * 0.1,
            );

            draw_line(
                render_x + self.body.size.x / 2.0,
                render_y + self.body.size.y / 2.0,
                vel_end.x,
                vel_end.y,
                2.0,
                YELLOW,
            );
        }
    }

    fn update(&mut self) {
        // Apply friction to horizontal movement
        self.body.velocity.x *= 0.8;

        // Stop very small movements
        if self.body.velocity.x.abs() < 1.0 {
            self.body.velocity.x = 0.0;
        }
    }
}
