use bevy::prelude::*;

// mod inputs;
pub mod util;
// mod view;

// use inputs::*;
use util::*;
// use view::*;

use bevy::{
    prelude::*,
    render::camera::OrthographicProjection,
    sprite::{Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "I am a window!".to_string(),
        width: 3000.,
        height: 1700.,
        vsync: true,
        ..Default::default()
    })
    // .insert_resource(Maps::default())
    .insert_resource(Cursor::default())
    .add_plugin(Material2dPlugin::<GraphEditShader>::default())
    .add_plugins(DefaultPlugins)
    // .add_asset::<GraphEditShader>()
    .add_event::<SpawnGraphEvent>()
    .add_event::<ReleaseAllEvent>()
    .add_startup_system(main_setup)
    
    // .add_startup_system(view_setup)
    // .add_system(spawn_graph)
    // .add_system(change_shader)
    // .add_system(record_mouse_events_system)
    // .add_system(release_all)
    // .add_system(move_graph_control_point)
    // blah
    ;

    app.run();
}

// fn test(
//     material: Res<Assets<Material2d>>,
// )

fn main_setup(
    mut commands: Commands,
    mut spawn_graph_event: EventWriter<SpawnGraphEvent>,
    // mut my_shader_params: ResMut<Assets<GraphEditShader>>,
    // mut maps: ResMut<Maps>,
    // mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GraphEditShader>>,
) {
    let size = Vec2::new(300.0, 300.0);

    let mut shader_params = GraphEditShader {
        // color: Color::YELLOW,
        // clearcolor: Color::BLACK,
        bounds: Vec2::new(0.0, 1.0),
        vars: [Vec4::ZERO; 16],
        zoom_time_width: Vec3::new(1.0, 1.0, 1.0),
        hover_index: -1,
        size: Vec2::new(300.0, 300.0),
    };

    shader_params.vars[0] = Vec4::new(0.2, 0.6, 0.0, 0.0);
    shader_params.vars[2] = Vec4::new(0.7, 0.8, 0.0, 0.0);
    shader_params.vars[1] = shader_params.vars[0] / 2.0 + shader_params.vars[2] / 2.0;

    // exponent of easing function
    shader_params.vars[1].z = 1.0;

    // linear version of the y position of the control point
    shader_params.vars[1].w = shader_params.vars[0].y / 2.0 + shader_params.vars[2].y / 2.0;

    // let shader_param_handle = my_shader_params.add(shader_params);

    let material = materials.add(shader_params);

    // quad
    commands.spawn().insert_bundle(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::new(size)))),
        material: material.clone(),
        ..Default::default()
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
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

    spawn_graph_event.send(SpawnGraphEvent {
        pos: Vec2::ZERO,
        shader_param_handle: material,
        id: 1112,
    });
}
