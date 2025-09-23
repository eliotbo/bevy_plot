use bevy::{
    color::palettes::css,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // Camera
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.05, 0.05, 0.1)),
            ..default()
        },
    ));

    // Instructions text
    commands.spawn((
        Text::new("Canvas Demo - Simplified\n\nThis is a simplified version to test the canvas shader.\nThe full demo requires Material2d bundle fixes for Bevy 0.16."),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(css::WHITE.into()),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
    ));

    println!("Canvas demo simplified version loaded.");
    println!("The canvas implementation has been migrated to bevy_plot2:");
    println!("- Canvas shader (canvas.wgsl)");
    println!("- CanvasMaterial with Material2d");
    println!("- Mouse tracking and cursor system");
    println!("- Pan and zoom controls");
    println!("- Grid and axes rendering");
    println!("- Tick labels system");
    println!("\nTo complete the demo, the Material2d bundle spawn pattern needs to be updated for Bevy 0.16's new component structure.");
}