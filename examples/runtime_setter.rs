use bevy::prelude::*;

use bevy_plot::*;

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
        .add_system(change_bezier_metaparameters_at_runtime)
        .run();
}

use bevy_plot::UpdateBezierShaderEvent;

// Press Mouse::Right and drag the mouse to change the thickness of the curve
pub fn change_bezier_metaparameters_at_runtime(
    mut plots: ResMut<Assets<Plot>>,
    query: Query<(Entity, &Handle<Plot>, &BezierCurveNumber)>,
    mouse_position: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut event: EventWriter<UpdateBezierShaderEvent>,
) {
    // for mut custom_uni in query.iter_mut() {
    for (entity, plot_handle, curve_number) in query.iter() {
        let plot = plots.get_mut(plot_handle).unwrap();

        let mouse_pos = mouse_position.position;

        if mouse_button_input.pressed(MouseButton::Right) {
            if let Some(mut bezier_data) = plot.data.bezier_groups.get_mut(curve_number.0) {
                bezier_data.size = mouse_pos.x / 100.0;

                // If show_animation is set to true, UpdateBezierShaderEvent will be sent elsewhere anyway,
                // so we don't need to send it twice every frame.
                if !bezier_data.show_animation {
                    event.send(UpdateBezierShaderEvent {
                        plot_handle: plot_handle.clone(),
                        entity,
                        group_number: curve_number.0,
                    });
                }
            }
        }
    }
}

// TODO:
// 2) Area under the curve
// 3) clean up the code
// 4) generate the docs
// 5) Automatically color curve, segments and markers with palette

fn setup(
    mut commands: Commands,
    colors_res: Res<HashMap<PlotColor, Vec<Color>>>,
    mut plots: ResMut<Assets<Plot>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let colors = colors_res.as_ref();

    let mut plot = Plot::default();

    // sine wave
    plot.plotopt_analytical(
        f3,
        vec![
            Opt::Size(2.0),
            Opt::Color(colors.get(&PlotColor::LightPink).unwrap()[1]),
        ],
    );

    let plot_handle = plots.add(plot.clone());
    commands.spawn().insert(plot_handle);
}

pub fn f3(x: f32, t: f32) -> f32 {
    let freq = 20.0;
    let y = (x * freq + t * 0.0).sin() / 2.0 + 0.5;
    return y;
}

/*

// Here is a way of changing the shader parameters without modifying the
// model. Changes will not be remembered if RespawnAllEvent is sent later.

pub fn change_segment_uni(
    mut query: Query<&mut SegmentUniform>,
    mouse_position: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for mut segment_uni in query.iter_mut() {
        let mouse_pos = mouse_position.position;

        if mouse_button_input.pressed(MouseButton::Left) {
            segment_uni.hole_size = mouse_pos.x / 100.0;
        } else if mouse_button_input.pressed(MouseButton::Right) {
            segment_uni.segment_size = (mouse_pos.x / 100.0).clamp(0.2, 3.0);
        }
    }
}

pub fn change_marker_uni(
    mut query: Query<&mut MarkerUniform>,
    mouse_position: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for mut custom_uni in query.iter_mut() {
        let mouse_pos = mouse_position.position;

        if mouse_button_input.pressed(MouseButton::Right) {
            custom_uni.marker_size = mouse_pos.x / 100.0;
        }
    }
}

*/
