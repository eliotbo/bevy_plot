use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    // render::camera::OrthographicProjection,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::canvas::*;
use crate::inputs::*;
// use crate::markers::SpawnMarkersEvent;

// use crate::plot_canvas_plugin::ChangeCanvasMaterialEvent;

use crate::canvas::ChangeCanvasMaterialEvent;
use crate::util::*;

use crate::bezier::*;
use crate::plot::*;

fn spawn_axis_tick_labels(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    plot_entity: Entity,
    text: &str,
    font_size: f32,
    position: Vec3,
    v_align: VerticalAlign,
    h_align: HorizontalAlign,
) {
    let font_color = Color::BLACK;
    // if text == "0.00" || text == "0.0" {
    //     font_color = Color::DARK_GRAY;
    // }
    // let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let font = asset_server.load("fonts/Roboto-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size,
        color: font_color,
    };
    let text_alignment = TextAlignment {
        vertical: v_align,
        horizontal: h_align,
    };

    let label_entity = commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(text, text_style.clone(), text_alignment),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .insert(PlotLabel)
        .id();

    commands.entity(plot_entity).push_children(&[label_entity]);
}

pub fn update_plot_labels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut materials: ResMut<Assets<CanvasMaterial>>,
    mut plots: ResMut<Assets<Plot>>,
    mut update_plot_labels_event: EventReader<UpdatePlotLabelsEvent>,
    plot_label_query: Query<Entity, With<PlotLabel>>,
    mut plot_sprite_query: Query<&mut GraphSprite>,
) {
    // If there is a stack of UpdatePlotLabelsEvent, only read the first one.
    if let Some(event) = update_plot_labels_event.iter().next() {
        for entity in plot_label_query.iter() {
            commands.entity(entity).despawn();
        }
        let graph_sprite = plot_sprite_query.get_mut(event.plot_entity).unwrap();

        let size = graph_sprite.original_size;

        let plot_handle = event.plot_handle.clone();
        let plot_entity = event.plot_entity;
        // if let Some(plot) = materials.get_mut(plot_handle.clone()) {
        if let Some(plot) = plots.get_mut(plot_handle.clone()) {
            let font_size = 16.0;
            let graph_y = size.y / (1. + plot.outer_border.y);
            let graph_x = size.x / (1. + plot.outer_border.x);

            let x_edge = size.x / (1. + plot.outer_border.x) / 2.0;
            let y_edge = size.y / (1. + plot.outer_border.y) / 2.0;

            let x_range = plot.bounds.up.x - plot.bounds.lo.x;
            let y_range = plot.bounds.up.y - plot.bounds.lo.y;

            let z = 0.0001;

            let numerical_precision = 2;

            ///////////////////////////// x_axis labels  /////////////////////////////
            {
                // distance from center for
                let center_dist_y = -graph_y / 2.0 + font_size * 1.0;

                // iterate
                let iter_x = x_edge * 2.0 / x_range;

                // integer corresponding to lowest x tick
                let bottom_x = (plot.bounds.lo.x / plot.tick_period.x).abs().floor() as i64
                    * (plot.bounds.lo.x).signum() as i64;

                // integer corresponding to highest x tick
                let top_x = (plot.bounds.up.x / plot.tick_period.x).abs().floor() as i64
                    * (plot.bounds.up.x).signum() as i64;

                for i in bottom_x..(top_x + 1) {
                    // let j = i;
                    let x_str =
                        format!("{:.1$}", i as f32 * plot.tick_period.x, numerical_precision);

                    // leftmost position on the x axis
                    let x0 = x_edge * (-1.0 - plot.bounds.lo.x * 2.0 / x_range);

                    // iterator for each label
                    let x_pos = iter_x * i as f32 * plot.tick_period.x;

                    let font_offset_x = -font_size * 0.2;
                    if (x0 + x_pos + font_offset_x + graph_x / 2.0) > font_size * 2.0
                        && (x0 + x_pos + font_offset_x - graph_x / 2.0) < -font_size * 0.0
                    {
                        spawn_axis_tick_labels(
                            &mut commands,
                            &asset_server,
                            plot_entity,
                            &x_str,
                            font_size,
                            Vec2::new(x0 + x_pos + font_offset_x, center_dist_y).extend(z),
                            VerticalAlign::Top,
                            HorizontalAlign::Right,
                        );
                    }
                }
            }

            ////////////////////////////////// y_axis labels //////////////////////////////////
            {
                // distance from center for
                let center_dist_x = -graph_x / 2.0 + font_size * 0.2;

                // iterate
                let iter_y = y_edge * 2.0 / y_range;

                // integer corresponding to lowest y tick
                let bottom_y = (plot.bounds.lo.y / plot.tick_period.y).abs().floor() as i64
                    * (plot.bounds.lo.y).signum() as i64;

                // integer corresponding to highest y tick
                let top_y = (plot.bounds.up.y / plot.tick_period.y).abs().floor() as i64
                    * (plot.bounds.up.y).signum() as i64;

                for i in bottom_y..top_y + 1 {
                    let y_str =
                        format!("{:.1$}", i as f32 * plot.tick_period.y, numerical_precision);

                    // leftmost position on the x axis
                    let y0 = y_edge * (-1.0 - plot.bounds.lo.y * 2.0 / y_range);

                    // iterator for each label
                    let y_pos = iter_y * i as f32 * plot.tick_period.y;

                    let font_offset_y = -font_size * 0.1;

                    if (y0 + y_pos + font_offset_y + graph_y / 2.0) > font_size * 1.2
                        && (y0 + y_pos + font_offset_y - graph_y / 2.0) < -font_size * 0.0
                    {
                        spawn_axis_tick_labels(
                            &mut commands,
                            &asset_server,
                            plot_entity,
                            &y_str,
                            font_size,
                            Vec2::new(center_dist_x, y0 + y_pos + font_offset_y).extend(z),
                            VerticalAlign::Top,
                            HorizontalAlign::Left,
                        );
                    }
                }
            }
        }
    }
}

// spawns a graph a shader_param_handle
pub fn spawn_graph(
    mut commands: Commands,
    mut spawn_graph_event: EventReader<SpawnGraphEvent>,
    mut materials: ResMut<Assets<CanvasMaterial>>,
    plots: ResMut<Assets<Plot>>,
    mut meshes: ResMut<Assets<Mesh>>,
    // asset_server: Res<AssetServer>,
    mut update_labels_event: EventWriter<UpdatePlotLabelsEvent>,
    // mut spawn_markers_event: EventWriter<SpawnMarkersEvent>,
    // mut spawn_beziercurve_event: EventWriter<SpawnBezierCurveEvent>,
    mut change_canvas_material_event: EventWriter<ChangeCanvasMaterialEvent>,
) {
    for event in spawn_graph_event.iter() {
        let plot_handle = event.plot_handle.clone();
        let plot = plots.get(plot_handle.clone()).unwrap();

        let material = CanvasMaterial::new(&plot);

        let canvas_material_handle = materials.add(material);

        println!("{:?}", "SPAWNING graphshapgraph");

        // quad
        let plot_entity = commands
            .spawn()
            .insert_bundle(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::new(plot.size)))),
                material: canvas_material_handle.clone(),
                transform: Transform::from_translation(plot.position.extend(0.0)),
                ..Default::default()
            })
            .insert(event.graph_sprite.clone())
            .insert(event.plot_handle.clone())
            .id();

        update_labels_event.send(UpdatePlotLabelsEvent {
            plot_handle: plot_handle.clone(),
            plot_entity,
        });

        // TODO after tests: remove this
        change_canvas_material_event.send(ChangeCanvasMaterialEvent {
            canvas_material_handle: canvas_material_handle.clone(),
            plot_handle: plot_handle.clone(),
        });

        // // TODO after tests: remove this
        // spawn_beziercurve_event.send(SpawnBezierCurveEvent {
        //     canvas_handle: canvas_material_handle,
        //     plot_handle,
        // });
    }
}

pub fn update_mouse_target(
    // mut commands: Commands,
    mut my_canvas_mats: ResMut<Assets<CanvasMaterial>>,
    mut my_plots: ResMut<Assets<Plot>>,
    //
    // graph_sprite_query: Query<(Entity, &GraphSprite, &Handle<CanvasMaterial>)>,
    graph_sprite_query: Query<(&mut Handle<CanvasMaterial>, &Handle<Plot>)>,

    cursor: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Middle) {
        for (canvas_material_handle, plot_handle) in graph_sprite_query.iter() {
            // println!("{:?}", "CHANGING SHADER");
            // if let Some(plot) = my_canvas_mat.get_mut(plot_handle) {
            if let Some(plot) = my_plots.get_mut(plot_handle) {
                plot.compute_zeros();
                if let Some(canvas_material) = my_canvas_mats.get_mut(canvas_material_handle) {
                    plot.compute_zeros();

                    // let mouse_world = cursor.position - 0.0 * plot.position - 0.0 * plot.zero_world;

                    // let ranges = plot.bounds.up - plot.bounds.lo;

                    // let mouse_plot = mouse_world * (1.0 + plot.outer_border.y) * ranges / plot.size;

                    let mouse_plot = plot.world_to_plot(cursor.position);
                    canvas_material.mouse_pos = mouse_plot;

                    // canvas_material.mouse_pos = cursor.position;
                }
            }
        }
    }
}

pub fn change_plot(
    mut commands: Commands,
    // mut my_canvas_mats: ResMut<Assets<CanvasMaterial>>,
    mut my_plots: ResMut<Assets<Plot>>,
    //
    // graph_sprite_query: Query<(Entity, &GraphSprite, &Handle<CanvasMaterial>)>,
    graph_sprite_query: Query<(
        Entity,
        &GraphSprite,
        // &mut Handle<CanvasMaterial>,
        &Handle<Plot>,
    )>,
    // markers_query: Query<Entity, With<MarkerUniform>>,
    //
    keyboard_input: Res<Input<KeyCode>>,
    cursor: Res<Cursor>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_button_input: Res<Input<MouseButton>>,
    // window_res: Res<WindowDescriptor>,
    mut release_all_event: EventWriter<ReleaseAllEvent>,
    mut update_plot_labels_event: EventWriter<UpdatePlotLabelsEvent>,
    mut windows: ResMut<Windows>,
    // mut change_canvas_material_event: EventWriter<ChangeCanvasMaterialEvent>,
) {
    for (plot_entity, graph_sprite, plot_handle) in graph_sprite_query.iter() {
        // println!("{:?}", "CHANGING SHADER");
        // if let Some(plot) = my_canvas_mat.get_mut(plot_handle) {
        if let Some(plot) = my_plots.get_mut(plot_handle) {
            plot.relative_mouse_pos = plot.world_to_plot(cursor.position);

            graph_sprite.hovered_on_plot_edges(cursor.position, &mut windows);

            for event in mouse_motion_events.iter() {
                if keyboard_input.pressed(KeyCode::Q) {
                    plot.globals.dum1 += (event.delta.x + event.delta.y) / 100.0;
                    plot.tick_period.x *= 1.0 + (event.delta.x) / 1000.0;
                    plot.tick_period.y *= 1.0 + (event.delta.y) / 1000.0;

                    plot.clamp_tick_period();

                    update_plot_labels_event.send(UpdatePlotLabelsEvent {
                        plot_handle: plot_handle.clone(),
                        plot_entity,
                    });
                }
                if keyboard_input.pressed(KeyCode::W) {
                    plot.globals.dum2 += (event.delta.x + event.delta.y) / 100.0;
                }
            }

            for wheel_event in mouse_wheel_events.iter() {
                // println!("{:?}", wheel_event);

                commands.entity(plot_entity).insert(ZoomAxes {
                    wheel_dir: wheel_event.y,
                    mouse_pos: cursor.position,
                });

                update_plot_labels_event.send(UpdatePlotLabelsEvent {
                    plot_handle: plot_handle.clone(),
                    plot_entity,
                });
            }

            if mouse_button_input.just_pressed(MouseButton::Left) {
                //
                //
                if graph_sprite.within_rect(cursor.position) {
                    commands.entity(plot_entity).insert(MoveAxes);

                    // for marker_entity in markers_query.iter() {
                    //     commands.entity(marker_entity).insert(MoveAxes);
                    // }
                }

                graph_sprite.clicked_on_plot_corner(cursor.position, &mut commands, plot_entity);
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        release_all_event.send(ReleaseAllEvent);
    }
}

pub fn release_all(
    mut commands: Commands,
    mut query2: Query<(Entity, &mut GraphSprite), With<ResizePlotWindow>>,
    query3: Query<Entity, With<MoveAxes>>,
    mut release_all_event: EventReader<ReleaseAllEvent>,
    mut windows: ResMut<Windows>,
) {
    for _ in release_all_event.iter() {
        for (entity, mut graph_sprite) in query2.iter_mut() {
            commands.entity(entity).remove::<ResizePlotWindow>();
            graph_sprite.previous_scale = graph_sprite.scale;
        }
        for entity in query3.iter() {
            commands.entity(entity).remove::<MoveAxes>();
        }
        let window = windows.get_primary_mut().unwrap();
        window.set_cursor_icon(CursorIcon::Default);
    }
}

pub fn adjust_graph_size(
    mut graph_sprite_query: Query<
        (
            Entity,
            &mut GraphSprite,
            &Handle<Plot>,
            // &Handle<CanvasMaterial>,
            &ResizePlotWindow,
            &mut Transform,
        ),
        Without<Locked>,
    >,
    // mut plot_query: Query<(&Handle<CanvasMaterial>, &ResizePlotWindow)>,
    mut my_canvas_mat: ResMut<Assets<CanvasMaterial>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut windows: ResMut<Windows>,
    cursor: Res<Cursor>,
    mut update_labels_event: EventWriter<UpdatePlotLabelsEvent>,
    // mut change_canvas_material_event: EventWriter<ChangeCanvasMaterialEvent>,
    //
) {
    for (plot_entity, mut graph_sprite, plot_handle, resize_corner, mut transform) in
        graph_sprite_query.iter_mut()
    {
        //

        if let Some(plot) = my_canvas_mat.get_mut(plot_handle) {
            let delta = cursor.pos_relative_to_click;
            let mut new_transform_scale; // = transform.scale.clone();

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
                } // _ => {}
            }

            new_transform_scale.x = new_transform_scale.x.clamp(0.1, 10.0);
            new_transform_scale.y = new_transform_scale.y.clamp(0.1, 10.0);

            transform.scale = new_transform_scale;

            graph_sprite.scale = transform.scale.truncate();

            plot.size = graph_sprite.scale * graph_sprite.original_size;

            update_labels_event.send(UpdatePlotLabelsEvent {
                plot_handle: plot_handle.clone(),
                plot_entity,
            });
        }
    }
}

pub fn adjust_graph_axes(
    mut commands: Commands,
    mut query: QuerySet<(
        QueryState<
            (Entity, &GraphSprite, &Handle<Plot>, &Handle<CanvasMaterial>),
            (With<MoveAxes>, Without<Locked>),
        >,
        QueryState<
            (
                Entity,
                &GraphSprite,
                &Handle<Plot>,
                &Handle<CanvasMaterial>,
                &ZoomAxes,
            ),
            Without<Locked>,
        >,
    )>,
    // mut my_canvas_mat: ResMut<Assets<CanvasMaterial>>,
    mut plots: ResMut<Assets<Plot>>,
    // mut cursor: ResMut<Cursor>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut update_plot_labels_event: EventWriter<UpdatePlotLabelsEvent>,
    mut change_canvas_material_event: EventWriter<ChangeCanvasMaterialEvent>,
) {
    let delta_pixels_vec = mouse_motion_events
        .iter()
        .map(|e| e.delta)
        .collect::<Vec<Vec2>>();
    let delta_pixels = delta_pixels_vec.iter().fold(Vec2::ZERO, |acc, x| acc + *x);

    if delta_pixels != Vec2::ZERO {
        for (plot_entity, _graph_sprite, plot_handle, material_handle) in query.q0().iter_mut() {
            println!("motion: {:?}", delta_pixels);

            if let Some(plot) = plots.get_mut(plot_handle) {
                // sum up the mouse movements

                plot.move_axes(delta_pixels);

                update_plot_labels_event.send(UpdatePlotLabelsEvent {
                    plot_handle: plot_handle.clone(),
                    plot_entity,
                });

                change_canvas_material_event.send(ChangeCanvasMaterialEvent {
                    plot_handle: plot_handle.clone(),
                    canvas_material_handle: material_handle.clone(),
                });

                // }
            }
        }
    }

    // zoom axes using mouse scroll
    for (plot_entity, _graph_sprite, plot_handle, material_handle, zoom_info) in
        query.q1().iter_mut()
    {
        //
        if let Some(plot) = plots.get_mut(plot_handle) {
            //
            plot.zoom_axes(zoom_info.wheel_dir);

            plot.clamp_tick_period();

            // plot.line_params.thickness = self.size.y / self.size.x;

            update_plot_labels_event.send(UpdatePlotLabelsEvent {
                plot_handle: plot_handle.clone(),
                plot_entity,
            });
            plot.clamp_tick_period();
        }

        commands.entity(plot_entity).remove::<ZoomAxes>();

        change_canvas_material_event.send(ChangeCanvasMaterialEvent {
            plot_handle: plot_handle.clone(),
            canvas_material_handle: material_handle.clone(),
        });
    }
}
