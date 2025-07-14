use macroquad::prelude::*;

/// Game configuration constants that can be easily tweaked
pub struct GameConfig;

impl GameConfig {
    // Player Physics
    pub const PLAYER_MOVE_SPEED: f32 = 200.0;
    pub const PLAYER_JUMP_FORCE: f32 = -400.0;
    pub const PLAYER_SIZE: (f32, f32) = (32.0, 32.0);
    pub const PLAYER_MAX_JUMPS: u32 = 2;
    pub const PLAYER_COLOR: Color = BLUE;

    // Physics Constants
    pub const GRAVITY: f32 = 980.0;
    pub const TERMINAL_VELOCITY: f32 = 500.0;
    pub const FRICTION: f32 = 0.8;

    // Platform Settings
    pub const GROUND_HEIGHT: f32 = 40.0;
    pub const PLATFORM_COLOR: Color = GREEN;
    pub const GROUND_COLOR: Color = BROWN;

    // Collectible Settings
    pub const COIN_VALUE: i32 = 10;
    pub const GEM_VALUE: i32 = 50;
    pub const POWERUP_VALUE: i32 = 100;
    pub const COLLECTIBLE_SIZE: (f32, f32) = (16.0, 16.0);
    pub const COLLECTIBLE_ANIMATION_SPEED: f32 = 3.0;

    // Scoring
    pub const DISTANCE_SCORE_MULTIPLIER: f32 = 0.1;
    pub const TIME_SCORE_MULTIPLIER: i32 = 10;

    // Camera Settings
    pub const CAMERA_SMOOTHING: f32 = 0.1;
    pub const CAMERA_OFFSET_X: f32 = 0.0;
    pub const CAMERA_OFFSET_Y: f32 = 0.0;

    // Visual Settings
    pub const BACKGROUND_COLOR: Color = SKYBLUE;
    pub const UI_TEXT_COLOR: Color = WHITE;
    pub const SCORE_TEXT_COLOR: Color = YELLOW;
    pub const DEBUG_TEXT_COLOR: Color = DARKGRAY;

    // Background Elements
    pub const CLOUD_PARALLAX_SPEED: f32 = 0.3;
    pub const MOUNTAIN_PARALLAX_SPEED: f32 = 0.1;
    pub const TREE_PARALLAX_SPEED: f32 = 0.5;
    pub const GRASS_PARALLAX_SPEED: f32 = 0.8;

    // Game Rules
    pub const DEATH_Y_THRESHOLD: f32 = 100.0; // How far below screen before death
    pub const WORLD_WIDTH: f32 = 2000.0; // For minimap calculations

    // Input Settings
    pub const INPUT_BUFFER_TIME: f32 = 0.1; // Seconds to buffer jump input

    // Animation Settings
    pub const FLOAT_AMPLITUDE: f32 = 3.0;
    pub const FLOAT_FREQUENCY: f32 = 3.0;
    pub const SPARKLE_FREQUENCY: f32 = 6.0;
    pub const GLOW_PULSE_SPEED: f32 = 4.0;

    // Performance Settings
    pub const TARGET_FPS: i32 = 60;
    pub const MAX_PARTICLES: usize = 100;

    // UI Layout
    pub const UI_MARGIN: f32 = 10.0;
    pub const UI_LINE_HEIGHT: f32 = 25.0;
    pub const UI_FONT_SIZE: f32 = 20.0;
    pub const UI_LARGE_FONT_SIZE: f32 = 50.0;
    pub const UI_SMALL_FONT_SIZE: f32 = 16.0;

    // Debug Settings
    pub const SHOW_DEBUG_INFO: bool = true;
    pub const SHOW_VELOCITY_INDICATOR: bool = true;
    pub const SHOW_COLLISION_BOXES: bool = false;
    pub const SHOW_GRID: bool = false;
    pub const GRID_SIZE: f32 = 32.0;

    // Color Palette
    pub const PALETTE_PRIMARY: Color = Color::new(0.2, 0.4, 1.0, 1.0);
    pub const PALETTE_SECONDARY: Color = Color::new(1.0, 0.6, 0.2, 1.0);
    pub const PALETTE_ACCENT: Color = Color::new(0.8, 0.2, 0.8, 1.0);
    pub const PALETTE_SUCCESS: Color = Color::new(0.2, 0.8, 0.2, 1.0);
    pub const PALETTE_WARNING: Color = Color::new(1.0, 1.0, 0.2, 1.0);
    pub const PALETTE_DANGER: Color = Color::new(1.0, 0.2, 0.2, 1.0);

    // Game Balance
    pub const SCORE_THRESHOLD_BRONZE: i32 = 100;
    pub const SCORE_THRESHOLD_SILVER: i32 = 500;
    pub const SCORE_THRESHOLD_GOLD: i32 = 1000;

    // Audio Settings (for future implementation)
    pub const MASTER_VOLUME: f32 = 1.0;
    pub const SFX_VOLUME: f32 = 0.8;
    pub const MUSIC_VOLUME: f32 = 0.6;

    // Helper methods for common calculations
    pub fn screen_center() -> Vec2 {
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
    }

    pub fn ground_y() -> f32 {
        screen_height() - Self::GROUND_HEIGHT
    }

    pub fn death_threshold() -> f32 {
        screen_height() + Self::DEATH_Y_THRESHOLD
    }

    pub fn player_spawn_position() -> Vec2 {
        Vec2::new(100.0, Self::ground_y() - Self::PLAYER_SIZE.1)
    }

    pub fn ui_position(line: i32) -> Vec2 {
        Vec2::new(
            Self::UI_MARGIN,
            Self::UI_MARGIN + (line as f32) * Self::UI_LINE_HEIGHT,
        )
    }

    // Color utility methods
    pub fn with_alpha(color: Color, alpha: f32) -> Color {
        Color::new(color.r, color.g, color.b, alpha)
    }

    pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color::new(
            a.r + (b.r - a.r) * t,
            a.g + (b.g - a.g) * t,
            a.b + (b.b - a.b) * t,
            a.a + (b.a - a.a) * t,
        )
    }

    // Get collectible color by type
    pub fn collectible_color(collectible_type: &str) -> Color {
        match collectible_type {
            "coin" => Self::PALETTE_WARNING,
            "gem" => Self::PALETTE_ACCENT,
            "powerup" => PINK,
            _ => WHITE,
        }
    }

    // Get platform color by type
    pub fn platform_color(platform_type: &str) -> Color {
        match platform_type {
            "ground" => Self::GROUND_COLOR,
            "normal" => Self::PLATFORM_COLOR,
            "breakable" => ORANGE,
            "moving" => Self::PALETTE_SECONDARY,
            _ => GRAY,
        }
    }

    // Difficulty scaling (for future implementation)
    pub fn get_difficulty_multiplier(time_survived: f32) -> f32 {
        1.0 + (time_survived / 60.0) * 0.1 // Increase difficulty by 10% every minute
    }

    // Score formatting
    pub fn format_score(score: i32) -> String {
        if score >= 1000000 {
            format!("{:.1}M", score as f32 / 1000000.0)
        } else if score >= 1000 {
            format!("{:.1}K", score as f32 / 1000.0)
        } else {
            score.to_string()
        }
    }

    // Time formatting
    pub fn format_time(seconds: f32) -> String {
        let minutes = (seconds / 60.0) as i32;
        let secs = seconds % 60.0;
        if minutes > 0 {
            format!("{}:{:04.1}", minutes, secs)
        } else {
            format!("{:.1}s", secs)
        }
    }

    // Performance monitoring
    pub fn is_performance_good() -> bool {
        get_fps() as f32 >= (Self::TARGET_FPS as f32 * 0.8) // 80% of target FPS
    }

    // Debug information
    pub fn get_debug_info() -> String {
        format!(
            "FPS: {} | Frame: {:.2}ms | Memory: Debug",
            get_fps(),
            get_frame_time() * 1000.0
        )
    }
}

// Configuration presets for different difficulty levels
pub struct DifficultyPresets;

impl DifficultyPresets {
    pub fn easy() -> DifficultyConfig {
        DifficultyConfig {
            gravity: GameConfig::GRAVITY * 0.8,
            jump_force: GameConfig::PLAYER_JUMP_FORCE * 1.2,
            move_speed: GameConfig::PLAYER_MOVE_SPEED * 1.1,
            max_jumps: 3,
        }
    }

    pub fn normal() -> DifficultyConfig {
        DifficultyConfig {
            gravity: GameConfig::GRAVITY,
            jump_force: GameConfig::PLAYER_JUMP_FORCE,
            move_speed: GameConfig::PLAYER_MOVE_SPEED,
            max_jumps: GameConfig::PLAYER_MAX_JUMPS,
        }
    }

    pub fn hard() -> DifficultyConfig {
        DifficultyConfig {
            gravity: GameConfig::GRAVITY * 1.2,
            jump_force: GameConfig::PLAYER_JUMP_FORCE * 0.9,
            move_speed: GameConfig::PLAYER_MOVE_SPEED * 0.9,
            max_jumps: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DifficultyConfig {
    pub gravity: f32,
    pub jump_force: f32,
    pub move_speed: f32,
    pub max_jumps: u32,
}

// Environment presets for different visual themes
pub struct EnvironmentPresets;

impl EnvironmentPresets {
    pub fn day() -> EnvironmentConfig {
        EnvironmentConfig {
            background_color: SKYBLUE,
            ground_color: BROWN,
            platform_color: GREEN,
            cloud_alpha: 0.8,
            mountain_alpha: 0.8,
        }
    }

    pub fn sunset() -> EnvironmentConfig {
        EnvironmentConfig {
            background_color: Color::new(1.0, 0.7, 0.3, 1.0),
            ground_color: Color::new(0.4, 0.2, 0.1, 1.0),
            platform_color: Color::new(0.6, 0.4, 0.2, 1.0),
            cloud_alpha: 0.6,
            mountain_alpha: 0.9,
        }
    }

    pub fn night() -> EnvironmentConfig {
        EnvironmentConfig {
            background_color: Color::new(0.1, 0.1, 0.3, 1.0),
            ground_color: Color::new(0.2, 0.2, 0.2, 1.0),
            platform_color: Color::new(0.3, 0.3, 0.4, 1.0),
            cloud_alpha: 0.4,
            mountain_alpha: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    pub background_color: Color,
    pub ground_color: Color,
    pub platform_color: Color,
    pub cloud_alpha: f32,
    pub mountain_alpha: f32,
}
