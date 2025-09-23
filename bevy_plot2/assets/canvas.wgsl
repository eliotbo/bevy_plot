#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::view

struct CanvasMaterial {
    mouse_pos: vec2<f32>,
    tick_period: vec2<f32>,
    bound_up: vec2<f32>,
    bound_lo: vec2<f32>,
    time: f32,
    zoom: f32,
    size: vec2<f32>,
    outer_border: vec2<f32>,
    position: vec2<f32>,
    show_target: f32,
    hide_contour: f32,
    target_pos: vec2<f32>,
    background_color1: vec4<f32>,
    background_color2: vec4<f32>,
    target_color: vec4<f32>,
    show_grid: f32,
    show_axes: f32,
};

@group(2) @binding(0)
var<uniform> material: CanvasMaterial;

fn from_pix_to_local(pixel_pos: vec2<f32>) -> vec2<f32> {
    // Convert pixel coordinates to local plot coordinates
    let x_range = material.bound_up.x - material.bound_lo.x;
    let y_range = material.bound_up.y - material.bound_lo.y;

    // Normalize pixel position to [0, 1] range based on canvas size.
    // The mesh UV coordinates have `y = 0` at the top of the quad, so flip the
    // vertical axis to match the world-space convention used by canvas data.
    let normalized = pixel_pos / material.size;
    let normalized_y = 1.0 - normalized.y;

    // Map to plot bounds
    let local_x = material.bound_lo.x + normalized.x * x_range;
    let local_y = material.bound_lo.y + normalized_y * y_range;

    return vec2<f32>(local_x, local_y);
}

fn from_local_to_pixels(uv_local: vec2<f32>) -> vec2<f32> {
    var uv = uv_local;

    uv.x = uv.x * material.size.x / (1.0 + material.outer_border.x);
    uv.x = uv.x / (material.bound_up.x - material.bound_lo.x);

    uv.y = uv.y * material.size.y / (1.0 + material.outer_border.y);
    uv.y = uv.y / (material.bound_up.y - material.bound_lo.y);

    return uv;
}

fn draw_grid_line(p: vec2<f32>, tick_period: vec2<f32>, thickness: f32) -> f32 {
    let grid_x = abs(p.x % tick_period.x);
    let grid_y = abs(p.y % tick_period.y);

    // Create grid lines at regular intervals
    let line_x = 1.0 - smoothstep(0.0, thickness, grid_x);
    let line_y = 1.0 - smoothstep(0.0, thickness, grid_y);

    return max(line_x, line_y);
}

fn draw_axes(p: vec2<f32>, thickness: f32) -> f32 {
    let axis_x = 1.0 - smoothstep(0.0, thickness, abs(p.y));
    let axis_y = 1.0 - smoothstep(0.0, thickness, abs(p.x));

    return max(axis_x, axis_y);
}

fn draw_target_crosshair(p: vec2<f32>, target_pos: vec2<f32>, size: f32, thickness: f32) -> f32 {
    let dist = length(p - target_pos);
    let circle = smoothstep(size - thickness, size, dist) * (1.0 - smoothstep(size, size + thickness, dist));

    let cross_x = (1.0 - smoothstep(0.0, thickness, abs(p.y - target_pos.y))) * smoothstep(size * 0.3, size * 0.7, abs(p.x - target_pos.x));
    let cross_y = (1.0 - smoothstep(0.0, thickness, abs(p.x - target_pos.x))) * smoothstep(size * 0.3, size * 0.7, abs(p.y - target_pos.y));

    return max(circle, max(cross_x, cross_y));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // SOLID COLOR TEST
    return vec4<f32>(0.2, 0.2, 0.3, 1.0);

    // Grid - COMMENTED OUT
    // if material.show_grid > 0.5 {
    //     let grid = draw_grid_line(local_pos, material.tick_period, 1.0);
    //     color = mix(color, vec4<f32>(0.4, 0.4, 0.4, 1.0), grid * 0.5);
    // }

    // Axes
    if material.show_axes > 0.5 {
        let axes = draw_axes(local_pos, 1.0);
        color = mix(color, vec4<f32>(0.5, 0.5, 0.5, 1.0), axes);
    }

    // Target crosshair
    if material.show_target > 0.5 {
        let target_crosshair = draw_target_crosshair(local_pos, material.target_pos, 10.0, 0.5);
        color = mix(color, material.target_color, target_crosshair);
    }

    // Border
    if material.hide_contour < 0.5 {
        let border_x = smoothstep(0.0, 2.0, uv.x) * (1.0 - smoothstep(1.0 - 2.0 / material.size.x, 1.0, uv.x));
        let border_y = smoothstep(0.0, 2.0, uv.y) * (1.0 - smoothstep(1.0 - 2.0 / material.size.y, 1.0, uv.y));
        let border = 1.0 - border_x * border_y;
        color = mix(color, vec4<f32>(0.2, 0.2, 0.2, 1.0), border * 0.5);
    }

    return color;
}
