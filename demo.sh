#!/bin/bash

# Rust Platformer Game Demo Script
# This script demonstrates the game features and provides quick access to different modes

echo "🎮 Rust Platformer Game Demo"
echo "============================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust/Cargo found"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Not in platformer project directory"
    echo "   Please run this script from the platformer project root"
    exit 1
fi

echo "✅ Project directory confirmed"
echo ""

# Function to display game features
show_features() {
    echo "🎯 Game Features:"
    echo "  • Player Movement: A/D or Arrow Keys"
    echo "  • Jump & Double Jump: SPACE, W, or Up Arrow"
    echo "  • Collectibles: Coins (10pts), Gems (50pts), Power-ups (100pts)"
    echo "  • Scoring: Distance traveled + collectibles"
    echo "  • Physics: Gravity, collision detection"
    echo "  • Camera: Follows player horizontally"
    echo "  • Background: Parallax scrolling effects"
    echo "  • Reset: Press R to restart anytime"
    echo ""
}

# Function to display controls
show_controls() {
    echo "🎮 Controls:"
    echo "  Movement:"
    echo "    A or ← : Move left"
    echo "    D or → : Move right"
    echo "    SPACE/W/↑ : Jump (can double jump)"
    echo ""
    echo "  Game:"
    echo "    R : Reset game"
    echo "    ESC : Quit game"
    echo ""
}

# Function to run the game
run_game() {
    echo "🚀 Starting the game..."
    echo "   Close the game window or press Ctrl+C to return to this menu"
    echo ""
    cargo run --release
}

# Function to check game
check_game() {
    echo "🔍 Checking game compilation..."
    cargo check
    if [ $? -eq 0 ]; then
        echo "✅ Game compiles successfully!"
    else
        echo "❌ Compilation errors found"
        return 1
    fi
}

# Function to build release version
build_release() {
    echo "🔨 Building release version..."
    cargo build --release
    if [ $? -eq 0 ]; then
        echo "✅ Release build successful!"
        echo "   Binary location: target/release/platformer"
        echo "   You can run it directly: ./target/release/platformer"
    else
        echo "❌ Release build failed"
        return 1
    fi
}

# Function to show project structure
show_structure() {
    echo "📁 Project Structure:"
    echo ""
    tree -I 'target|.git' || find . -type f -name "*.rs" | head -20
    echo ""
}

# Function to show game tips
show_tips() {
    echo "💡 Gameplay Tips:"
    echo "  • Use double jump to reach higher platforms"
    echo "  • Collect all items for maximum score"
    echo "  • The camera follows you - explore to the right!"
    echo "  • Your score increases with distance and collectibles"
    echo "  • Try to survive as long as possible"
    echo "  • Don't fall off the bottom of the screen!"
    echo ""
}

# Main menu
while true; do
    echo "Choose an option:"
    echo "1) 🎮 Run Game"
    echo "2) 🔍 Check Compilation"
    echo "3) 🔨 Build Release"
    echo "4) 🎯 Show Features"
    echo "5) 🎮 Show Controls"
    echo "6) 💡 Show Tips"
    echo "7) 📁 Show Project Structure"
    echo "8) 🚪 Exit"
    echo ""
    read -p "Enter your choice (1-8): " choice
    echo ""

    case $choice in
        1)
            show_controls
            run_game
            ;;
        2)
            check_game
            ;;
        3)
            build_release
            ;;
        4)
            show_features
            ;;
        5)
            show_controls
            ;;
        6)
            show_tips
            ;;
        7)
            show_structure
            ;;
        8)
            echo "👋 Thanks for trying the Rust Platformer!"
            echo "   Happy coding! 🦀"
            exit 0
            ;;
        *)
            echo "❌ Invalid option. Please choose 1-8."
            ;;
    esac
    echo ""
    echo "Press Enter to continue..."
    read
    echo ""
done
