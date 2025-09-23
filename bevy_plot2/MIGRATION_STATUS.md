# Canvas Migration Status for bevy_plot2

## Completed ✅

### 1. Canvas Material & Shader System
- ✅ Ported `canvas.wgsl` shader to Bevy 0.16's shader system
- ✅ Created `CanvasMaterial` as a `Material2d` implementation with AsBindGroup derive
- ✅ Updated shader imports to match Bevy 0.16 conventions
- ✅ Implemented fragment shader with grid, axes, gradient background, and target crosshair

### 2. Font System
- ✅ Copied `Roboto-Bold.ttf` to assets folder
- ✅ Updated text rendering to use Bevy 0.16's new text system (Text2d, TextFont)

### 3. User Interactions
- ✅ Implemented `Cursor` resource for mouse tracking
- ✅ Added zoom functionality (scroll wheel)
- ✅ Added pan functionality (click and drag)
- ✅ Created MoveAxes component for drag state management

### 4. Canvas Components
- ✅ Defined `CanvasParams` component with position, scale, size, hover_radius
- ✅ Implemented `PlotLabel` component for axis labels
- ✅ Created `CanvasMaterialHandle` wrapper component for material handle queries
- ✅ Added helper methods (within_rect for hit testing)

### 5. Canvas Systems
- ✅ `update_cursor_system` - Tracks mouse position in world space
- ✅ `update_canvas_mouse_pos` - Updates material uniforms with mouse position
- ✅ `pan_canvas_system` - Handles canvas panning
- ✅ `zoom_canvas_system` - Handles canvas zooming
- ✅ `update_tick_labels_system` - Creates tick labels for axes

## Known Issues 🔧

### Material2d Bundle Pattern
The main `canvas_demo` binary has compilation issues due to Bevy 0.16's new component structure for 2D meshes. The Material2d bundle pattern needs further investigation for proper spawning of 2D material meshes.

### Deprecated API Warnings
- `get_single()` should be replaced with `single()`
- `get_single_mut()` should be replaced with `single_mut()`

## File Structure

```
bevy_plot2/
├── src/
│   ├── lib.rs                      # Main library with plugin definition
│   ├── canvas.rs                   # Canvas material and systems
│   ├── bin/
│   │   ├── canvas_demo.rs          # Full demo (has compilation issues)
│   │   └── canvas_demo_simple.rs   # Simple working demo
├── assets/
│   ├── canvas.wgsl                 # Canvas shader
│   └── Roboto-Bold.ttf            # Font for labels
├── Cargo.toml
└── MIGRATION_STATUS.md            # This file
```

## How to Test

1. **Simple Demo (Working):**
   ```bash
   cargo run --bin canvas_demo_simple
   ```

2. **Full Demo (Needs Fix):**
   ```bash
   cargo build --bin canvas_demo  # Currently has compilation errors
   ```

## Next Steps

1. Fix the Material2d bundle spawning pattern for Bevy 0.16
2. Address deprecated API warnings
3. Add plot resizing functionality
4. Implement multiple plot management
5. Add color palette system
6. Create more comprehensive examples

## Key Changes from Bevy 0.6 to 0.16

- Shader imports: `bevy_sprite::` → `bevy_sprite::mesh2d_vertex_output::`
- Text system completely redesigned: `Text::from_section()` → `Text2d::new()` with `TextFont`
- Material2d trait simplified but bundle spawning pattern changed
- Component queries for material handles need wrapper components
- Mouse/keyboard input APIs updated but similar in usage