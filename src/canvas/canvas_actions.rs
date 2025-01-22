use bevy::text::{TextFont, TextLayout};
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    // // sprite::Mesh2dHandle,
    // text::{FontSmoothing, LineBreak, TextBounds},
};

use crate::canvas::*;
use crate::inputs::*;

use crate::canvas::RespawnAllEvent;
use crate::util::*;

// use crate::bezier::*;
use crate::plot::*;

fn spawn_axis_tick_labels(
    commands: &mut Commands,
    // asset_server: &Res<AssetServer>,
    plot_entity: Entity,
    text: &str,
    font_size: f32,
    position: Vec3,
    // justify: JustifyText,
    anchor: bevy::sprite::Anchor,

    // v_align: AlignItems,
    // h_align: AlignItems,
    font_color: Color,
    font: &Handle<Font>,
) {
    // let font = asset_server.load("fonts/Roboto-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size,
        ..default() // color: font_color,
    };
    // let text_alignment = TextAlignment {
    //     vertical: v_align,
    //     horizontal: h_align,
    // };

    // let text_font = TextFont {
    //     font: font.clone(),
    //     font_size,
    //     ..default()
    // };

    // let text_justification = JustifyText::Center;

    commands.entity(plot_entity).with_children(|parent| {
        parent
            .spawn((
                Text2d::new(text),
                text_font,
                // TextLayout::new_with_justify(justify),
                // text: Text::from_section(, text_font.clone()).with_alignment(text_alignment),
                Transform::from_translation(position),
                anchor,
            ))
            .insert(PlotLabel);
    });
}

pub(crate) fn update_target(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    maybe_font: Res<TickLabelFont>,
    // mut materials: ResMut<Assets<CanvasMaterial>>,
    mut plots: ResMut<PlotMap>,
    mut update_target_labels_event: EventReader<UpdateTargetLabelEvent>,
    taget_label_query: Query<Entity, With<TargetLabel>>,
    // canvas_query: Query<(Entity, &mut Handle<CanvasMaterial>, &Handle<Plot>)>,
    mut canvas_materials: ResMut<Assets<CanvasMaterial>>,
    // font_handle: Res<TickLabelFont>,
    // mut canvas_query: Query<&mut Canvas>,
) {
    if let Some(event) = update_target_labels_event.read().next() {
        for entity in taget_label_query.iter() {
            commands.entity(entity).despawn();
        }

        // let graph_sprite = canvas_query.get_mut(event.canvas_entity).unwrap();

        // let size = graph_sprite.original_size;

        let plot_id = event.plot_id.clone();
        let plot_entity = event.canvas_entity;
        // if let Some(plot) = materials.get_mut(plot_handle.clone()) {
        if let Some(plot) = plots.get_mut(&plot_id) {
            //

            if plot.show_target && plot.target_toggle {
                let target_text_z_plane = 1.2;
                let font_size = 16.0;

                let pos = plot.target_position;

                let target_str_x = format_numeric_label(&plot, pos.x, pos.x > 1000.0 || pos.x < 0.01);
                let target_str_y = format_numeric_label(&plot, pos.y, pos.y > 1000.0 || pos.y < 0.01);

                let target_str = format!("({}, {})", target_str_x, target_str_y);

                let offset = font_size * 0.2;
                let mut target_position =
                    plot.to_local(plot.target_position).extend(target_text_z_plane) + Vec3::new(offset, offset, 0.0);

                // let font = asset_server.load("fonts/Roboto-Regular.ttf");

                // let mut text_alignment = TextAlignment {
                //     vertical: VerticalAlign::Bottom,
                //     horizontal: HorizontalAlign::Left,
                // };

                let upper_limits = plot.canvas_position + plot.canvas_size / 2.0;
                let lower_limits = plot.canvas_position - plot.canvas_size / 2.0;

                if target_position.x > upper_limits.x - font_size * 1.0 {
                    // text_alignment.horizontal = HorizontalAlign::Right;
                    target_position.x -= font_size * 0.4;
                }

                if let Some(font_handle) = maybe_font.maybe_font.as_ref() {
                    let text_font = TextFont {
                        font: font_handle.clone(),
                        font_size,
                        ..default() // color: plot.target_label_color,
                    };

                    let offset = Vec3::new(font_size * 0.2, font_size * 0.02, 0.0);

                    if !(target_position.y > upper_limits.y - font_size * 1.2
                        || target_position.y < lower_limits.y + font_size * 0.2)
                    {
                        commands.entity(plot_entity).with_children(|parent| {
                            parent
                                .spawn((
                                    Text2d::new(target_str),
                                    text_font,
                                    TextLayout::new_with_justify(JustifyText::Left),
                                    TextColor(plot.target_label_color),
                                    Transform::from_translation(target_position + offset),
                                    bevy::sprite::Anchor::BottomLeft,
                                ))
                                .insert(TargetLabel);
                        });

                        // commands.entity(plot_entity).push_children(&[label_entity]);
                    }
                }
            }

            // update canvas shader
            if let Some(canvas_mat) = canvas_materials.get_mut(&event.canvas_material_handle) {
                canvas_mat.update_all(&plot);
            }
        }
    }
}

pub(crate) fn update_plot_labels(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    maybe_font: Res<TickLabelFont>,
    // mut materials: ResMut<Assets<CanvasMaterial>>,
    mut plots: ResMut<PlotMap>,
    mut update_plot_labels_event: EventReader<UpdatePlotLabelsEvent>,
    plot_label_query: Query<Entity, With<PlotLabel>>,
    mut canvas_query: Query<&mut CanvasParams>,
) {
    // If there is a stack of UpdatePlotLabelsEvent, only read the first one.
    if let Some(font_handle) = maybe_font.maybe_font.as_ref() {
        if let Some(event) = update_plot_labels_event.read().next() {
            let plot_id = event.plot_id.clone();

            // if let Some(plot) = materials.get_mut(plot_handle.clone()) {

            if let Some(plot) = plots.get_mut(&plot_id.clone()) {
                if !plot.hide_tick_labels {
                    for entity in plot_label_query.iter() {
                        commands.entity(entity).despawn();
                    }
                    let plot_entity = event.canvas_entity;

                    let graph_sprite = canvas_query.get_mut(event.canvas_entity).unwrap();

                    let size = graph_sprite.original_size;

                    let font_size = 16.0;

                    // TODO: clean this up using to_local inside the Plot struct
                    let graph_y = size.y / (1. + plot.outer_border.y);
                    let graph_x = size.x / (1. + plot.outer_border.x);

                    let x_edge = size.x / (1. + plot.outer_border.x) / 2.0;
                    let y_edge = size.y / (1. + plot.outer_border.y) / 2.0;

                    let x_range = plot.bounds.up.x - plot.bounds.lo.x;
                    let y_range = plot.bounds.up.y - plot.bounds.lo.y;

                    let text_z_plane = 1.0001;

                    ///////////////////////////// x_axis labels  /////////////////////////////
                    {
                        // distance from center for
                        let center_dist_y = -graph_y / 2.0 + font_size * 0.;

                        // iterate
                        let iter_x = x_edge * 2.0 / x_range;

                        // integer corresponding to lowest x tick
                        let bottom_x = (plot.bounds.lo.x / plot.tick_period.x).abs().floor() as i64
                            * (plot.bounds.lo.x).signum() as i64;

                        // integer corresponding to highest x tick
                        let top_x = (plot.bounds.up.x / plot.tick_period.x).abs().floor() as i64
                            * (plot.bounds.up.x).signum() as i64;

                        let max_abs_x = (plot.tick_period.x * bottom_x as f32)
                            .abs()
                            .max(plot.tick_period.x * top_x as f32);

                        for i in bottom_x..(top_x + 1) {
                            if plot.hide_half_ticks && (i % 2).abs() == 1 {
                                continue;
                            }
                            // // let j = i;

                            // let mut x_str = format!(
                            //     "{:.1$}",
                            //     i as f32 * plot.tick_period.x,
                            //     plot.significant_digits
                            // );

                            // // scientific notation if the numbers are larger than 1000
                            // if max_abs_x >= 1000.0 || max_abs_x < 0.01 {
                            //     // x_str = format!("{:+e}", i as f32 * plot.tick_period.x);
                            //     x_str = format!(
                            //         "{:+.1$e}",
                            //         i as f32 * plot.tick_period.x,
                            //         plot.significant_digits
                            //     );
                            //     if let Some(rest) = x_str.strip_prefix("+") {
                            //         x_str = rest.to_string();
                            //     }
                            // }

                            let x_str = format_numeric_label(
                                &plot,
                                i as f32 * plot.tick_period.x,
                                max_abs_x >= 1000.0 || max_abs_x < 0.01,
                            );

                            // leftmost position on the x axis
                            let x0 = x_edge * (-1.0 - plot.bounds.lo.x * 2.0 / x_range);

                            // iterator for each label
                            let x_pos = iter_x * i as f32 * plot.tick_period.x;

                            let font_offset_x = font_size * 0.2;
                            // if the tick label is too far to the left, do not spawn it
                            if (x0 + x_pos + font_offset_x + graph_x / 2.0) > font_size * 3.0
                        // if the tick label is too far to the right, do not spawn it
                        && (x0 + x_pos + font_offset_x - graph_x / 2.0) < -font_size * 3.0
                            {
                                spawn_axis_tick_labels(
                                    &mut commands,
                                    // &asset_server,
                                    plot_entity,
                                    &x_str,
                                    font_size,
                                    Vec2::new(x0 + x_pos + font_offset_x, center_dist_y).extend(text_z_plane),
                                    bevy::sprite::Anchor::BottomLeft,
                                    // JustifyText::Right,
                                    // AlignItems::Start,
                                    // AlignItems::End,
                                    plot.tick_label_color,
                                    &font_handle,
                                );
                            }
                        }
                    }

                    ////////////////////////////////// y_axis labels //////////////////////////////////
                    {
                        // distance from center for
                        let center_dist_x = -graph_x / 2.0 + font_size * 0.20;

                        // iterate
                        let iter_y = y_edge * 2.0 / y_range;

                        // integer corresponding to lowest y tick
                        let bottom_y = (plot.bounds.lo.y / plot.tick_period.y).abs().floor() as i64
                            * (plot.bounds.lo.y).signum() as i64;

                        // integer corresponding to highest y tick
                        let top_y = (plot.bounds.up.y / plot.tick_period.y).abs().floor() as i64
                            * (plot.bounds.up.y).signum() as i64;

                        let max_abs_y = (plot.tick_period.y * bottom_y as f32)
                            .abs()
                            .max(plot.tick_period.y * top_y as f32);

                        for i in bottom_y..top_y + 1 {
                            if plot.hide_half_ticks && (i % 2).abs() == 1 {
                                continue;
                            }

                            // let mut y_str = format!(
                            //     "{:.1$}",
                            //     i as f32 * plot.tick_period.y,
                            //     plot.significant_digits
                            // );

                            // // scientific notation if the numbers are larger than 1000
                            // if max_abs_y >= 1000.0 || max_abs_y < 0.01 {
                            //     y_str = format!(
                            //         "{:+.1$e}",
                            //         i as f32 * plot.tick_period.y,
                            //         plot.significant_digits
                            //     );
                            //     if let Some(rest) = y_str.strip_prefix("+") {
                            //         y_str = rest.to_string();
                            //     }
                            // }

                            let y_str = format_numeric_label(
                                &plot,
                                i as f32 * plot.tick_period.y,
                                // scientific notation if the numbers are larger than 1000 or smaller than 0.01
                                max_abs_y >= 1000.0 || max_abs_y < 0.01,
                            );

                            // leftmost position on the x axis
                            let y0 = y_edge * (-1.0 - plot.bounds.lo.y * 2.0 / y_range);

                            // iterator for each label
                            let y_pos = iter_y * i as f32 * plot.tick_period.y;

                            let font_offset_y = -font_size * 0.0;

                            if (y0 + y_pos + font_offset_y + graph_y / 2.0) > font_size * 1.2
                                && (y0 + y_pos + font_offset_y - graph_y / 2.0) < -font_size * 0.0
                            {
                                spawn_axis_tick_labels(
                                    &mut commands,
                                    // &asset_server,
                                    plot_entity,
                                    &y_str,
                                    font_size,
                                    Vec2::new(center_dist_x, y0 + y_pos + font_offset_y).extend(0.0001),
                                    // JustifyText::Left,
                                    bevy::sprite::Anchor::TopLeft,
                                    // VerticalAlign::Top,
                                    // HorizontalAlign::Left,
                                    plot.tick_label_color,
                                    font_handle,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

// delays the update of the plot labels until the next frame, after which the
// plot canvas is definitely spawned
pub(crate) fn wait_for_graph_spawn(
    mut wait_for_update_labels_event: EventReader<WaitForUpdatePlotLabelsEvent>,
    mut update_labels_event: EventWriter<UpdatePlotLabelsEvent>,
) {
    for event in wait_for_update_labels_event.read() {
        update_labels_event.send(UpdatePlotLabelsEvent {
            plot_id: event.plot_id,
            canvas_entity: event.quad_entity,
        });
    }
}

// spawns a graph a shader_param_handle
pub(crate) fn spawn_graph(
    mut commands: Commands,
    mut spawn_graph_event: EventReader<SpawnGraphEvent>,
    mut materials: ResMut<Assets<CanvasMaterial>>,
    mut plots: ResMut<PlotMap>,

    mut meshes: ResMut<Assets<Mesh>>,

    // mut update_labels_event: EventWriter<UpdatePlotLabelsEvent>,
    // mut spawn_markers_event: EventWriter<SpawnMarkersEvent>,
    // mut spawn_beziercurve_event: EventWriter<SpawnBezierCurveEvent>,
    mut wait_for_update_labels_event: EventWriter<WaitForUpdatePlotLabelsEvent>,
    mut change_canvas_material_event: EventWriter<RespawnAllEvent>,
) {
    for event in spawn_graph_event.read() {
        let plot_id = event.plot_id;

        if let Some(plot) = plots.plots.get_mut(&plot_id) {
            let material = CanvasMaterial::new(&plot);
            // let material = CanvasMaterial::new(&plot);

            let canvas_material_handle = materials.add(material);

            // In 0.15, we use the Mesh2d / MeshMaterial2d components.
            let plot_entity = commands
                .spawn((
                    // The quad itself
                    // Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::new(plot.canvas_size)))),
                    Mesh2d(meshes.add(Mesh::from(bevy::math::prelude::Rectangle::from_size(plot.canvas_size)))),
                    // Our custom material
                    MeshMaterial2d(canvas_material_handle.clone()),
                    // Basic transform
                    Transform::from_translation(plot.canvas_position.extend(0.0001)),
                    GlobalTransform::default(),
                    // Our custom component
                    event.canvas.clone(),
                    PlotIdComponent(plot_id),
                ))
                .id();

            plot.entity = Some(plot_entity);

            wait_for_update_labels_event.send(WaitForUpdatePlotLabelsEvent {
                quad_entity: plot_entity.clone(),
                plot_id: plot_id,
            });

            change_canvas_material_event.send(RespawnAllEvent {
                // canvas_material_handle: canvas_material_handle.clone(),
                plot_id,
            });

            // // spawn each analytical curve
            // plot.data.bezier_groups.iter().enumerate().for_each(|(k, _)| {
            //     spawn_beziercurve_event.send(SpawnBezierCurveEvent {
            //         group_number: k,
            //         plot_id,
            //     })
            // });
        }
    }
}
fn format_numeric_label(plot: &Plot, label: f32, scientific_notation: bool) -> String {
    // scientific notation if the numbers are larger than 1000
    // if max_abs_y >= 1000.0 || max_abs_y < 0.01 {
    if scientific_notation {
        let mut formatted = format!("{:+.1$e}", label, plot.significant_digits);
        if let Some(rest) = formatted.strip_prefix("+") {
            formatted = rest.to_string();
        }
        return formatted;
    } else {
        format!("{:.1$}", label, plot.significant_digits)
    }
}

pub(crate) fn update_mouse_target(
    // mut commands: Commands,
    mut my_canvas_mats: ResMut<Assets<CanvasMaterial>>,
    //
    // canvas_query: Query<(Entity, &GraphSprite, &Handle<CanvasMaterial>)>,
    mut canvas_query: Query<(Entity, &mut MeshMaterial2d<CanvasMaterial>, &PlotIdComponent)>,
    mut update_target_labels_event: EventWriter<UpdateTargetLabelEvent>,

    cursor: Res<Cursor>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut plots: ResMut<PlotMap>,
) {
    if mouse_button_input.pressed(MouseButton::Middle) {
        for (canvas_entity, mut canvas_material, plot_id_component) in &mut canvas_query {
            // println!("{:?}", "CHANGING SHADER");
            // if let Some(plot) = my_canvas_mat.get_mut(plot_handle) {

            let plot_id = plot_id_component.0;
            let plot = plots.plots.get_mut(&plot_id).unwrap();

            if let Some(canvas_material_instance) = my_canvas_mats.get_mut(&canvas_material.0) {
                plot.compute_zeros();

                if mouse_button_input.just_pressed(MouseButton::Middle) {
                    // println!("target_toggle");
                    plot.target_toggle = !plot.target_toggle;
                }

                if canvas_material_instance.within_rect(cursor.position) {
                    plot.target_position = plot.world_to_plot(cursor.position);
                    canvas_material_instance.mouse_pos = plot.to_local(plot.target_position) + plot.canvas_position;

                    update_target_labels_event.send(UpdateTargetLabelEvent {
                        plot_id: plot.id, // Assuming `plot.id` is a unique identifier
                        canvas_entity,
                        canvas_material_handle: canvas_material.clone(),
                    });
                }
            }

            // if let Some(canvas_material) = my_canvas_mats.get_mut(canvas_material_handle) {
            //     plot.compute_zeros();

            //     //
            //     if mouse_button_input.just_pressed(MouseButton::Middle) {
            //         plot.target_toggle = !plot.target_toggle;
            //     }

            //     if canvas_material.within_rect(cursor.position) {
            //         //
            //         plot.target_position = plot.world_to_plot(cursor.position);
            //         canvas_material.mouse_pos = plot.to_local(plot.target_position) + plot.canvas_position;

            //         update_target_labels_event.send(UpdateTargetLabelEvent {
            //             plot_handle: plot_handle.clone(),
            //             canvas_entity,
            //             canvas_material_handle: canvas_material_handle.clone(),
            //         });
            //     }
            // }
        }
    }
}

pub(crate) fn change_plot(
    mut commands: Commands,
    mut my_plots: ResMut<PlotMap>,
    canvas_query: Query<(
        Entity,
        &CanvasParams,
        &PlotIdComponent,
        &mut MeshMaterial2d<CanvasMaterial>,
    )>,

    keyboard_input: Res<ButtonInput<KeyCode>>,
    cursor: Res<Cursor>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,

    mut release_all_event: EventWriter<ReleaseAllEvent>,
    mut update_plot_labels_event: EventWriter<UpdatePlotLabelsEvent>,
    mut update_target_labels_event: EventWriter<UpdateTargetLabelEvent>,
    window: Single<Entity, With<Window>>,
) {
    for (canvas_entity, graph_sprite, plot_id, canvas_material_handle) in canvas_query.iter() {
        //
        if let Some(plot) = my_plots.get_mut(&plot_id.0) {
            plot.plot_coord_mouse_pos = plot.world_to_plot(cursor.position);

            graph_sprite.hovered_on_plot_edges(cursor.position, *window, &mut commands);

            for event in mouse_motion_events.read() {
                //
                // When pressing P and moving the mouse, the tick period changes
                if keyboard_input.pressed(KeyCode::KeyP) {
                    plot.tick_period.x *= 1.0 + (event.delta.x) / 1000.0;
                    plot.tick_period.y *= 1.0 + (event.delta.y) / 1000.0;

                    plot.clamp_tick_period();

                    update_plot_labels_event.send(UpdatePlotLabelsEvent {
                        plot_id: plot_id.0,
                        canvas_entity,
                    });
                    update_target_labels_event.send(UpdateTargetLabelEvent {
                        plot_id: plot_id.0,
                        canvas_entity,
                        canvas_material_handle: canvas_material_handle.clone(),
                    });
                }
            }

            for wheel_event in mouse_wheel_events.read() {
                commands.entity(canvas_entity).insert(ZoomAxes {
                    wheel_dir: wheel_event.y,
                    mouse_pos: cursor.position,
                });

                update_plot_labels_event.send(UpdatePlotLabelsEvent {
                    plot_id: plot_id.0,
                    canvas_entity,
                });

                update_target_labels_event.send(UpdateTargetLabelEvent {
                    plot_id: plot_id.0,
                    canvas_entity,
                    canvas_material_handle: canvas_material_handle.clone(),
                });
            }

            if mouse_button_input.just_pressed(MouseButton::Left) {
                //
                if graph_sprite.within_rect(cursor.position) {
                    commands.entity(canvas_entity).insert(MoveAxes);
                }

                graph_sprite.clicked_on_plot_corner(cursor.position, &mut commands, canvas_entity);
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        release_all_event.send(ReleaseAllEvent);
    }
}

pub(crate) fn release_all(
    mut commands: Commands,
    mut query2: Query<(Entity, &mut CanvasParams), With<ResizePlotWindow>>,
    query3: Query<Entity, With<MoveAxes>>,
    mut release_all_event: EventReader<ReleaseAllEvent>,
    mut window_entity: Single<Entity, With<Window>>,
) {
    for _ in release_all_event.read() {
        for (entity, mut graph_sprite) in query2.iter_mut() {
            commands.entity(entity).remove::<ResizePlotWindow>();
            graph_sprite.previous_scale = graph_sprite.scale;
        }
        for entity in query3.iter() {
            commands.entity(entity).remove::<MoveAxes>();
        }
        // let window = window.get_primary_mut().unwrap();
        // window.set_cursor_icon(CursorIcon::Default);
        commands
            .entity(*window_entity)
            .insert(CursorIcon::System(SystemCursorIcon::Default));
    }
}

pub(crate) fn adjust_graph_size(
    mut canvas_query: Query<
        (
            Entity,
            &mut CanvasParams,
            &PlotIdComponent,
            &MeshMaterial2d<CanvasMaterial>,
            &ResizePlotWindow,
            &mut Transform,
        ),
        Without<Locked>,
    >,
    mut my_canvas_mat: ResMut<Assets<CanvasMaterial>>,
    mut plots: ResMut<PlotMap>,
    cursor: Res<Cursor>,
    mut update_labels_event: EventWriter<UpdatePlotLabelsEvent>,
) {
    for (canvas_entity, mut graph_sprite, plot_id_comp, mat_handle, resize_corner, mut transform) in &mut canvas_query {
        //

        if let Some(canvas_material) = my_canvas_mat.get_mut(mat_handle) {
            let delta = cursor.pos_relative_to_click;
            let mut new_transform_scale;

            match resize_corner.corner {
                Corner::TopLeft => {
                    new_transform_scale = Vec3::new(
                        resize_corner.previous_scale.x - delta.x / 350.0,
                        resize_corner.previous_scale.y + delta.y / 350.0,
                        1.0,
                    );
                }
                Corner::TopRight => {
                    let delta = cursor.pos_relative_to_click;

                    new_transform_scale = Vec3::new(
                        resize_corner.previous_scale.x + delta.x / 350.0,
                        resize_corner.previous_scale.y + delta.y / 350.0,
                        1.0,
                    );
                }

                Corner::BottomRight => {
                    let delta = cursor.pos_relative_to_click;

                    new_transform_scale = Vec3::new(
                        resize_corner.previous_scale.x + delta.x / 350.0,
                        resize_corner.previous_scale.y - delta.y / 350.0,
                        1.0,
                    );
                }
                Corner::BottomLeft => {
                    let delta = cursor.pos_relative_to_click;

                    new_transform_scale = Vec3::new(
                        resize_corner.previous_scale.x - delta.x / 350.0,
                        resize_corner.previous_scale.y - delta.y / 350.0,
                        1.0,
                    );
                }
            }

            new_transform_scale.x = new_transform_scale.x.clamp(0.1, 10.0);
            new_transform_scale.y = new_transform_scale.y.clamp(0.1, 10.0);

            transform.scale = new_transform_scale;

            graph_sprite.scale = transform.scale.truncate();

            canvas_material.size = graph_sprite.scale * graph_sprite.original_size;

            update_labels_event.send(UpdatePlotLabelsEvent {
                plot_id: plot_id_comp.0,
                canvas_entity,
            });

            if let Some(plot) = plots.get_mut(&plot_id_comp.0) {
                plot.canvas_size = canvas_material.size;
            }
        }
    }
}

pub(crate) fn adjust_graph_axes(
    mut commands: Commands,
    mut query: ParamSet<(
        Query<
            (Entity, &CanvasParams, &PlotIdComponent, &MeshMaterial2d<CanvasMaterial>),
            (With<MoveAxes>, Without<Locked>),
        >,
        Query<
            (
                Entity,
                &CanvasParams,
                &PlotIdComponent,
                &MeshMaterial2d<CanvasMaterial>,
                &ZoomAxes,
            ),
            Without<Locked>,
        >,
    )>,

    mut plots: ResMut<PlotMap>,

    mut mouse_motion_events: EventReader<MouseMotion>,
    mut update_plot_labels_event: EventWriter<UpdatePlotLabelsEvent>,
    mut change_canvas_material_event: EventWriter<RespawnAllEvent>,
    mut update_target_labels_event: EventWriter<UpdateTargetLabelEvent>,
    mut canvas_materials: ResMut<Assets<CanvasMaterial>>,
    // mut spawn_beziercurve_event: EventWriter<SpawnBezierCurveEvent>,
) {
    let delta_pixels_vec = mouse_motion_events.read().map(|e| e.delta).collect::<Vec<Vec2>>();
    let delta_pixels = delta_pixels_vec.iter().fold(Vec2::ZERO, |acc, x| acc + *x);

    // when axes have been moved, respawn the data
    if delta_pixels != Vec2::ZERO {
        for (canvas_entity, _graph_sprite, plot_id_comp, material_handle) in query.p0().iter_mut() {
            if let Some(plot) = plots.get_mut(&plot_id_comp.0) {
                plot.move_axes(delta_pixels);

                // println!("bounds: {:?}", plot.bounds);

                plot.compute_zeros();

                // if let Some(canvas_mat) = canvas_materials.get_mut(&material_handle.0) {
                //     canvas_mat.update_all(&plot);
                // }

                update_plot_labels_event.send(UpdatePlotLabelsEvent {
                    plot_id: plot_id_comp.0,
                    canvas_entity,
                });

                update_target_labels_event.send(UpdateTargetLabelEvent {
                    plot_id: plot_id_comp.0,
                    canvas_entity,
                    canvas_material_handle: material_handle.clone(),
                });

                change_canvas_material_event.send(RespawnAllEvent {
                    plot_id: plot_id_comp.0,
                    // canvas_material_handle: material_handle.clone(),
                });

                // plot.data.bezier_groups.iter().enumerate().for_each(|(k, _)| {
                //     let bezier_curve = plot.data.bezier_groups.get(k).unwrap();
                //     // So as to not spawn twice when show_animation is turned on
                //     if !bezier_curve.show_animation {
                //         spawn_beziercurve_event.send(SpawnBezierCurveEvent {
                //             group_number: k,
                //             plot_handle: plot_handle.clone(),
                //         })
                //     }
                // });
            }
        }
    }

    // zoom axes using mouse scroll
    for (canvas_entity, _graph_sprite, plot_id_comp, material_handle, zoom_info) in query.p1().iter_mut() {
        //
        if let Some(plot) = plots.get_mut(&plot_id_comp.0) {
            //
            plot.zoom_axes(zoom_info.wheel_dir);

            plot.clamp_tick_period();

            plot.compute_zeros();

            update_plot_labels_event.send(UpdatePlotLabelsEvent {
                plot_id: plot_id_comp.0,
                canvas_entity,
            });

            update_target_labels_event.send(UpdateTargetLabelEvent {
                plot_id: plot_id_comp.0,
                canvas_entity,
                canvas_material_handle: material_handle.clone(),
            });

            // plot.data.bezier_groups.iter().enumerate().for_each(|(k, _)| {
            //     let bezier_curve = plot.data.bezier_groups.get(k).unwrap();
            //     // So as to not spawn twice when show_animation is turned on
            //     if !bezier_curve.show_animation {
            //         spawn_beziercurve_event.send(SpawnBezierCurveEvent {
            //             group_number: k,
            //             plot_handle: plot_id_comp.clone(),
            //         })
            //     }
            // });
        }

        commands.entity(canvas_entity).remove::<ZoomAxes>();

        change_canvas_material_event.send(RespawnAllEvent {
            plot_id: plot_id_comp.0,
            // canvas_material_handle: material_handle.clone(),
        });
    }
}
