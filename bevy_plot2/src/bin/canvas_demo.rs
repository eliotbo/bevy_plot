use bevy::{
    color::palettes::css,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::mesh::{Mesh2d, MeshTag},
    sprite::{Material2dPlugin, MeshMaterial2d},
    window::{Window, WindowPlugin},
};
use bevy_plot2::*;
use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// Simple material for the data rectangles
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct DataRectMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for DataRectMaterial {
    fn fragment_shader() -> ShaderRef {
        "data_rect.wgsl".into()
    }
}

// Component to mark data rectangles
#[derive(Component)]
struct DataRect {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1920.0, 1080.0).into(), // Double the size
                    title: "Canvas Demo".to_string(),
                    ..default()
                }),
                ..default()
            }),
            BevyPlot2Plugin,
            Material2dPlugin::<DataRectMaterial>::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_time, toggle_features))
        .run();
}

// Generate quadratic curve data points
fn generate_quadratic_data() -> Vec<(f32, f32)> {
    let mut data_points = Vec::new();
    let num_points = 100;

    for i in 0..num_points {
        let x = (i as f32 - num_points as f32 / 2.0) / 5.0; // Range from -10 to 10
        let y = 0.2 * x * x - 3.0; // Quadratic curve: y = 0.2x² - 3
        data_points.push((x, y));
    }

    data_points
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CanvasMaterial>>,
    mut data_materials: ResMut<Assets<DataRectMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.05, 0.05, 0.1)),
            ..default()
        },
    ));

    // Create canvas material
    let mut canvas_material = CanvasMaterial::default();
    canvas_material.size = Vec2::new(800.0, 600.0);
    canvas_material.position = Vec2::ZERO;
    canvas_material.bound_lo = Vec2::new(-50.0, -50.0);
    canvas_material.bound_up = Vec2::new(50.0, 50.0);
    canvas_material.tick_period = Vec2::new(10.0, 10.0);
    canvas_material.show_grid = 1.0;
    canvas_material.show_axes = 1.0;
    canvas_material.show_target = 1.0;
    canvas_material.background_color1 = Vec4::new(0.1, 0.1, 0.2, 1.0);
    canvas_material.background_color2 = Vec4::new(0.2, 0.1, 0.3, 1.0);
    canvas_material.target_color = Vec4::new(1.0, 0.7, 0.0, 1.0);

    let canvas_material_handle = materials.add(canvas_material);

    // Create mesh for the canvas
    let mesh = meshes.add(Rectangle::new(800.0, 600.0));

    // Spawn the canvas entity with mesh and material
    // In Bevy 0.16, we use Mesh2d and MeshMaterial2d components
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(canvas_material_handle.clone()),
        Transform::from_translation(Vec3::ZERO),
        CanvasParams::default(),
        PlotCanvas,
        CanvasMaterialHandle(canvas_material_handle),
    ));

    // Generate and spawn instanced rectangles for quadratic curve data
    let data_points = generate_quadratic_data();

    // Create shared mesh and material for instancing
    let rect_mesh = meshes.add(Rectangle::new(6.0, 6.0)); // 3x bigger: 2.0 * 3 = 6.0
    let rect_material = data_materials.add(DataRectMaterial {
        color: LinearRgba::rgb(0.8, 0.4, 0.2), // Orange color for data points
    });

    // // Spawn instanced rectangles
    // for (i, (x, y)) in data_points.iter().enumerate() {
    //     commands.spawn((
    //         Mesh2d(rect_mesh.clone()),
    //         MeshMaterial2d(rect_material.clone()),
    //         Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)), // Will be updated by coordinate system
    //         MeshTag(i as u32),
    //         DataRect { x: *x, y: *y },
    //     ));
    // }

    // Instructions text
    commands.spawn((
        Text::new("Canvas Demo with Quadratic Data\n\nFeatures:\n- Orange rectangles show y = 0.2x² - 3\n- Automatic instancing (100 rectangles in 1 draw call)\n- Rectangles follow canvas coordinate system\n- Auto-culling when outside bounds\n- Responsive scaling with zoom\n\nControls:\n- Left Mouse: Pan\n- Scroll: Zoom\n- G: Toggle Grid\n- A: Toggle Axes\n- T: Toggle Target\n- B: Toggle Border\n- R: Reset View"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(css::WHITE.into()),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
    ));

    // FPS counter
    commands.spawn((
        Text::new("FPS: 0"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(css::YELLOW_GREEN.into()),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
        FpsText,
    ));

    // Position display
    commands.spawn((
        Text::new("Mouse: (0, 0)\nBounds: [(-50, -50), (50, 50)]"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(css::LIGHT_BLUE.into()),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            bottom: Val::Px(10.0),
            ..default()
        },
        PositionText,
    ));
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct PositionText;

fn update_time(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut materials: ResMut<Assets<CanvasMaterial>>,
    cursor: Res<Cursor>,
    query: Query<(&CanvasParams, &CanvasMaterialHandle)>,
    mut fps_query: Query<&mut Text, (With<FpsText>, Without<PositionText>)>,
    mut pos_query: Query<&mut Text, (With<PositionText>, Without<FpsText>)>,
) {
    // Print FPS to console every second
    static mut LAST_PRINT: f32 = 0.0;
    let current_time = time.elapsed_secs();
    unsafe {
        if current_time - LAST_PRINT > 1.0 {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    println!("FPS: {:.1}", value);
                    LAST_PRINT = current_time;
                }
            }
        }
    }
    // Update time in material
    for (canvas_params, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.time = time.elapsed_secs();

            // Update target position to follow a circular path
            let t = time.elapsed_secs();
            material.target_pos = Vec2::new(t.cos() * 20.0, t.sin() * 20.0);

            // Update position text
            if let Ok(mut text) = pos_query.single_mut() {
                let mouse_text = if canvas_params.within_rect(cursor.world_position) {
                    let relative_pos = cursor.world_position - canvas_params.position;
                    let scale = (material.bound_up - material.bound_lo) / canvas_params.original_size;
                    let local_pos = relative_pos * scale + (material.bound_up + material.bound_lo) / 2.0;
                    format!("Mouse: ({:.1}, {:.1})", local_pos.x, local_pos.y)
                } else {
                    "Mouse: Outside canvas".to_string()
                };

                text.0 = format!(
                    "{}\nBounds: [({:.1}, {:.1}), ({:.1}, {:.1})]\nZoom: {:.2}x",
                    mouse_text,
                    material.bound_lo.x,
                    material.bound_lo.y,
                    material.bound_up.x,
                    material.bound_up.y,
                    material.zoom
                );
            }
        }
    }

    // Update FPS
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            if let Ok(mut text) = fps_query.single_mut() {
                text.0 = format!("FPS: {:.0}", value);
            }
        }
    }
}

fn toggle_features(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<CanvasMaterial>>,
    query: Query<&CanvasMaterialHandle>,
) {
    for material_handle in query.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            // Toggle grid
            if keyboard_input.just_pressed(KeyCode::KeyG) {
                material.show_grid = if material.show_grid > 0.5 { 0.0 } else { 1.0 };
                println!("Grid: {}", if material.show_grid > 0.5 { "ON" } else { "OFF" });
            }

            // Toggle axes
            if keyboard_input.just_pressed(KeyCode::KeyA) {
                material.show_axes = if material.show_axes > 0.5 { 0.0 } else { 1.0 };
                println!("Axes: {}", if material.show_axes > 0.5 { "ON" } else { "OFF" });
            }

            // Toggle target
            if keyboard_input.just_pressed(KeyCode::KeyT) {
                material.show_target = if material.show_target > 0.5 { 0.0 } else { 1.0 };
                println!("Target: {}", if material.show_target > 0.5 { "ON" } else { "OFF" });
            }

            // Toggle border
            if keyboard_input.just_pressed(KeyCode::KeyB) {
                material.hide_contour = if material.hide_contour > 0.5 { 0.0 } else { 1.0 };
                println!("Border: {}", if material.hide_contour < 0.5 { "ON" } else { "OFF" });
            }

            // Reset view
            if keyboard_input.just_pressed(KeyCode::KeyR) {
                material.bound_lo = Vec2::new(-50.0, -50.0);
                material.bound_up = Vec2::new(50.0, 50.0);
                material.zoom = 1.0;
                println!("View reset");
            }
        }
    }
}

fn update_data_rectangles(
    materials: Res<Assets<CanvasMaterial>>,
    canvas_query: Query<(&CanvasParams, &CanvasMaterialHandle)>,
    mut data_query: Query<(&mut Transform, &mut Visibility, &DataRect)>,
) {
    if let Ok((canvas_params, material_handle)) = canvas_query.single() {
        if let Some(material) = materials.get(&material_handle.0) {
            // Get canvas bounds from the material
            let bound_lo = material.bound_lo;
            let bound_up = material.bound_up;
            let canvas_size = canvas_params.original_size;

            for (mut transform, mut visibility, data_rect) in data_query.iter_mut() {
                let x = data_rect.x;
                let y = data_rect.y;

                // Check if the data point is within the visible bounds
                if x >= bound_lo.x && x <= bound_up.x && y >= bound_lo.y && y <= bound_up.y {
                    // Show the rectangle
                    *visibility = Visibility::Visible;

                    // Transform from data coordinates to canvas pixel coordinates
                    let range_x = bound_up.x - bound_lo.x;
                    let range_y = bound_up.y - bound_lo.y;

                    // Normalize to [0, 1] within the current bounds
                    let norm_x = (x - bound_lo.x) / range_x;
                    let norm_y = (y - bound_lo.y) / range_y;

                    // Convert to canvas pixel coordinates (centered at canvas position)
                    let pixel_x = (norm_x - 0.5) * canvas_size.x;
                    let pixel_y = (norm_y - 0.5) * canvas_size.y;

                    // Update transform position
                    transform.translation.x = canvas_params.position.x + pixel_x;
                    transform.translation.y = canvas_params.position.y + pixel_y;
                    transform.translation.z = 0.1; // Always above canvas

                    // Scale rectangles based on zoom level (smaller when zoomed out)
                    let zoom_scale = 1.0 / (range_x / 100.0).max(0.1); // Base scale on x range
                    transform.scale = Vec3::splat(zoom_scale.clamp(0.1, 3.0));
                } else {
                    // Hide rectangles outside bounds
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}