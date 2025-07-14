# Rust Platformer Game

A simple 2D platforming game built with Rust and the Macroquad game engine. This project demonstrates clean architecture patterns, modular design, and idiomatic Rust practices for game development.

## Features

- **Smooth Physics**: Gravity, jumping, and collision detection
- **Player Movement**: WASD/Arrow key controls with double jump
- **Platform System**: Multiple platform types with visual indicators
- **Collectible System**: Coins, gems, and power-ups with animations
- **Scoring System**: Points from collectibles and distance traveled
- **Camera System**: Follows the player horizontally
- **Background Graphics**: Parallax scrolling with clouds, mountains, and trees
- **Game States**: Playing and Game Over states
- **Visual Polish**: Animated collectibles, floating motion, and particle effects

## Architecture

The game follows a clean, modular architecture:

```
src/
├── main.rs           # Game entry point and main loop
├── game/             # Game state management
│   ├── mod.rs        # Core game logic
│   └── states.rs     # Game state definitions
├── entities/         # Game entities (Player, Platforms, Collectibles)
│   ├── mod.rs        # Entity trait and physics body
│   ├── player.rs     # Player entity with movement
│   ├── platform.rs   # Platform entity with types
│   └── collectible.rs # Collectible items (coins, gems, power-ups)
├── physics/          # Physics simulation
│   ├── mod.rs        # Core physics (gravity, movement)
│   └── collision.rs  # Collision detection utilities
├── input/            # Input handling system
│   └── mod.rs        # Keyboard input management
└── graphics/         # Rendering utilities
    └── mod.rs        # Graphics helper functions
```

## Controls

- **Movement**: A/D or Left/Right arrow keys
- **Jump**: SPACE, W, or Up arrow key
- **Double Jump**: Press jump again while in air
- **Reset**: R key to restart the game
- **Game Over Recovery**: SPACE or ENTER to restart

## Installation & Running

### Prerequisites

- Rust (latest stable version)
- Linux environment with graphics support

### Quick Start

1. Clone or download the project
2. Navigate to the project directory:
   ```bash
   cd platformer
   ```
3. Run the game:
   ```bash
   cargo run
   ```

### Building for Release

```bash
cargo build --release
```

The optimized binary will be available at `target/release/platformer`.

## Game Mechanics

### Player Character
- **Size**: 32x32 pixels (blue rectangle with eyes)
- **Movement Speed**: 200 pixels/second
- **Jump Force**: 400 pixels/second upward
- **Double Jump**: Can jump twice before touching ground
- **Physics**: Affected by gravity (980 px/s²)

### Platforms
- **Ground Platform**: Full-width brown platform at bottom
- **Jump Platforms**: Green platforms at various heights
- **Visual Feedback**: Each platform type has unique decorations

### Collectibles
- **Coins**: Yellow circular items worth 10 points each
- **Gems**: Purple diamond-shaped items worth 50 points each
- **Power-ups**: Pink rectangular items worth 100 points each
- **Animations**: Floating motion and sparkle effects

### Camera System
- Follows player horizontally
- Smooth tracking with screen-center positioning
- Allows infinite horizontal exploration

## Code Highlights

### Entity Component System
The game uses a simple but effective entity system:

```rust
pub trait Entity {
    fn position(&self) -> Vec2;
    fn size(&self) -> Vec2;
    fn render(&self, camera_x: f32, camera_y: f32);
    fn update(&mut self);
}
```

### Physics Integration
Clean separation between physics and game logic:

```rust
// Apply physics
self.physics.apply_gravity(&mut self.player);
self.physics.update_position(&mut self.player);

// Check collisions
for platform in &self.platforms {
    self.physics.check_collision(&mut self.player, platform);
}
```

### Input Abstraction
Flexible input system supporting multiple key bindings:

```rust
if self.input.is_key_down(KeyCode::A) || self.input.is_key_down(KeyCode::Left) {
    self.player.move_left();
}
```

## Technical Details

### Dependencies
- **macroquad**: Cross-platform game engine for 2D games
- **Standard Library**: Uses only Rust std for core logic

### Performance
- 60 FPS target with vsync
- Efficient collision detection using AABB (Axis-Aligned Bounding Boxes)
- Minimal memory allocations during gameplay

### Cross-Platform
While developed for Linux, the codebase uses Macroquad which supports:
- Linux
- Windows
- macOS
- Web (WASM)

## Future Enhancements

The architecture supports easy extension with:

- **Enemies**: Add hostile entities with AI
- **Multiple Levels**: Level loading system
- **Audio**: Sound effects and music
- **Animations**: Sprite-based character animations
- **Moving Platforms**: Dynamic platform behaviors
- **Better Particle Effects**: Enhanced visual polish and feedback
- **Save System**: Progress persistence
- **Multiplayer**: Network play support

## Development

### Code Style
- Follows Rust conventions and idioms
- Comprehensive documentation
- Modular design for maintainability
- Clear separation of concerns

### Testing
```bash
# Check code compilation
cargo check

# Run with full warnings
cargo clippy

# Format code
cargo fmt
```

### Adding New Features

1. **New Entity Types**: Implement the `Entity` trait
2. **New Physics**: Extend the `Physics` struct
3. **New Input**: Add to `InputHandler`
4. **New Graphics**: Use utilities in `graphics` module

## License

This project is created for educational purposes. Feel free to use, modify, and learn from the code.

## Contributing

This is a learning project, but suggestions and improvements are welcome! The code is designed to be readable and educational for those learning Rust game development.

---

**Happy Gaming! 🎮**