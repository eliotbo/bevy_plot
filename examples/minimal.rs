use bevy::prelude::*;
use bevy_plot::*;
use itertools_num::linspace;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlotPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut plots: ResMut<Assets<Plot>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut plot = Plot::default();

    let xs_linspace = linspace(-1.0, 1.0, 150);
    let xs = xs_linspace.into_iter().collect::<Vec<f32>>();

    let ys = xs
        .iter()
        .map(|x| Vec2::new(*x, custom_sin(*x)))
        .collect::<Vec<Vec2>>();

    plot.plot(ys);

    let plot_handle = plots.add(plot.clone());
    commands.spawn().insert(plot_handle);
}

pub fn custom_sin(x: f32) -> f32 {
    (4.0 * 3.1416 * x).sin() * 0.5 / x + 0.2
}
