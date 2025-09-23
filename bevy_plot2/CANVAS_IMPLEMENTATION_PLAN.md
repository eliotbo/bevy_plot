# Canvas Implementation Plan for bevy_plot2

## Overview
This document outlines the migration plan for implementing the canvas module from the original bevy_plot (Bevy 0.12) to bevy_plot2 (Bevy 0.16).

## Core Components to Migrate

### 1. Canvas Material & Shader System
**Files:** `canvas.wgsl`, canvas material implementation
**Key Features:**
- Custom 2D material shader for rendering the plot canvas
- Grid rendering with configurable tick periods
- Axes rendering with show/hide capability
- Background gradient support (background_color1, background_color2)
- Target crosshair rendering with configurable color
- Mouse position tracking in shader

**Implementation Steps:**
1. Port the `canvas.wgsl` shader to Bevy 0.16's shader system
2. Create `CanvasMaterial` as a `Material2d` implementation
3. Update shader imports to match Bevy 0.16 conventions
4. Implement `AsBindGroup` derive for the material

### 2. Font System
**Files:** `Roboto-Bold.ttf`
**Key Features:**
- Tick labels for axes
- Target position labels
- Configurable font size and color

**Implementation Steps:**
1. Copy `Roboto-Bold.ttf` to assets folder
2. Update text rendering to use Bevy 0.16's new text system
3. Replace deprecated text components with new ones (Text2d, TextFont, etc.)

### 3. User Interactions
**Files:** `inputs.rs`, `canvas_actions.rs`
**Key Features:**
- Mouse tracking and cursor position
- Zoom functionality (scroll wheel)
- Pan functionality (click and drag)
- Plot window resizing (corner dragging)
- Target position tracking

**Implementation Steps:**
1. Port `Cursor` resource for mouse tracking
2. Update event systems to Bevy 0.16 patterns
3. Implement zoom/pan controls
4. Add corner resize functionality

### 4. Canvas Parameters & Components
**Key Components:**
- `CanvasParams`: Position, scale, size, hover_radius
- `PlotLabel`: Component for axis labels
- `TargetLabel`: Component for target position labels
- `ResizePlotWindow`: Component for resize handles

**Implementation Steps:**
1. Define all component structs
2. Implement helper methods (within_rect, clicked_on_plot_corner)
3. Set up component queries and systems

### 5. Plot Management System
**Key Resources:**
- `PlotMap`: HashMap for managing multiple plots
- `ColorPalette`: Default colors for plots
- `TickLabelFont`: Font resource management

**Events:**
- `SpawnGraphEvent`
- `UpdatePlotLabelsEvent`
- `UpdateTargetLabelEvent`
- `RespawnAllEvent`
- `ReleaseAllEvent`

**Implementation Steps:**
1. Define all event types
2. Create PlotMap resource for plot management
3. Implement event handlers
4. Set up system ordering

## Migration Priority Order

### Phase 1: Foundation (Week 1)
1. Set up basic 2D canvas with Material2d
2. Port the shader file and get basic rendering working
3. Implement mouse tracking system

### Phase 2: Core Features (Week 2)
1. Add grid rendering
2. Add axes rendering
3. Implement zoom/pan controls
4. Add font system and labels

### Phase 3: Advanced Features (Week 3)
1. Target crosshair system
2. Plot resizing functionality
3. Multiple plot management
4. Color palette system

### Phase 4: Polish & Testing (Week 4)
1. Performance optimization
2. Bug fixes
3. Documentation
4. Example applications

## Key API Changes (Bevy 0.12 → 0.16)

### Text System
- Old: `Text::from_section()` with `TextStyle`
- New: `Text2d::new()` with `TextFont`

### Material System
- Material2d trait implementation remains similar
- AsBindGroup derive macro usage updated
- Shader imports changed from `bevy_sprite::` to new paths

### Event System
- EventReader/EventWriter APIs remain similar
- System parameter syntax updated

### Camera & Projection
- OrthographicProjection handling updated
- Camera2d component simplified

## Testing Strategy

1. **Unit Tests:** Test individual components (CanvasParams methods)
2. **Integration Tests:** Test event flow and system interactions
3. **Visual Tests:** Manual testing of rendering output
4. **Performance Tests:** FPS monitoring with multiple plots

## Success Criteria

- [ ] Canvas renders with correct background gradient
- [ ] Grid lines display at proper intervals
- [ ] Axes render with tick marks and labels
- [ ] Mouse interactions work (pan, zoom, hover)
- [ ] Target crosshair follows mouse correctly
- [ ] Text labels render with correct font
- [ ] Multiple plots can be managed simultaneously
- [ ] Performance maintains 60+ FPS with typical usage

## Known Challenges

1. **Shader Compatibility:** WGSL syntax changes between Bevy versions
2. **Text Rendering:** Significant API changes in text system
3. **Material Pipeline:** Updated Material2d trait requirements
4. **Performance:** Ensuring efficient instancing for multiple plots

## Notes

- The original implementation uses z-layering for rendering order:
  - Canvas: 0.0001
  - Text/Labels: 1.0001
  - Bezier curves: 1.10
  - Segments: 1.11
  - Markers: 1.12
  - Target text: 1.2

- Consider using Bevy's built-in batching/instancing for better performance
- May need to update to use Bevy's new required components pattern