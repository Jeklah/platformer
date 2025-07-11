use macroquad::prelude::*;

use super::{Entity, PhysicsBody};

#[derive(Debug, Clone)]
pub struct Platform {
    pub body: PhysicsBody,
    pub color: Color,
    pub platform_type: PlatformType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlatformType {
    Ground,
    Normal,
    Breakable,
    Moving,
}

impl Platform {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            body: PhysicsBody::new(x, y, width, height),
            color: GREEN,
            platform_type: PlatformType::Normal,
        }
    }

    pub fn new_ground(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            body: PhysicsBody::new(x, y, width, height),
            color: BROWN,
            platform_type: PlatformType::Ground,
        }
    }

    pub fn new_breakable(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            body: PhysicsBody::new(x, y, width, height),
            color: ORANGE,
            platform_type: PlatformType::Breakable,
        }
    }

    pub fn get_bounds(&self) -> (f32, f32, f32, f32) {
        self.body.get_bounds()
    }

    pub fn overlaps_with(&self, other: &PhysicsBody) -> bool {
        self.body.overlaps_with(other)
    }

    // Check if an entity is landing on top of this platform
    pub fn is_landing_on(&self, entity_body: &PhysicsBody, entity_prev_y: f32) -> bool {
        let (_, platform_top, _, _) = self.get_bounds();
        let (_, entity_bottom, _, _) = entity_body.get_bounds();

        // Entity is landing if:
        // 1. It's overlapping horizontally
        // 2. Entity's bottom is touching or slightly below platform top
        // 3. Entity was above the platform in the previous frame
        entity_prev_y + entity_body.size.y <= platform_top + 5.0
            && entity_bottom >= platform_top - 5.0
            && entity_body.velocity.y >= 0.0
    }
}

impl Entity for Platform {
    fn position(&self) -> Vec2 {
        self.body.position
    }

    fn size(&self) -> Vec2 {
        self.body.size
    }

    fn render(&self, camera_x: f32, camera_y: f32) {
        let render_x = self.body.position.x + camera_x;
        let render_y = self.body.position.y + camera_y;

        // Draw platform
        draw_rectangle(
            render_x,
            render_y,
            self.body.size.x,
            self.body.size.y,
            self.color,
        );

        // Draw platform border
        draw_rectangle_lines(
            render_x,
            render_y,
            self.body.size.x,
            self.body.size.y,
            2.0,
            DARKGRAY,
        );

        // Add visual indicators based on platform type
        match self.platform_type {
            PlatformType::Ground => {
                // Draw grass texture on top
                for i in 0..((self.body.size.x / 8.0) as i32) {
                    let grass_x = render_x + (i as f32) * 8.0;
                    draw_line(grass_x, render_y - 2.0, grass_x, render_y - 8.0, 2.0, LIME);
                }
            }
            PlatformType::Breakable => {
                // Draw crack pattern
                draw_line(
                    render_x + 10.0,
                    render_y + 5.0,
                    render_x + 30.0,
                    render_y + 15.0,
                    1.0,
                    DARKGRAY,
                );
                draw_line(
                    render_x + 40.0,
                    render_y + 8.0,
                    render_x + 55.0,
                    render_y + 12.0,
                    1.0,
                    DARKGRAY,
                );
            }
            PlatformType::Moving => {
                // Draw arrow to indicate movement
                let center_x = render_x + self.body.size.x / 2.0;
                let center_y = render_y + self.body.size.y / 2.0;
                draw_triangle(
                    Vec2::new(center_x - 5.0, center_y),
                    Vec2::new(center_x + 5.0, center_y - 3.0),
                    Vec2::new(center_x + 5.0, center_y + 3.0),
                    YELLOW,
                );
            }
            PlatformType::Normal => {
                // Normal platforms don't need extra decoration
            }
        }
    }

    fn update(&mut self) {
        // Platforms are generally static, but moving platforms would update here
        match self.platform_type {
            PlatformType::Moving => {
                // TODO: Implement moving platform logic
            }
            _ => {
                // Static platforms don't need updates
            }
        }
    }
}
