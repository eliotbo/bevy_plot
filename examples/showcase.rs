use bevy::prelude::*;

use bevy_plot::*;

use itertools_num::linspace;
use std::collections::HashMap;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1000.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlotPlugin)
        .add_startup_system(setup)
        .add_system(change_segment_uni)
        .add_system(change_marker_uni)
        .add_system(change_bezier_uni)
        .run();
}

use bevy_plot::UpdateBezierShaderEvent;

pub fn change_bezier_uni(
    // mut query: Query<&mut BezierCurveUniform>,
    mut plots: ResMut<Assets<Plot>>,
    query: Query<(Entity, &Handle<Plot>, &BezierCurveNumber), With<BezierMesh2d>>,
    mouse_position: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut event: EventWriter<UpdateBezierShaderEvent>,
) {
    // for mut custom_uni in query.iter_mut() {
    for (entity, plot_handle, curve_number) in query.iter() {
        let plot = plots.get_mut(plot_handle).unwrap();

        let mouse_pos = mouse_position.position;

        if mouse_button_input.pressed(MouseButton::Left) {
            plot.bezier_dummy = mouse_pos.x / 100.0;
        } else if mouse_button_input.pressed(MouseButton::Right) {
            if let Some(mut bezier_data) = plot.data.bezier_groups.get_mut(curve_number.0) {
                // bezier_data.mech = if mouse_pos.x > 0.0 { true } else { false };
                bezier_data.size = mouse_pos.x / 100.0;

                // So as to not send the event twice is show_animation is set to true
                if !bezier_data.show_animation {
                    event.send(UpdateBezierShaderEvent {
                        plot_handle: plot_handle.clone(),
                        entity,
                        group_number: curve_number.0,
                    });
                }
            }
        }
    }
}

pub fn change_segment_uni(
    mut query: Query<&mut SegmentUniform>,
    mouse_position: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for mut segment_uni in query.iter_mut() {
        let mouse_pos = mouse_position.position;

        if mouse_button_input.pressed(MouseButton::Left) {
            segment_uni.hole_size = mouse_pos.x / 100.0;
            // println!("left: {}, right: {}", segment_uni.left, segment_uni.mech);
        } else if mouse_button_input.pressed(MouseButton::Right) {
            segment_uni.segment_size = (mouse_pos.x / 100.0).clamp(0.2, 3.0);

            if segment_uni.segment_size < 2.0 {
                segment_uni.mech = 0.0;
            } else {
                segment_uni.mech = 1.0;
            }
        }
    }
}

pub fn change_marker_uni(
    mut query: Query<&mut MarkerUniform>,
    mouse_position: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for mut custom_uni in query.iter_mut() {
        let mouse_pos = mouse_position.position;

        if mouse_button_input.pressed(MouseButton::Right) {
            custom_uni.marker_size = mouse_pos.x / 100.0;
            // println!("{:?}", custom_uni.marker_size);
            // println!("{}", custom_uni.ya.z);
        }
        // else if mouse_button_input.pressed(MouseButton::Right) {
        //     custom_uni.ya.x = mouse_pos.x / 100.0;
        //     custom_uni.ya.y = mouse_pos.y / 100.0;
        // }
        // println!("{:?}", custom_uni.ya);
    }
}

// TODO:
// 2) Area under the curve
// 3) clean up the code
// 4) generate the docs
// 5) Automatically color curve, segments and markers with palette

fn setup(
    mut commands: Commands,
    colors_res: Res<HashMap<PlotColor, Vec<Color>>>,
    mut plots: ResMut<Assets<Plot>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let colors = colors_res.as_ref();

    let mut plot = Plot::default();

    plot.canvas_size = Vec2::new(777.0, 555.0);
    plot.canvas_position = Vec2::new(-77.0, 0.0);
    plot.bezier_num_points = 75;
    plot.hide_half_ticks = true;
    plot.significant_digits = 3;
    plot.show_target = true;
    plot.tick_label_color = colors.get(&PlotColor::Black).unwrap()[5];

    let purple = colors.get(&PlotColor::Cream).unwrap()[1];
    let darker_purple = colors.get(&PlotColor::Cream).unwrap()[2] * 0.8;

    // transparent background
    plot.background_color1 = purple;
    plot.background_color2 = darker_purple;

    plot.target_color = colors.get(&PlotColor::Blue).unwrap()[5];
    plot.target_label_color = colors.get(&PlotColor::Black).unwrap()[1];
    plot.show_grid = false;

    let lower_bound = Vec2::new(-0.2, -0.2);
    let upper_bound = Vec2::new(1.0, 5.0);

    plot.set_bounds(lower_bound, upper_bound);

    let xs_linspace = linspace(-0.05, 0.9, 50);
    let xs = xs_linspace.into_iter().collect::<Vec<f32>>();

    let ys = xs
        .iter()
        .map(|x| Vec2::new(*x, f(*x)))
        .collect::<Vec<Vec2>>();

    plot.plotopt(
        ys,
        vec![
            Opt::Size(0.75),
            Opt::Color(colors.get(&PlotColor::Blue).unwrap()[5]),
            Opt::Mech(false),
        ],
    );

    let ys = xs
        .iter()
        .map(|x| Vec2::new(*x, f2(*x) + 1.3))
        .collect::<Vec<Vec2>>();

    plot.plotopt(
        ys,
        vec![
            Opt::Size(0.75),
            Opt::Color(colors.get(&PlotColor::Black).unwrap()[4]),
            Opt::Mech(false),
            Opt::MarkerStyle(MarkerStyle::Circle),
            Opt::MarkerSize(0.5),
            Opt::Contour(true),
            Opt::MarkerColor(colors.get(&PlotColor::Green).unwrap()[5]),
            Opt::MarkerInnerPointColor(colors.get(&PlotColor::Black).unwrap()[4]),
        ],
    );

    let plot_handle = plots.add(plot.clone());
    commands.spawn().insert(plot_handle);
}

// example function to be plotted
pub fn f(x: f32) -> f32 {
    let freq = 4.0;
    let y = (x * freq).sin() / 2.0;
    return y;
}

// example function to be plotted
pub fn f2(x: f32) -> f32 {
    let freq = 8.0;
    let y = (x * freq).sin() / 2.0;
    return y;
}
