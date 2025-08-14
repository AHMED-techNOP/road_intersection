# Traffic Intersection Simulator - Code Organization

## File Structure

The code has been organized into separate modules for better maintainability and understanding:

```
src/
├── main.rs      - Main application entry point and game loop
├── types.rs     - Data structures and constants
├── traffic.rs   - Traffic light logic and timing
├── cars.rs      - Car spawning and movement behavior
└── graphics.rs  - All rendering and visual functions
```

## Module Descriptions

### 📁 `main.rs` 
- **Purpose**: Application entry point and main game loop
- **Contents**: 
  - Window configuration
  - Main game loop with input handling
  - Module imports and coordination

### 📁 `types.rs`
- **Purpose**: Core data structures and game constants
- **Contents**:
  - `TrafficLight` struct
  - `Car` struct  
  - `TrafficSystem` struct
  - All game constants (road width, timings, etc.)

### 📁 `traffic.rs`
- **Purpose**: Traffic light management and timing logic
- **Contents**:
  - `get_random_turn()` - Random turn generation
  - `calculate_green_time()` - Smart timing based on car count
  - `update_traffic_lights()` - Main traffic light state machine

### 📁 `cars.rs`
- **Purpose**: Car behavior, spawning and movement
- **Contents**:
  - `try_spawn_car_X()` functions for each direction
  - `should_stop_for_light()` - Traffic light compliance
  - `try_to_turn()` - Turning logic at intersections
  - `is_car_too_close()` - Collision prevention

### 📁 `graphics.rs`
- **Purpose**: All visual rendering and drawing functions
- **Contents**:
  - `draw_ui()` - User interface panel
  - `update_and_draw_cars()` - Car rendering with arrows
  - `draw_lights()` - Realistic traffic light rendering
  - `draw_roads()` - Road system with markings
  - `draw_dashed_line()` - Road marking helper

## Benefits of This Organization

1. **🎯 Separation of Concerns**: Each file has a specific responsibility
2. **📖 Easier to Understand**: Functions are grouped logically
3. **🔧 Easier to Maintain**: Changes to graphics don't affect traffic logic
4. **🚀 Easier to Extend**: New features can be added to appropriate modules
5. **👥 Team Friendly**: Multiple developers can work on different aspects

## How the Modules Work Together

1. **`main.rs`** orchestrates everything and handles user input
2. **`types.rs`** provides the data structures everyone uses
3. **`traffic.rs`** manages when lights change
4. **`cars.rs`** handles car behavior and movement
5. **`graphics.rs`** makes everything visible on screen

## Usage

The modular structure makes it easy to:
- **Modify graphics**: Edit `graphics.rs` 
- **Change traffic timing**: Edit `traffic.rs`
- **Add new car behaviors**: Edit `cars.rs`
- **Add new data types**: Edit `types.rs`
- **Change controls**: Edit `main.rs`

All modules work together seamlessly while keeping the code clean and organized!
