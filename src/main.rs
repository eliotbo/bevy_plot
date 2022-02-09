use bevy::prelude::*;

mod canvas;
use canvas::*;

pub mod markers;
pub use markers::*;

pub mod segments;
pub use segments::*;

mod inputs;
mod util;
// use util::*;

mod bezier;
// use bezier::*;

mod plot;
use plot::*;
// use inputs::*;

use itertools_num::linspace;
use std::collections::HashMap;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 1000.,
            height: 1300.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlotPlugin)
        .add_startup_system(setup)
        .add_system(exit)
        .run();
}

// a system that exist the program upon pressing q or escape
fn exit(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) || keyboard_input.just_pressed(KeyCode::Q) {
        std::process::exit(0);
    }
}
/* TODO:
// 1) NaN,
2) Line thickness, line color, line style,
3) syntax plot([x,y])
// 4) update thickness of axes + border with the zoom parameter
5) impl PlotFormat for Vec<(f32, f32)>...

*/

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

// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut spawn_graph_event: EventWriter<SpawnGraphEvent>,
    // mut materials: ResMut<Assets<CanvasMaterial>>,
    colors_res: Res<HashMap<PlotColor, Vec<Color>>>,
    mut plots: ResMut<Assets<Plot>>,
) {
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_translation(Vec3::new(00.0, 0.0, 10.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        orthographic_projection: OrthographicProjection {
            scale: 1.0,
            far: 100000.0,
            near: -100000.0,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_2d()
    });

    let colors = colors_res.as_ref();
    let mut plot = Plot::default();

    plot.compute_zeros();
    plot.position = Vec2::new(-100.0, -55.0);
    plot.bezier_num_points = 75;

    let xs_linspace = linspace(-1.0, 1.0, 50);
    let xs = xs_linspace.into_iter().collect::<Vec<f32>>();

    // // insert the marker data inside the Plot struct
    // let ys = vec![
    //     Vec2::new(2.0, 3.04),
    //     Vec2::new(1.0, 3.42),
    //     Vec2::new(0.5, 3.79),
    //     Vec2::new(0.25, 4.58),
    // ];

    let ys = xs
        .iter()
        .map(|x| Vec2::new(*x, f(*x)))
        .collect::<Vec<Vec2>>();

    // let marker_style = Opt::MarkerStyle(MarkerStyle::Cross);

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
            Opt::MarkerColor(colors.get(&PlotColor::Green).unwrap()[4]),
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

    let mut graph_sprite = GraphSprite {
        id: 111268946,
        position: plot.position,
        previous_position: plot.position,
        original_size: plot.size,
        scale: Vec2::splat(1.0),
        previous_scale: Vec2::splat(1.0),
        hover_radius: 20.0, // change to 0 to disable resize of GraphSprite
        analytical_functions: [None; 8],
        plot_handle: plot_handle.clone(),
    };

    graph_sprite.analytical_functions[2] = Some(|x| custom_sin(x));

    spawn_graph_event.send(SpawnGraphEvent {
        pos: Vec2::ZERO,
        // shader_param_handle: material_handle,
        graph_sprite,
        plot_handle: plot_handle.clone(),
    });

    // insert the analytical functions inside the Plot struct
    // plot.plot(())

    // plot.plot_analytical(|x| custom_sin(x), 1);

    // graph_sprite.make_data(&mut plot);
}

pub fn custom_sin(x: f32) -> f32 {
    (2.0 * 2.0 * 3.1416 * (x)).sin() * 0.5 + 0.2
}

pub fn custom_sin2(x: f32) -> f32 {
    (2.0 * 2.0 * 3.1416 * (x)).sin() * 0.5 + 0.4
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
