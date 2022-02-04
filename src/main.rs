use bevy::prelude::*;

mod canvas;
use canvas::*;

pub mod markers;
pub use markers::*;

mod inputs;
mod util;
use util::*;

// use inputs::*;

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
        .add_plugin(PlotCanvasPlugin)
        .add_plugin(MarkersPlugin)
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

// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut spawn_graph_event: EventWriter<SpawnGraphEvent>,
    // mut materials: ResMut<Assets<CanvasMaterial>>,
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
    // .insert(Cam::default());

    let size = Vec2::new(777.0, 500.0) * 1.0;

    let mut plot = Plot {
        relative_mouse_pos: Vec2::new(0.4, 0.5),

        tick_period: Vec2::new(0.27, 0.22),

        bounds: PlotCanvasBounds {
            up: Vec2::new(2.1, 3.4) * 1.0,
            lo: Vec2::new(-1.03, -0.85) * 1.0,
        },

        globals: PlotGlobals {
            time: 0.0,
            zoom: 1.0,
            dum1: 0.0, // for future use
            dum2: 0.0, // for future use
        },

        size: size.clone(),
        outer_border: Vec2::new(0.03 * size.y / size.x, 0.03),
        zero_world: Vec2::new(0.0, 0.0),

        // position: Vec2::new(65.0, 28.0) * 1.,
        position: Vec2::ZERO,
    };
    plot.compute_zeros();

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

    // plot.plot_analytical(|x| custom_sin(x), 1);

    // graph_sprite.make_data(&mut plot);

    spawn_graph_event.send(SpawnGraphEvent {
        pos: Vec2::ZERO,
        // shader_param_handle: material_handle,
        graph_sprite,
        plot_handle: plot_handle.clone(),
    });
}

pub fn custom_sin(x: f32) -> f32 {
    (2.0 * 2.0 * 3.1416 * (x)).sin() * 0.5 + 0.2
}
