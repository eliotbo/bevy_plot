use bevy::prelude::*;
use bevy_plot::*;

fn main() {
    App::new()
        // .insert_resource(WindowDescriptor {
        //     width: 800.,
        //     height: 600.,
        //     ..Default::default()
        // })
        .add_plugins((
            PlotPlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    name: Some("bevy.app".into()),
                    resolution: (800., 600.).into(),

                    ..default()
                }),
                ..default()
            }),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut plots: ResMut<PlotMap>,
    // asset_server: Res<AssetServer>,
    // mut maybe_font: ResMut<TickLabelFont>,
) {
    commands.spawn(Camera2d::default());

    // let font: Handle<Font> = asset_server.load("fonts/Roboto-Bold.ttf");
    // maybe_font.maybe_font = Some(font);

    let mut plot = Plot::default();
    plot.canvas_size = Vec2::new(790.0, 590.0);

    plot.show_target = true;
    plot.show_grid = true;
    plot.hide_contour = true;
    // plot.hide_tick_labels = true;

    // // transparent background
    // plot.background_color1 = Color::rgba(0.0, 0.0, 0.0, 0.0);
    // plot.background_color2 = Color::rgba(0.0, 0.0, 0.0, 0.0);

    // extremeties of the graph axes
    let lower_bound = Vec2::new(-1.5, -1.0);
    let upper_bound = Vec2::new(3.0, 10.0);
    plot.set_bounds(lower_bound, upper_bound);

    // note that a closure would work as well
    plot.plot_func(easing_func);

    plots.add(plot.clone());
}

// The function is not animated, so we don't use the time variable t.
pub fn easing_func(x: f32, _t: f32) -> f32 {
    let start_point: Vec2 = Vec2::ZERO;
    let end_point: Vec2 = Vec2::splat(1.0);
    let y_min = start_point.y;
    let y_max = end_point.y;

    // visual bug appears when the exponent is close to zero
    let expo: f32 = 7.1;

    let xp = (x - start_point.x) / (end_point.x - start_point.x);
    let mut sign = (1.0 - xp).signum();
    if sign == 0.0 {
        sign = 1.0;
    }
    let f = y_max - sign * (1.0 - xp).abs().powf(expo) * (y_max - y_min);

    return f;
}
