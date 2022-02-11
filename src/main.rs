// use bevy::prelude::*;

// mod canvas;
// // use canvas::*;

// pub mod markers;
// pub use markers::*;

// pub mod segments;
// pub use segments::*;

// mod inputs;
// mod util;
// // use util::*;

// mod bezier;
// // use bezier::*;

// mod plot;
// use plot::*;
// // use inputs::*;

// use itertools_num::linspace;
// use std::collections::HashMap;

// fn main() {
//     App::new()
//         .insert_resource(WindowDescriptor {
//             title: "I am a window!".to_string(),
//             width: 1000.,
//             height: 1300.,
//             vsync: true,
//             ..Default::default()
//         })
//         .add_plugins(DefaultPlugins)
//         .add_plugin(PlotPlugin)
//         .add_startup_system(setup)
//         .add_system(exit)
//         .run();
// }

// fn setup(
//     mut commands: Commands,
//     colors_res: Res<HashMap<PlotColor, Vec<Color>>>,
//     mut plots: ResMut<Assets<Plot>>,
// ) {
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());

//     let colors = colors_res.as_ref();

//     let mut plot = Plot::default();

//     plot.canvas_position = Vec2::new(-100.0, -55.0);
//     plot.bezier_num_points = 75;

//     let xs_linspace = linspace(-1.0, 1.0, 50);
//     let xs = xs_linspace.into_iter().collect::<Vec<f32>>();

//     let ys = xs
//         .iter()
//         .map(|x| Vec2::new(*x, f(*x)))
//         .collect::<Vec<Vec2>>();

//     // let marker_style = Opt::MarkerStyle(MarkerStyle::Cross);

//     plot.plotopt(
//         ys,
//         vec![
//             Opt::Size(0.75),
//             Opt::Color(colors.get(&PlotColor::Gray).unwrap()[2]),
//             Opt::Mech(true),
//         ],
//     );

//     let ys = xs
//         .iter()
//         .map(|x| Vec2::new(*x, f2(*x) + 1.3))
//         .collect::<Vec<Vec2>>();

//     plot.plotopt(
//         ys,
//         vec![
//             Opt::Size(0.75),
//             Opt::Color(colors.get(&PlotColor::Green).unwrap()[1]),
//             Opt::Mech(false),
//             Opt::MarkerStyle(MarkerStyle::Circle),
//             Opt::MarkerSize(0.5),
//             Opt::Contour(true),
//             Opt::MarkerColor(colors.get(&PlotColor::Green).unwrap()[4]),
//             Opt::MarkerInnerPointColor(colors.get(&PlotColor::Green).unwrap()[4]),
//         ],
//     );

//     plot.plot_analytical(easing_func);

//     // quadratic curve
//     let quad_style = Opt::LineStyle(LineStyle::Solid);
//     // let quad_color = Opt::Color(Color::rgb(0.1, 0.5, 0.0));
//     let quad_color = Opt::Color(colors.get(&PlotColor::Orange).unwrap()[5]);
//     let quad_size = Opt::Size(0.5);
//     let quad_options = vec![quad_style, quad_color, quad_size];
//     plot.plotopt_analytical(|x: f32| x * x, quad_options);

//     plot.plotopt_analytical(
//         |x: f32| x * x + 1.5,
//         vec![
//             Opt::Size(2.0),
//             Opt::Color(colors.get(&PlotColor::LightPink).unwrap()[1]),
//             Opt::Mech(true),
//         ],
//     );

//     let plot_handle = plots.add(plot.clone());
//     commands.spawn().insert(plot_handle);
// }

// // example function to be plotted
// pub fn f(x: f32) -> f32 {
//     let freq = 4.0;
//     let y = (x * freq).sin() / 2.0;
//     return y;
// }

// // example function to be plotted
// pub fn f2(x: f32) -> f32 {
//     let freq = 40.0;
//     let y = (x * freq).sin() / 2.0;
//     return y;
// }

// pub fn custom_sin(x: f32) -> f32 {
//     (2.0 * 2.0 * 3.1416 * (x)).sin() * 0.5 + 0.2
// }

// pub fn custom_sin2(x: f32) -> f32 {
//     (2.0 * 2.0 * 3.1416 * (x)).sin() * 0.5 + 0.4
// }

// pub fn easing_func(x: f32) -> f32 {
//     let start_point: Vec2 = Vec2::ZERO;
//     let end_point: Vec2 = Vec2::splat(1.0);
//     let y_min = start_point.y;
//     let y_max = end_point.y;
//     let mut expo: f32 = 5.0;

//     let mut xp = (x - start_point.x) / (end_point.x - start_point.x);
//     let mut f = y_max - (1.0 - xp).powf(expo) * (y_max - y_min);

//     // switch start point and end point if the exponent is under 1
//     if expo < 1.0 {
//         expo = 1.0 / expo;
//         xp = (x - end_point.x) / (start_point.x - end_point.x);
//         f = y_min + (1.0 - xp).powf(expo) * (y_max - y_min);
//     }

//     return f;
// }

// // a system that exist the program upon pressing q or escape
// fn exit(keyboard_input: Res<Input<KeyCode>>) {
//     if keyboard_input.just_pressed(KeyCode::Escape) || keyboard_input.just_pressed(KeyCode::Q) {
//         std::process::exit(0);
//     }
// }
