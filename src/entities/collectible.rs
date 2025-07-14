use macroquad::prelude::*;

use super::{Entity, PhysicsBody};

#[derive(Debug, Clone)]
pub struct Collectible {
    pub body: PhysicsBody,
    pub color: Color,
    pub collected: bool,
    pub value: i32,
    pub collectible_type: CollectibleType,
    pub animation_time: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CollectibleType {
    Coin,
    Gem,
    PowerUp,
}

impl Collectible {
    pub fn new(x: f32, y: f32, collectible_type: CollectibleType) -> Self {
        let (color, value) = match collectible_type {
            CollectibleType::Coin => (YELLOW, 10),
            CollectibleType::Gem => (PURPLE, 50),
            CollectibleType::PowerUp => (PINK, 100),
        };

        Self {
            body: PhysicsBody::new(x, y, 16.0, 16.0),
            color,
            collected: false,
            value,
            collectible_type,
            animation_time: 0.0,
        }
    }

    pub fn new_coin(x: f32, y: f32) -> Self {
        Self::new(x, y, CollectibleType::Coin)
    }

    pub fn new_gem(x: f32, y: f32) -> Self {
        Self::new(x, y, CollectibleType::Gem)
    }

    pub fn new_power_up(x: f32, y: f32) -> Self {
        Self::new(x, y, CollectibleType::PowerUp)
    }

    pub fn collect(&mut self) -> i32 {
        if !self.collected {
            self.collected = true;
            self.value
        } else {
            0
        }
    }

    pub fn is_collected(&self) -> bool {
        self.collected
    }

    pub fn check_collection(&mut self, player_body: &PhysicsBody) -> i32 {
        if !self.collected && self.body.overlaps_with(player_body) {
            self.collect()
        } else {
            0
        }
    }

    pub fn get_bounds(&self) -> (f32, f32, f32, f32) {
        self.body.get_bounds()
    }
}

impl Entity for Collectible {
    fn position(&self) -> Vec2 {
        self.body.position
    }

    fn size(&self) -> Vec2 {
        self.body.size
    }

    fn render(&self, camera_x: f32, camera_y: f32) {
        if self.collected {
            return; // Don't render collected items
        }

        let render_x = self.body.position.x + camera_x;
        let render_y = self.body.position.y + camera_y;

        // Animate the collectible with floating motion
        let float_offset = (self.animation_time * 3.0).sin() * 3.0;
        let animated_y = render_y + float_offset;

        // Animate color brightness
        let brightness = 0.8 + 0.2 * (self.animation_time * 4.0).sin();
        let animated_color = Color::new(
            self.color.r * brightness,
            self.color.g * brightness,
            self.color.b * brightness,
            self.color.a,
        );

        match self.collectible_type {
            CollectibleType::Coin => {
                // Draw coin as a circle with inner circle
                draw_circle(
                    render_x + self.body.size.x / 2.0,
                    animated_y + self.body.size.y / 2.0,
                    self.body.size.x / 2.0,
                    animated_color,
                );
                draw_circle_lines(
                    render_x + self.body.size.x / 2.0,
                    animated_y + self.body.size.y / 2.0,
                    self.body.size.x / 2.0,
                    2.0,
                    ORANGE,
                );
                // Inner symbol
                draw_circle(
                    render_x + self.body.size.x / 2.0,
                    animated_y + self.body.size.y / 2.0,
                    3.0,
                    ORANGE,
                );
            }
            CollectibleType::Gem => {
                // Draw gem as a diamond shape
                let center_x = render_x + self.body.size.x / 2.0;
                let center_y = animated_y + self.body.size.y / 2.0;
                let size = self.body.size.x / 2.0;

                // Diamond vertices
                let top = Vec2::new(center_x, center_y - size);
                let right = Vec2::new(center_x + size * 0.7, center_y);
                let bottom = Vec2::new(center_x, center_y + size);
                let left = Vec2::new(center_x - size * 0.7, center_y);

                // Draw diamond
                draw_triangle(top, right, bottom, animated_color);
                draw_triangle(top, left, bottom, animated_color);

                // Draw sparkle effect
                let sparkle_time = self.animation_time * 6.0;
                if (sparkle_time % 2.0) > 1.5 {
                    draw_circle(center_x + 3.0, center_y - 3.0, 1.0, WHITE);
                    draw_circle(center_x - 2.0, center_y + 2.0, 1.0, WHITE);
                }
            }
            CollectibleType::PowerUp => {
                // Draw power-up as a glowing rectangle with plus sign
                draw_rectangle(
                    render_x,
                    animated_y,
                    self.body.size.x,
                    self.body.size.y,
                    animated_color,
                );
                draw_rectangle_lines(
                    render_x,
                    animated_y,
                    self.body.size.x,
                    self.body.size.y,
                    2.0,
                    WHITE,
                );

                // Draw plus sign
                let center_x = render_x + self.body.size.x / 2.0;
                let center_y = animated_y + self.body.size.y / 2.0;
                draw_line(
                    center_x,
                    center_y - 4.0,
                    center_x,
                    center_y + 4.0,
                    2.0,
                    WHITE,
                );
                draw_line(
                    center_x - 4.0,
                    center_y,
                    center_x + 4.0,
                    center_y,
                    2.0,
                    WHITE,
                );

                // Glow effect
                draw_circle(center_x, center_y, 12.0, Color::new(1.0, 1.0, 1.0, 0.1));
            }
        }
    }

    fn update(&mut self) {
        self.animation_time += get_frame_time();
    }
}
