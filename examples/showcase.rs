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
        .add_system(exit)
        .run();
}

pub fn change_bezier_uni(
    mut query: Query<&mut BezierCurveUniform>,
    mouse_position: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for mut custom_uni in query.iter_mut() {
        let mouse_pos = mouse_position.position;

        if mouse_button_input.pressed(MouseButton::Left) {
            custom_uni.left = mouse_pos.x / 100.0;
            // println!("left: {}, right: {}", custom_uni.left, custom_uni.mech);
            // println!("BEZ left: {}", custom_uni.left,);
        } else if mouse_button_input.pressed(MouseButton::Right) {
            custom_uni.mech = mouse_pos.x / 100.0;
            // custom_uni.ya.x = mouse_pos.x / 100.0;
            // custom_uni.ya.y = mouse_pos.y / 100.0;
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

            // segment_uni.ya.x = mouse_pos.x / 100.0;
            // segment_uni.ya.y = mouse_pos.y / 100.0;
            // println!(
            //     "Seg Size: {}, Seg Hole: {}",
            //     segment_uni.segment_size, segment_uni.hole_size
            // );
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
// 2) fix the target
// 3) clean up the code
// 4) generate the docs
// 1) Modify the transform instead of spawning brand new entities
// this way, the uniform will stay the same
//
// 2) Add a way to change the color of the plot.
// Copilot, do it for me!

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

    let rr = 0.100;
    let lower_bound = Vec2::new(-2.0, -1.0) * rr;
    let upper_bound = Vec2::new(3.0, 4.0) * rr;

    plot.set_bounds(lower_bound, upper_bound);

    let xs_linspace = linspace(-1.0, 1.0, 50);
    let xs = xs_linspace.into_iter().collect::<Vec<f32>>();

    let ys = xs
        .iter()
        .map(|x| Vec2::new(*x, f(*x)))
        .collect::<Vec<Vec2>>();

    plot.plotopt(
        ys,
        vec![
            Opt::Size(0.75),
            Opt::Color(colors.get(&PlotColor::Gray).unwrap()[2]),
            Opt::Mech(true),
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
            Opt::Color(colors.get(&PlotColor::Green).unwrap()[1]),
            Opt::Mech(false),
            Opt::MarkerStyle(MarkerStyle::Circle),
            Opt::MarkerSize(0.5),
            Opt::Contour(true),
            Opt::MarkerColor(colors.get(&PlotColor::Green).unwrap()[4]),
            Opt::MarkerInnerPointColor(colors.get(&PlotColor::Green).unwrap()[4]),
        ],
    );

    plot.plot_analytical(easing_func);

    // quadratic curve
    let quad_style = Opt::LineStyle(LineStyle::Solid);
    // let quad_color = Opt::Color(Color::rgb(0.1, 0.5, 0.0));
    let quad_color = Opt::Color(colors.get(&PlotColor::Orange).unwrap()[5]);
    let quad_size = Opt::Size(0.5);
    let quad_options = vec![quad_style, quad_color, quad_size];
    plot.plotopt_analytical(|x: f32| x * x, quad_options);

    plot.plotopt_analytical(
        |x: f32| x * x + 1.5,
        vec![
            Opt::Size(2.0),
            Opt::Color(colors.get(&PlotColor::LightPink).unwrap()[1]),
            Opt::Mech(true),
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
    let freq = 40.0;
    let y = (x * freq).sin() / 2.0;
    return y;
}

pub fn easing_func(x: f32) -> f32 {
    let start_point: Vec2 = Vec2::ZERO;
    let end_point: Vec2 = Vec2::splat(1.0);
    let y_min = start_point.y;
    let y_max = end_point.y;
    let mut expo: f32 = 5.0;

    let mut xp = (x - start_point.x) / (end_point.x - start_point.x);
    let mut f = y_max - (1.0 - xp).powf(expo) * (y_max - y_min);

    // switch start point and end point if the exponent is under 1
    if expo < 1.0 {
        expo = 1.0 / expo;
        xp = (x - end_point.x) / (start_point.x - end_point.x);
        f = y_min + (1.0 - xp).powf(expo) * (y_max - y_min);
    }

    return f;
}

// a system that exist the program upon pressing q or escape
fn exit(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) || keyboard_input.just_pressed(KeyCode::Q) {
        std::process::exit(0);
    }
}
