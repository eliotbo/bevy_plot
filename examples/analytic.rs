use bevy::prelude::*;
use bevy_plot::*;

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

    // note that a closure would work as well
    plot.plot_analytical(easing_function);

    let plot_handle = plots.add(plot.clone());
    commands.spawn().insert(plot_handle);
}

// The function is not animated, so we don't use the time variable t.
pub fn easing_function(x: f32, _t: f32) -> f32 {
    let start_point: Vec2 = Vec2::ZERO;
    let end_point: Vec2 = Vec2::splat(1.0);
    let y_min = start_point.y;
    let y_max = end_point.y;
    let expo: f32 = 5.0;

    let xp = (x - start_point.x) / (end_point.x - start_point.x);
    let f = y_max - (1.0 - xp).powf(expo) * (y_max - y_min);

    return f;
}
