use bevy::{
    color::palettes::css,
    input::mouse::MouseWheel,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CanvasMaterial {
    #[uniform(0)]
    pub mouse_pos: Vec2,
    #[uniform(0)]
    pub tick_period: Vec2,
    #[uniform(0)]
    pub bound_up: Vec2,
    #[uniform(0)]
    pub bound_lo: Vec2,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub zoom: f32,
    #[uniform(0)]
    pub size: Vec2,
    #[uniform(0)]
    pub outer_border: Vec2,
    #[uniform(0)]
    pub position: Vec2,
    #[uniform(0)]
    pub show_target: f32,
    #[uniform(0)]
    pub hide_contour: f32,
    #[uniform(0)]
    pub target_pos: Vec2,
    #[uniform(0)]
    pub background_color1: Vec4,
    #[uniform(0)]
    pub background_color2: Vec4,
    #[uniform(0)]
    pub target_color: Vec4,
    #[uniform(0)]
    pub show_grid: f32,
    #[uniform(0)]
    pub show_axes: f32,
}

impl Default for CanvasMaterial {
    fn default() -> Self {
        Self {
            mouse_pos: Vec2::ZERO,
            tick_period: Vec2::new(10.0, 10.0),
            bound_up: Vec2::new(100.0, 100.0),
            bound_lo: Vec2::new(-100.0, -100.0),
            time: 0.0,
            zoom: 1.0,
            size: Vec2::new(800.0, 600.0),
            outer_border: Vec2::new(0.1, 0.1),
            position: Vec2::ZERO,
            show_target: 0.0,
            hide_contour: 0.0,
            target_pos: Vec2::ZERO,
            background_color1: Vec4::new(0.1, 0.1, 0.2, 1.0),
            background_color2: Vec4::new(0.05, 0.05, 0.15, 1.0),
            target_color: Vec4::new(1.0, 0.5, 0.0, 1.0),
            show_grid: 1.0,
            show_axes: 1.0,
        }
    }
}

impl Material2d for CanvasMaterial {
    fn fragment_shader() -> ShaderRef {
        "canvas.wgsl".into()
    }
}

#[derive(Component, Clone)]
pub struct CanvasParams {
    pub position: Vec2,
    pub previous_position: Vec2,
    pub original_size: Vec2,
    pub scale: Vec2,
    pub previous_scale: Vec2,
    pub hover_radius: f32,
}

impl Default for CanvasParams {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            previous_position: Vec2::ZERO,
            original_size: Vec2::new(800.0, 600.0),
            scale: Vec2::ONE,
            previous_scale: Vec2::ONE,
            hover_radius: 20.0,
        }
    }
}

impl CanvasParams {
    pub fn within_rect(&self, position: Vec2) -> bool {
        let size = self.original_size * self.scale;
        position.x < self.position.x + size.x / 2.0
            && position.x > self.position.x - size.x / 2.0
            && position.y < self.position.y + size.y / 2.0
            && position.y > self.position.y - size.y / 2.0
    }
}

#[derive(Resource, Default)]
pub struct Cursor {
    pub position: Vec2,
    pub world_position: Vec2,
}

#[derive(Component)]
pub struct PlotCanvas;

#[derive(Component)]
pub struct CanvasMaterialHandle(pub Handle<CanvasMaterial>);

#[derive(Component)]
pub struct PlotLabel {
    pub axis: Axis,
    pub value: f32,
}

#[derive(Component)]
pub struct TargetLabel;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Component)]
pub struct MoveAxes {
    pub start_mouse_pos: Vec2,
    pub start_bounds: (Vec2, Vec2),
}

#[derive(Component)]
pub struct ZoomAxes;

pub fn update_cursor_system(
    mut cursor: ResMut<Cursor>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    if let Some(cursor_position) = window.cursor_position() {
        cursor.position = cursor_position;

        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            cursor.world_position = world_pos;
        }
    }
}

pub fn update_canvas_mouse_pos(
    cursor: Res<Cursor>,
    mut materials: ResMut<Assets<CanvasMaterial>>,
    query: Query<(&CanvasParams, &CanvasMaterialHandle)>,
) {
    for (canvas_params, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            if canvas_params.within_rect(cursor.world_position) {
                let relative_pos = cursor.world_position - canvas_params.position;
                material.mouse_pos = relative_pos;
            }
        }
    }
}

pub fn pan_canvas_system(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut materials: ResMut<Assets<CanvasMaterial>>,
    mut query: Query<(
        Entity,
        &CanvasParams,
        &CanvasMaterialHandle,
        Option<&MoveAxes>,
    )>,
) {
    for (entity, canvas_params, material_handle, move_axes) in query.iter_mut() {
        if canvas_params.within_rect(cursor.world_position) {
            if mouse_button_input.just_pressed(MouseButton::Left) {
                if let Some(material) = materials.get(&material_handle.0) {
                    commands.entity(entity).insert(MoveAxes {
                        start_mouse_pos: cursor.world_position,
                        start_bounds: (material.bound_lo, material.bound_up),
                    });
                }
            }
        }

        if let Some(move_axes) = move_axes {
            if mouse_button_input.pressed(MouseButton::Left) {
                if let Some(material) = materials.get_mut(&material_handle.0) {
                    let delta = cursor.world_position - move_axes.start_mouse_pos;
                    let scale = (material.bound_up - material.bound_lo) / canvas_params.original_size;

                    // For Y axis: flip the delta direction to match mathematical coordinates (Y up)
                    let corrected_delta = Vec2::new(delta.x, -delta.y);
                    material.bound_lo = move_axes.start_bounds.0 - corrected_delta * scale;
                    material.bound_up = move_axes.start_bounds.1 - corrected_delta * scale;
                }
            } else {
                commands.entity(entity).remove::<MoveAxes>();
            }
        }
    }
}

pub fn zoom_canvas_system(
    mut scroll_events: EventReader<MouseWheel>,
    cursor: Res<Cursor>,
    mut materials: ResMut<Assets<CanvasMaterial>>,
    query: Query<(&CanvasParams, &CanvasMaterialHandle)>,
) {
    for event in scroll_events.read() {
        for (canvas_params, material_handle) in query.iter() {
            if canvas_params.within_rect(cursor.world_position) {
                if let Some(material) = materials.get_mut(&material_handle.0) {
                    let zoom_factor = 1.0 + event.y * 0.1;
                    let center = (material.bound_up + material.bound_lo) / 2.0;

                    let half_range = (material.bound_up - material.bound_lo) / 2.0;
                    let new_half_range = half_range / zoom_factor;

                    material.bound_lo = center - new_half_range;
                    material.bound_up = center + new_half_range;
                    material.zoom *= zoom_factor;
                }
            }
        }
    }
}

pub fn update_tick_labels_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<Assets<CanvasMaterial>>,
    canvas_query: Query<(&CanvasParams, &CanvasMaterialHandle)>,
    label_query: Query<(Entity, &PlotLabel)>,
) {
    let font = asset_server.load("Roboto-Bold.ttf");

    for (canvas_params, material_handle) in canvas_query.iter() {
        if let Some(material) = materials.get(&material_handle.0) {
            // Remove old labels
            for (entity, _) in label_query.iter() {
                commands.entity(entity).despawn();
            }

            // Create new labels for X axis
            let x_range = material.bound_up.x - material.bound_lo.x;
            let x_ticks = (x_range / material.tick_period.x) as i32;
            let x_start = (material.bound_lo.x / material.tick_period.x).floor() * material.tick_period.x;

            for i in 0..=x_ticks + 1 {
                let value = x_start + i as f32 * material.tick_period.x;
                if value >= material.bound_lo.x && value <= material.bound_up.x {
                    let pixel_x = (value - material.bound_lo.x) / x_range * canvas_params.original_size.x * canvas_params.scale.x;
                    let pos = canvas_params.position + Vec2::new(pixel_x - canvas_params.original_size.x * canvas_params.scale.x / 2.0, -canvas_params.original_size.y * canvas_params.scale.y / 2.0 - 20.0);

                    commands.spawn((
                        Text2d::new(format!("{:.1}", value)),
                        TextFont {
                            font: font.clone(),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(css::WHITE.into()),
                        Transform::from_translation(pos.extend(1.0)),
                        PlotLabel {
                            axis: Axis::X,
                            value,
                        },
                    ));
                }
            }

            // Create new labels for Y axis
            let y_range = material.bound_up.y - material.bound_lo.y;
            let y_ticks = (y_range / material.tick_period.y) as i32;
            let y_start = (material.bound_lo.y / material.tick_period.y).floor() * material.tick_period.y;

            for i in 0..=y_ticks + 1 {
                let value = y_start + i as f32 * material.tick_period.y;
                if value >= material.bound_lo.y && value <= material.bound_up.y {
                    let pixel_y = (value - material.bound_lo.y) / y_range * canvas_params.original_size.y * canvas_params.scale.y;
                    let pos = canvas_params.position + Vec2::new(-canvas_params.original_size.x * canvas_params.scale.x / 2.0 - 30.0, pixel_y - canvas_params.original_size.y * canvas_params.scale.y / 2.0);

                    commands.spawn((
                        Text2d::new(format!("{:.1}", value)),
                        TextFont {
                            font: font.clone(),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(css::WHITE.into()),
                        Transform::from_translation(pos.extend(1.0)),
                        PlotLabel {
                            axis: Axis::Y,
                            value,
                        },
                    ));
                }
            }
        }
    }
}