use bevy::{prelude::*, sprite::Material2dPlugin};

use crate::canvas::*;
use crate::inputs::Cursor;
use crate::inputs::*;
use crate::util::*;

pub struct PlotCanvasPlugin;

impl Plugin for PlotCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor::default())
            .add_asset::<Plot>()
            .add_plugin(Material2dPlugin::<CanvasMaterial>::default())
            .add_event::<SpawnGraphEvent>()
            .add_event::<ReleaseAllEvent>()
            .add_event::<UpdatePlotLabelsEvent>()
            .add_event::<ChangeCanvasMaterialEvent>()
            .add_system(release_all)
            .add_system(spawn_graph.label("spawn_graph"))
            .add_system(change_plot)
            .add_system(update_plot_labels.before("spawn_graph"))
            .add_system(adjust_graph_size)
            .add_system(adjust_graph_axes)
            .add_system(record_mouse_events_system)
            .add_system(change_canvas_material);
    }
}

pub struct ChangeCanvasMaterialEvent {
    pub plot_handle: Handle<Plot>,
    pub canvas_material_handle: Handle<CanvasMaterial>,
}

pub fn change_canvas_material(
    // mut commands: Commands,
    mut materials: ResMut<Assets<CanvasMaterial>>,
    plots: ResMut<Assets<Plot>>,
    mut change_mat_event: EventReader<ChangeCanvasMaterialEvent>,
) {
    for event in change_mat_event.iter() {
        if let Some(material) = materials.get_mut(event.canvas_material_handle.clone()) {
            if let Some(plot) = plots.get(event.plot_handle.clone()) {
                material.update_all(&plot);
            }
        }
    }
}
