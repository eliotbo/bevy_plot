use bevy::prelude::*;

use bevy_plot::*;

use itertools_num::linspace;
use std::collections::HashMap;

fn main() {
    App::new()
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

    let xs_linspace = linspace(-0.1, 1.1, 32);
    let xs = xs_linspace.into_iter().collect::<Vec<f32>>();

    let ys = xs
        .iter()
        .map(|x| Vec2::new(*x, f(*x)))
        .collect::<Vec<Vec2>>();

    plot.plotopt(
        ys,
        vec![
            Opt::LineStyle(LineStyle::None),
            Opt::Mech(false),
            Opt::MarkerStyle(MarkerStyle::Triangle),
            Opt::MarkerSize(2.0),
            Opt::Contour(true),
            Opt::MarkerColor(colors.get(&PlotColor::Green).unwrap()[5]),
            Opt::MarkerInnerPointColor(Color::BLACK),
        ],
    );

    let plot_handle = plots.add(plot.clone());
    commands.spawn().insert(plot_handle);
}

pub fn f(mut x: f32) -> f32 {
    let freq = 15.0;
    x = x - 0.5;
    let y = (x * freq).sin() / 4.0 * (1.2 - x.abs()) + 0.3;
    return y;
}
