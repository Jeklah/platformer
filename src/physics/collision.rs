use macroquad::prelude::*;

use crate::entities::{PhysicsBody, Platform};

#[derive(Debug, Clone, PartialEq)]
pub enum CollisionSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct CollisionInfo {
    pub side: CollisionSide,
    pub overlap: f32,
    pub contact_point: Vec2,
}

pub struct CollisionDetector;

impl CollisionDetector {
    pub fn new() -> Self {
        Self
    }

    /// Check if two axis-aligned bounding boxes overlap
    pub fn aabb_overlap(rect1: (f32, f32, f32, f32), rect2: (f32, f32, f32, f32)) -> bool {
        let (x1, y1, x2, y2) = rect1;
        let (ox1, oy1, ox2, oy2) = rect2;

        x1 < ox2 && x2 > ox1 && y1 < oy2 && y2 > oy1
    }

    /// Get detailed collision information between two physics bodies
    pub fn get_collision_info(body1: &PhysicsBody, body2: &PhysicsBody) -> Option<CollisionInfo> {
        let bounds1 = body1.get_bounds();
        let bounds2 = body2.get_bounds();

        if !Self::aabb_overlap(bounds1, bounds2) {
            return None;
        }

        let (x1, y1, x2, y2) = bounds1;
        let (ox1, oy1, ox2, oy2) = bounds2;

        // Calculate overlap distances
        let overlap_x = (x2 - ox1).min(ox2 - x1);
        let overlap_y = (y2 - oy1).min(oy2 - y1);

        // Determine collision side based on minimum overlap
        let (side, overlap) = if overlap_x < overlap_y {
            // Horizontal collision
            if x1 < ox1 {
                (CollisionSide::Right, overlap_x)
            } else {
                (CollisionSide::Left, overlap_x)
            }
        } else {
            // Vertical collision
            if y1 < oy1 {
                (CollisionSide::Bottom, overlap_y)
            } else {
                (CollisionSide::Top, overlap_y)
            }
        };

        let contact_point = Vec2::new((x1 + x2) / 2.0, (y1 + y2) / 2.0);

        Some(CollisionInfo {
            side,
            overlap,
            contact_point,
        })
    }

    /// Check if a point is inside a rectangle
    pub fn point_in_rect(point: Vec2, rect: (f32, f32, f32, f32)) -> bool {
        let (x1, y1, x2, y2) = rect;
        point.x >= x1 && point.x <= x2 && point.y >= y1 && point.y <= y2
    }

    /// Check if a moving entity will collide with a static platform
    pub fn predict_collision(
        entity: &PhysicsBody,
        platform: &Platform,
        velocity: Vec2,
        delta_time: f32,
    ) -> Option<CollisionInfo> {
        let future_position = entity.position + velocity * delta_time;
        let future_body = PhysicsBody {
            position: future_position,
            ..*entity
        };

        Self::get_collision_info(&future_body, &platform.body)
    }

    /// Sweep test for continuous collision detection
    pub fn sweep_test(
        entity: &PhysicsBody,
        platform: &Platform,
        velocity: Vec2,
        delta_time: f32,
    ) -> Option<(f32, CollisionInfo)> {
        // Simple sweep test using multiple samples
        let samples = 10;
        let step = delta_time / samples as f32;

        for i in 1..=samples {
            let t = step * i as f32;
            let test_position = entity.position + velocity * t;
            let test_body = PhysicsBody {
                position: test_position,
                ..*entity
            };

            if let Some(collision) = Self::get_collision_info(&test_body, &platform.body) {
                return Some((t, collision));
            }
        }

        None
    }

    /// Check if an entity is standing on a platform
    pub fn is_on_platform(entity: &PhysicsBody, platform: &Platform, tolerance: f32) -> bool {
        let entity_bounds = entity.get_bounds();
        let platform_bounds = platform.get_bounds();

        let (ex1, _ey1, ex2, ey2) = entity_bounds;
        let (px1, py1, px2, _py2) = platform_bounds;

        // Check if entity is horizontally aligned with platform
        let horizontal_overlap = ex1 < px2 && ex2 > px1;

        // Check if entity is just above the platform
        let vertical_alignment = (ey2 - py1).abs() <= tolerance;

        horizontal_overlap && vertical_alignment
    }

    /// Get the distance between two physics bodies
    pub fn distance_between(body1: &PhysicsBody, body2: &PhysicsBody) -> f32 {
        let center1 = body1.position + body1.size / 2.0;
        let center2 = body2.position + body2.size / 2.0;

        (center2 - center1).length()
    }

    /// Check if two bodies are within a certain distance
    pub fn within_distance(body1: &PhysicsBody, body2: &PhysicsBody, distance: f32) -> bool {
        Self::distance_between(body1, body2) <= distance
    }
}

/// Utility functions for collision resolution
pub struct CollisionResolver;

impl CollisionResolver {
    /// Resolve collision by separating two bodies
    pub fn separate_bodies(
        body1: &mut PhysicsBody,
        body2: &PhysicsBody,
        collision: &CollisionInfo,
    ) {
        match collision.side {
            CollisionSide::Top => {
                body1.position.y = body2.position.y - body1.size.y;
                body1.velocity.y = body1.velocity.y.min(0.0);
                body1.on_ground = true;
            }
            CollisionSide::Bottom => {
                body1.position.y = body2.position.y + body2.size.y;
                body1.velocity.y = body1.velocity.y.max(0.0);
            }
            CollisionSide::Left => {
                body1.position.x = body2.position.x - body1.size.x;
                body1.velocity.x = body1.velocity.x.min(0.0);
            }
            CollisionSide::Right => {
                body1.position.x = body2.position.x + body2.size.x;
                body1.velocity.x = body1.velocity.x.max(0.0);
            }
        }
    }

    /// Apply bounce effect to a body
    pub fn apply_bounce(body: &mut PhysicsBody, collision: &CollisionInfo, restitution: f32) {
        match collision.side {
            CollisionSide::Top | CollisionSide::Bottom => {
                body.velocity.y = -body.velocity.y * restitution;
            }
            CollisionSide::Left | CollisionSide::Right => {
                body.velocity.x = -body.velocity.x * restitution;
            }
        }
    }

    /// Apply friction to a body
    pub fn apply_friction(body: &mut PhysicsBody, friction_coefficient: f32) {
        if body.on_ground {
            body.velocity.x *= 1.0 - friction_coefficient;
        }
    }
}

impl Default for CollisionDetector {
    fn default() -> Self {
        Self::new()
    }
}
