use macroquad::prelude::*;

pub mod collectible;
pub mod platform;
pub mod player;

pub use collectible::Collectible;
pub use platform::Platform;
pub use player::Player;

// Base trait for all entities
pub trait Entity {
    fn position(&self) -> Vec2;
    fn size(&self) -> Vec2;
    fn render(&self, camera_x: f32, camera_y: f32);
    fn update(&mut self);
}

// Common physics properties for entities
#[derive(Debug, Clone)]
pub struct PhysicsBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: Vec2,
    pub on_ground: bool,
    pub mass: f32,
}

impl PhysicsBody {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
            size: Vec2::new(width, height),
            on_ground: false,
            mass: 1.0,
        }
    }

    pub fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (
            self.position.x,
            self.position.y,
            self.position.x + self.size.x,
            self.position.y + self.size.y,
        )
    }

    pub fn overlaps_with(&self, other: &PhysicsBody) -> bool {
        let (x1, y1, x2, y2) = self.get_bounds();
        let (ox1, oy1, ox2, oy2) = other.get_bounds();

        x1 < ox2 && x2 > ox1 && y1 < oy2 && y2 > oy1
    }
}
