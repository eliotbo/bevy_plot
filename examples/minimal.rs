use bevy::prelude::*;
use bevy_plot::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlotPlugin))
        .add_systems(Startup, setup)
        .run();
}

// If no fonts are loaded and put into the TickLabelFont resource,
// the canvas will not include the tick labels. See the "markers" example
// for an instance of loading a font.
fn setup(mut commands: Commands, mut plots: ResMut<PlotMap>) {
    commands.spawn(Camera2d::default());

    let mut plot = Plot::default();

    let xs = (0..30).map(|i| i as f32 / 30.0).collect::<Vec<f32>>();

    let ys = xs.iter().map(|x| Vec2::new(*x, 0.5 * x)).collect::<Vec<Vec2>>();

    plot.plot(ys);

    plots.add(plot.clone());
    // commands.spawn().insert(plot_handle);
}
