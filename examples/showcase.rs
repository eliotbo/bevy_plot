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

    let xs_linspace = linspace(-0.1, 0.9, 50);
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

// sine waves
pub fn f(x: f32) -> f32 {
    let freq = 40.0;
    let x2 = x - 0.45;
    let y = (x2 * freq).sin() / x2 / 25.0 + 3.0;
    return y;
}

pub fn f2(x: f32) -> f32 {
    let freq = 8.0;
    let y = (x * freq).sin() / 2.0;
    return y;
}
