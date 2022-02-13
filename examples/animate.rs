use bevy::prelude::*;

use bevy_plot::*;

use std::collections::HashMap;

// BUG: Lag comes and goes depending on the zoom value.

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlotPlugin)
        .add_startup_system(setup)
        .add_system(exit)
        .run();
}

fn setup(
    mut commands: Commands,
    colors_res: Res<HashMap<PlotColor, Vec<Color>>>,
    mut plots: ResMut<Assets<Plot>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let colors = colors_res.as_ref();

    let mut plot = Plot::default();
    plot.canvas_size = Vec2::new(790.0, 590.0);
    // plot.canvas_size = Vec2::new(802.0, 602.0) / (1.0 - plot.outer_border);

    plot.show_axes = false;
    plot.show_grid = false;
    plot.hide_contour = true;
    plot.hide_tick_labels = true;

    // transparent background
    plot.background_color1 = Color::rgba(0.0, 0.0, 0.0, 0.0);
    plot.background_color2 = Color::rgba(0.0, 0.0, 0.0, 0.0);

    // number of sample points taken to plot the given function
    plot.bezier_num_points = 75;

    // extremeties of the graph axes
    let lower_bound = Vec2::new(-1.5, -1.0);
    let upper_bound = Vec2::new(3.0, 10.0);
    plot.set_bounds(lower_bound, upper_bound);

    // quadratic curve
    plot.plotopt_func(
        |x: f32, t: f32| 2.0 + x * x * (1.5 + (t * 2.0).sin()),
        vec![
            Opt::LineStyle(LineStyle::Solid),
            Opt::Color(colors.get(&PlotColor::Orange).unwrap()[5]),
            Opt::Size(0.5),
            Opt::Animate(true),
        ],
    );

    // sine wave
    plot.plotopt_func(
        f3,
        vec![
            Opt::Size(2.0),
            Opt::Color(colors.get(&PlotColor::LightPink).unwrap()[1]),
            Opt::Mech(true),
            Opt::Animate(true),
        ],
    );

    // easing function (typically used in animations)
    plot.plotopt_func(easing_func, vec![Opt::Animate(true)]);

    // Dummy entity that will be deleted as soon as its purpose has been served.
    // Required for easy access to the plot handle when spawning the graph,
    // instead of building a plot handle from a weak handle which can
    // lead to complications.
    let plot_handle = plots.add(plot.clone());
    commands.spawn().insert(plot_handle);
}

pub fn f3(x: f32, t: f32) -> f32 {
    let freq = 5.0;
    let y = (x * freq + t * 2.0).sin() / 2.0 + 5.0;
    return y;
}

pub fn easing_func(x: f32, t: f32) -> f32 {
    let start_point: Vec2 = Vec2::ZERO;
    let end_point: Vec2 = Vec2::splat(1.0);
    let y_min = start_point.y;
    let y_max = end_point.y;

    // visual bug appears when the exponent is close to zero
    let expo: f32 = 4.1 + (t * 2.0).sin() * 3.0;

    let xp = (x - start_point.x) / (end_point.x - start_point.x);
    let mut sign = (1.0 - xp).signum();
    if sign == 0.0 {
        sign = 1.0;
    }
    let f = y_max - sign * (1.0 - xp).abs().powf(expo) * (y_max - y_min);

    return f;
}

// a system that exist the program upon pressing q or escape
fn exit(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) || keyboard_input.just_pressed(KeyCode::Q) {
        std::process::exit(0);
    }
}
