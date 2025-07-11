use macroquad::prelude::*;

pub struct InputHandler {
    // Current frame key states
    current_keys: Vec<KeyCode>,
    // Previous frame key states
    previous_keys: Vec<KeyCode>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            current_keys: Vec::new(),
            previous_keys: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        // Store previous frame's keys
        self.previous_keys = self.current_keys.clone();

        // Clear current keys
        self.current_keys.clear();

        // Check all possible keys we care about
        let keys_to_check = vec![
            KeyCode::A,
            KeyCode::D,
            KeyCode::W,
            KeyCode::S,
            KeyCode::Space,
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::Enter,
            KeyCode::Escape,
            KeyCode::R,
            KeyCode::P,
        ];

        for key in keys_to_check {
            if is_key_down(key) {
                self.current_keys.push(key);
            }
        }
    }

    /// Check if a key is currently being held down
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.current_keys.contains(&key)
    }

    /// Check if a key was just pressed this frame
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.current_keys.contains(&key) && !self.previous_keys.contains(&key)
    }

    /// Check if a key was just released this frame
    pub fn is_key_released(&self, key: KeyCode) -> bool {
        !self.current_keys.contains(&key) && self.previous_keys.contains(&key)
    }

    /// Get the horizontal movement input (-1 for left, 1 for right, 0 for none)
    pub fn get_horizontal_input(&self) -> f32 {
        let mut horizontal = 0.0;

        if self.is_key_down(KeyCode::A) || self.is_key_down(KeyCode::Left) {
            horizontal -= 1.0;
        }

        if self.is_key_down(KeyCode::D) || self.is_key_down(KeyCode::Right) {
            horizontal += 1.0;
        }

        horizontal
    }

    /// Get the vertical movement input (-1 for up, 1 for down, 0 for none)
    pub fn get_vertical_input(&self) -> f32 {
        let mut vertical = 0.0;

        if self.is_key_down(KeyCode::W) || self.is_key_down(KeyCode::Up) {
            vertical -= 1.0;
        }

        if self.is_key_down(KeyCode::S) || self.is_key_down(KeyCode::Down) {
            vertical += 1.0;
        }

        vertical
    }

    /// Check if the jump key was pressed
    pub fn is_jump_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::Space)
            || self.is_key_pressed(KeyCode::W)
            || self.is_key_pressed(KeyCode::Up)
    }

    /// Check if the action key was pressed (for interacting with objects)
    pub fn is_action_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::Enter) || self.is_key_pressed(KeyCode::Space)
    }

    /// Check if the pause key was pressed
    pub fn is_pause_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::P) || self.is_key_pressed(KeyCode::Escape)
    }

    /// Check if the reset key was pressed
    pub fn is_reset_pressed(&self) -> bool {
        self.is_key_pressed(KeyCode::R)
    }

    /// Get all currently pressed keys (for debugging)
    pub fn get_pressed_keys(&self) -> &Vec<KeyCode> {
        &self.current_keys
    }

    /// Check if any key is pressed
    pub fn any_key_pressed(&self) -> bool {
        !self.current_keys.is_empty()
    }

    /// Check if any key was just pressed this frame
    pub fn any_key_just_pressed(&self) -> bool {
        for key in &self.current_keys {
            if !self.previous_keys.contains(key) {
                return true;
            }
        }
        false
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for common input patterns
pub mod helpers {
    use super::*;

    /// Check if movement keys are being held
    pub fn is_moving(input: &InputHandler) -> bool {
        input.get_horizontal_input() != 0.0 || input.get_vertical_input() != 0.0
    }

    /// Get normalized movement vector
    pub fn get_movement_vector(input: &InputHandler) -> Vec2 {
        let horizontal = input.get_horizontal_input();
        let vertical = input.get_vertical_input();

        let movement = Vec2::new(horizontal, vertical);

        // Normalize diagonal movement
        if movement.length() > 1.0 {
            movement.normalize()
        } else {
            movement
        }
    }

    /// Check if player wants to move left
    pub fn wants_to_move_left(input: &InputHandler) -> bool {
        input.is_key_down(KeyCode::A) || input.is_key_down(KeyCode::Left)
    }

    /// Check if player wants to move right
    pub fn wants_to_move_right(input: &InputHandler) -> bool {
        input.is_key_down(KeyCode::D) || input.is_key_down(KeyCode::Right)
    }

    /// Check if player just started moving left
    pub fn just_started_moving_left(input: &InputHandler) -> bool {
        (input.is_key_pressed(KeyCode::A) || input.is_key_pressed(KeyCode::Left))
            && !wants_to_move_right(input)
    }

    /// Check if player just started moving right
    pub fn just_started_moving_right(input: &InputHandler) -> bool {
        (input.is_key_pressed(KeyCode::D) || input.is_key_pressed(KeyCode::Right))
            && !wants_to_move_left(input)
    }

    /// Check if player just stopped moving horizontally
    pub fn just_stopped_moving_horizontal(input: &InputHandler) -> bool {
        input.get_horizontal_input() == 0.0 && {
            let prev_horizontal = if input.previous_keys.contains(&KeyCode::A)
                || input.previous_keys.contains(&KeyCode::Left)
            {
                -1.0
            } else if input.previous_keys.contains(&KeyCode::D)
                || input.previous_keys.contains(&KeyCode::Right)
            {
                1.0
            } else {
                0.0
            };
            prev_horizontal != 0.0
        }
    }
}
