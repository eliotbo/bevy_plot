use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy_plot::*;

use bevy_framepace::FramepacePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1000., 600.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells Wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
            // LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin::default(),
            PlotPlugin,
        ))
        .add_systems(Startup, setup)
        // .add_systems(Update, frame_limiter)
        .add_plugins(FramepacePlugin)
        .run();
}

// If no fonts are loaded and put into the TickLabelFont resource,
// the canvas will not include the tick labels. See the "markers" example
// for an instance of loading a font.
fn setup(
    mut commands: Commands,
    mut plots: ResMut<PlotMap>,
    asset_server: Res<AssetServer>,
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    let font_handle = TickLabelFont {
        maybe_font: Some(asset_server.load("fonts/FiraSans-Bold.ttf")),
    };
    commands.insert_resource(font_handle);

    commands.spawn(Camera2d::default());

    let mut plot = Plot::default();

    let xs = (0..30).map(|i| i as f32 / 30.0).collect::<Vec<f32>>();

    let ys = xs.iter().map(|x| Vec2::new(*x, 0.5 * x)).collect::<Vec<Vec2>>();

    plot.plot(ys);
    plot.set_bounds(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 0.0));

    plots.add(plot.clone());

    settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);
    // commands.spawn().insert(plot_handle);
}
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn frame_limiter() {
    static mut LAST_FRAME_TIME: Option<Instant> = None;
    let target_frame_duration = Duration::from_secs_f64(1.0 / 60.0); // Cap to 60 FPS

    unsafe {
        let now = Instant::now();
        if let Some(last_frame_time) = LAST_FRAME_TIME {
            let frame_time = now.duration_since(last_frame_time);

            if frame_time < target_frame_duration {
                sleep(target_frame_duration - frame_time);
            }
        }
        LAST_FRAME_TIME = Some(Instant::now());
    }
}
