use macroquad::prelude::*;

use crate::entities::{Entity, Platform, Player};

pub mod collision;

pub struct Physics {
    pub gravity: f32,
    pub terminal_velocity: f32,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            gravity: 980.0, // pixels per second squared
            terminal_velocity: 500.0,
        }
    }

    pub fn apply_gravity(&self, player: &mut Player) {
        if !player.is_on_ground() {
            let delta_time = get_frame_time();
            let mut velocity = player.velocity();

            // Apply gravity
            velocity.y += self.gravity * delta_time;

            // Limit terminal velocity
            if velocity.y > self.terminal_velocity {
                velocity.y = self.terminal_velocity;
            }

            player.set_velocity(velocity);
        }
    }

    pub fn update_position(&self, player: &mut Player) {
        let delta_time = get_frame_time();
        let velocity = player.velocity();
        let mut position = player.position();

        // Update position based on velocity
        position.x += velocity.x * delta_time;
        position.y += velocity.y * delta_time;

        player.set_position(position);

        // Update player state
        player.update();
    }

    pub fn check_collision(&self, player: &mut Player, platform: &Platform) {
        let player_bounds = (
            player.position().x,
            player.position().y,
            player.position().x + player.size().x,
            player.position().y + player.size().y,
        );

        let platform_bounds = platform.get_bounds();

        // Check if there's an overlap
        if self.rectangles_overlap(player_bounds, platform_bounds) {
            self.resolve_collision(player, platform);
        }
    }

    fn rectangles_overlap(&self, rect1: (f32, f32, f32, f32), rect2: (f32, f32, f32, f32)) -> bool {
        let (x1, y1, x2, y2) = rect1;
        let (ox1, oy1, ox2, oy2) = rect2;

        x1 < ox2 && x2 > ox1 && y1 < oy2 && y2 > oy1
    }

    fn resolve_collision(&self, player: &mut Player, platform: &Platform) {
        let player_bounds = (
            player.position().x,
            player.position().y,
            player.position().x + player.size().x,
            player.position().y + player.size().y,
        );

        let platform_bounds = platform.get_bounds();

        let (px1, py1, px2, py2) = player_bounds;
        let (plx1, ply1, plx2, ply2) = platform_bounds;

        // Calculate overlap distances
        let overlap_x = (px2 - plx1).min(plx2 - px1);
        let overlap_y = (py2 - ply1).min(ply2 - py1);

        let mut position = player.position();
        let mut velocity = player.velocity();

        // Resolve collision based on smallest overlap
        if overlap_x < overlap_y {
            // Horizontal collision
            if px1 < plx1 {
                // Player is to the left of platform
                position.x = plx1 - player.size().x;
            } else {
                // Player is to the right of platform
                position.x = plx2;
            }
            velocity.x = 0.0;
        } else {
            // Vertical collision
            if py1 < ply1 {
                // Player is above platform (landing)
                position.y = ply1 - player.size().y;
                velocity.y = 0.0;
                player.set_on_ground(true);
            } else {
                // Player is below platform (hitting head)
                position.y = ply2;
                velocity.y = 0.0;
            }
        }

        player.set_position(position);
        player.set_velocity(velocity);
    }

    pub fn check_bounds(&self, player: &mut Player) {
        let mut position = player.position();
        let mut velocity = player.velocity();

        // Keep player within screen bounds horizontally (optional)
        if position.x < 0.0 {
            position.x = 0.0;
            velocity.x = 0.0;
        }

        // Don't restrict right boundary to allow scrolling

        player.set_position(position);
        player.set_velocity(velocity);
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self::new()
    }
}
