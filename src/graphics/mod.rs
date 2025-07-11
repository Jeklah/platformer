use macroquad::prelude::*;

pub struct GraphicsUtils;

impl GraphicsUtils {
    /// Draw a filled rectangle with a border
    pub fn draw_rectangle_with_border(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        fill_color: Color,
        border_color: Color,
        border_width: f32,
    ) {
        // Draw filled rectangle
        draw_rectangle(x, y, width, height, fill_color);

        // Draw border
        draw_rectangle_lines(x, y, width, height, border_width, border_color);
    }

    /// Draw a gradient rectangle
    pub fn draw_gradient_rectangle(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        top_color: Color,
        bottom_color: Color,
    ) {
        // Simple gradient using multiple horizontal lines
        let steps = height as i32;
        let step_height = height / steps as f32;

        for i in 0..steps {
            let t = i as f32 / steps as f32;
            let color = Color::new(
                top_color.r + (bottom_color.r - top_color.r) * t,
                top_color.g + (bottom_color.g - top_color.g) * t,
                top_color.b + (bottom_color.b - top_color.b) * t,
                top_color.a + (bottom_color.a - top_color.a) * t,
            );

            draw_rectangle(x, y + i as f32 * step_height, width, step_height, color);
        }
    }

    /// Draw a circle with a border
    pub fn draw_circle_with_border(
        x: f32,
        y: f32,
        radius: f32,
        fill_color: Color,
        border_color: Color,
        border_width: f32,
    ) {
        // Draw filled circle
        draw_circle(x, y, radius, fill_color);

        // Draw border by drawing a slightly larger circle outline
        draw_circle_lines(x, y, radius, border_width, border_color);
    }

    /// Draw text with a shadow/outline effect
    pub fn draw_text_with_shadow(
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
        text_color: Color,
        shadow_color: Color,
        shadow_offset: Vec2,
    ) {
        // Draw shadow
        draw_text(
            text,
            x + shadow_offset.x,
            y + shadow_offset.y,
            font_size,
            shadow_color,
        );

        // Draw main text
        draw_text(text, x, y, font_size, text_color);
    }

    /// Draw text centered at a position
    pub fn draw_text_centered(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
        let text_x = x - text_dimensions.width / 2.0;
        let text_y = y + text_dimensions.height / 2.0;

        draw_text(text, text_x, text_y, font_size, color);
    }

    /// Draw a progress bar
    pub fn draw_progress_bar(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        progress: f32, // 0.0 to 1.0
        bg_color: Color,
        fill_color: Color,
        border_color: Color,
    ) {
        let clamped_progress = progress.clamp(0.0, 1.0);
        let fill_width = width * clamped_progress;

        // Draw background
        draw_rectangle(x, y, width, height, bg_color);

        // Draw fill
        if fill_width > 0.0 {
            draw_rectangle(x, y, fill_width, height, fill_color);
        }

        // Draw border
        draw_rectangle_lines(x, y, width, height, 2.0, border_color);
    }

    /// Draw a simple button
    pub fn draw_button(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        text: &str,
        is_hovered: bool,
        is_pressed: bool,
    ) -> bool {
        let button_color = if is_pressed {
            DARKGRAY
        } else if is_hovered {
            LIGHTGRAY
        } else {
            GRAY
        };

        // Draw button background
        draw_rectangle(x, y, width, height, button_color);
        draw_rectangle_lines(x, y, width, height, 2.0, BLACK);

        // Draw button text
        let text_color = if is_pressed { WHITE } else { BLACK };
        Self::draw_text_centered(text, x + width / 2.0, y + height / 2.0, 20.0, text_color);

        // Return if button was clicked
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            return mouse_pos.0 >= x
                && mouse_pos.0 <= x + width
                && mouse_pos.1 >= y
                && mouse_pos.1 <= y + height;
        }

        false
    }

    /// Check if mouse is over a rectangle
    pub fn is_mouse_over_rect(x: f32, y: f32, width: f32, height: f32) -> bool {
        let mouse_pos = mouse_position();
        mouse_pos.0 >= x
            && mouse_pos.0 <= x + width
            && mouse_pos.1 >= y
            && mouse_pos.1 <= y + height
    }

    /// Draw a simple animated sprite (using color cycling)
    pub fn draw_animated_rectangle(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        base_color: Color,
        animation_speed: f32,
    ) {
        let time = get_time() as f32;
        let wave = (time * animation_speed).sin();

        // Animate the brightness
        let brightness = 0.8 + 0.2 * wave;
        let animated_color = Color::new(
            base_color.r * brightness,
            base_color.g * brightness,
            base_color.b * brightness,
            base_color.a,
        );

        draw_rectangle(x, y, width, height, animated_color);
    }

    /// Draw a grid for debugging
    pub fn draw_debug_grid(camera_x: f32, camera_y: f32, grid_size: f32, color: Color) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Vertical lines
        let start_x = (camera_x / grid_size).floor() * grid_size - camera_x;
        let mut x = start_x;
        while x < screen_w {
            draw_line(x, 0.0, x, screen_h, 1.0, color);
            x += grid_size;
        }

        // Horizontal lines
        let start_y = (camera_y / grid_size).floor() * grid_size - camera_y;
        let mut y = start_y;
        while y < screen_h {
            draw_line(0.0, y, screen_w, y, 1.0, color);
            y += grid_size;
        }
    }

    /// Draw a simple particle effect
    pub fn draw_particles(center: Vec2, count: i32, radius: f32, color: Color, time_offset: f32) {
        let time = get_time() as f32 + time_offset;

        for i in 0..count {
            let angle = (i as f32 / count as f32) * 2.0 * std::f32::consts::PI;
            let wave = (time * 3.0 + i as f32 * 0.5).sin();
            let distance = radius * (0.5 + 0.5 * wave);

            let x = center.x + angle.cos() * distance;
            let y = center.y + angle.sin() * distance;

            let alpha = 0.5 + 0.5 * wave;
            let particle_color = Color::new(color.r, color.g, color.b, alpha);

            draw_circle(x, y, 3.0, particle_color);
        }
    }

    /// Draw a simple health bar
    pub fn draw_health_bar(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        current_health: f32,
        max_health: f32,
    ) {
        let health_percentage = (current_health / max_health).clamp(0.0, 1.0);

        // Choose color based on health percentage
        let fill_color = if health_percentage > 0.6 {
            GREEN
        } else if health_percentage > 0.3 {
            YELLOW
        } else {
            RED
        };

        Self::draw_progress_bar(
            x,
            y,
            width,
            height,
            health_percentage,
            DARKGRAY,
            fill_color,
            WHITE,
        );
    }

    /// Draw a simple minimap
    pub fn draw_minimap(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        world_width: f32,
        world_height: f32,
        player_pos: Vec2,
        platforms: &[Vec2], // Platform positions
    ) {
        // Draw minimap background
        draw_rectangle(x, y, width, height, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_rectangle_lines(x, y, width, height, 2.0, WHITE);

        // Scale factors
        let scale_x = width / world_width;
        let scale_y = height / world_height;

        // Draw platforms
        for platform_pos in platforms {
            let map_x = x + platform_pos.x * scale_x;
            let map_y = y + platform_pos.y * scale_y;
            draw_rectangle(map_x, map_y, 4.0, 2.0, GREEN);
        }

        // Draw player
        let player_map_x = x + player_pos.x * scale_x;
        let player_map_y = y + player_pos.y * scale_y;
        draw_circle(player_map_x, player_map_y, 3.0, BLUE);
    }
}

/// Color utility functions
pub mod colors {
    use super::*;

    /// Create a color with alpha
    pub fn with_alpha(color: Color, alpha: f32) -> Color {
        Color::new(color.r, color.g, color.b, alpha)
    }

    /// Lerp between two colors
    pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color::new(
            a.r + (b.r - a.r) * t,
            a.g + (b.g - a.g) * t,
            a.b + (b.b - a.b) * t,
            a.a + (b.a - a.a) * t,
        )
    }

    /// Get a rainbow color based on time
    pub fn rainbow_color(time: f32, speed: f32) -> Color {
        let hue = (time * speed) % 1.0;
        hsv_to_rgb(hue, 1.0, 1.0)
    }

    /// Convert HSV to RGB
    pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
        let c = v * s;
        let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 1.0 / 6.0 {
            (c, x, 0.0)
        } else if h < 2.0 / 6.0 {
            (x, c, 0.0)
        } else if h < 3.0 / 6.0 {
            (0.0, c, x)
        } else if h < 4.0 / 6.0 {
            (0.0, x, c)
        } else if h < 5.0 / 6.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Color::new(r + m, g + m, b + m, 1.0)
    }

    /// Common game colors
    pub const PLATFORM_GREEN: Color = Color::new(0.2, 0.8, 0.2, 1.0);
    pub const PLATFORM_BROWN: Color = Color::new(0.6, 0.4, 0.2, 1.0);
    pub const PLAYER_BLUE: Color = Color::new(0.2, 0.4, 1.0, 1.0);
    pub const DANGER_RED: Color = Color::new(1.0, 0.2, 0.2, 1.0);
    pub const COLLECTIBLE_YELLOW: Color = Color::new(1.0, 1.0, 0.2, 1.0);
    pub const UI_BACKGROUND: Color = Color::new(0.0, 0.0, 0.0, 0.7);
    pub const UI_BORDER: Color = Color::new(0.8, 0.8, 0.8, 1.0);
}
