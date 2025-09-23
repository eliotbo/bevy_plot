pub mod canvas;

pub use canvas::*;

use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;

pub struct BevyPlot2Plugin;

impl Plugin for BevyPlot2Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CanvasMaterial>::default())
            .init_resource::<Cursor>()
            .add_systems(
                Update,
                (
                    update_cursor_system,
                    update_canvas_mouse_pos,
                    pan_canvas_system,
                    zoom_canvas_system,
                    update_tick_labels_system,
                ),
            );
    }
}