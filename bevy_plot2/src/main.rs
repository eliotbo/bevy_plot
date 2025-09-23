//! A 2D bevy plot library starting point with rectangles

use bevy::{
    prelude::*,
    render::mesh::Mesh2d,
    sprite::MeshMaterial2d,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera for 2D
    commands.spawn(Camera2d);

    // Create a grid of rectangles using 2D meshes
    for x in 0..10 {
        for y in 0..10 {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(20.0, 20.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(
                    Color::hsl(x as f32 * 36.0, 0.5 + y as f32 * 0.05, 0.5)
                ))),
                Transform::from_xyz(
                    x as f32 * 40.0 - 180.0,
                    y as f32 * 40.0 - 180.0,
                    0.0
                ),
            ));
        }
    }
}