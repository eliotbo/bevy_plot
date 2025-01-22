// pub mod canvas;
pub mod canvas_actions;
#[allow(unused_imports)]
pub use canvas_actions::*;

// use bevy::{
//     ecs::system::{lifetimeless::SRes, SystemParamItem},
//     prelude::*,
//     reflect::TypeUuid,
//     render::{
//         extract_resource::ExtractResource,
//         render_asset::{PrepareAssetError, RenderAsset},
//         render_resource::*,
//         renderer::RenderDevice,
//     },
//     sprite::{Material2d, Material2dPipeline, Material2dPlugin},
// };

use crate::plot::*;
use crate::util::*;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::*;
use bevy::sprite::{Material2d, Material2dPlugin};
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;

#[derive(Component)]
pub(crate) struct PlotLabel;

#[derive(Component)]
pub(crate) struct TargetLabel;

#[derive(Event)]
pub(crate) struct SpawnGraphEvent {
    pub plot_id: PlotId,
    pub canvas: CanvasParams,
}

pub(crate) enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

// TODO: unimplemented
#[derive(Component)]
pub(crate) struct ResizePlotWindow {
    pub corner: Corner,
    #[allow(dead_code)]
    pub previous_position: Vec2,
    pub previous_scale: Vec2,
}

#[derive(Component, Clone)]
pub(crate) struct CanvasParams {
    pub position: Vec2,
    #[allow(dead_code)]
    pub previous_position: Vec2,
    pub original_size: Vec2,
    pub scale: Vec2,
    pub previous_scale: Vec2,
    pub hover_radius: f32,
}

impl CanvasParams {
    pub(crate) fn within_rect(&self, position: Vec2) -> bool {
        let size = self.original_size * self.scale;
        if position.x < self.position.x + size.x / 2.0
            && position.x > self.position.x - size.x / 2.0
            && position.y < self.position.y + size.y / 2.0
            && position.y > self.position.y - size.y / 2.0
        {
            return true;
        }
        return false;
    }

    pub(crate) fn clicked_on_plot_corner(&self, position: Vec2, commands: &mut Commands, entity: Entity) {
        let size = self.original_size * self.scale;
        let top_right = self.position + Vec2::new(size.x / 2.0, size.y / 2.0);
        let bottom_left = self.position + Vec2::new(-size.x / 2.0, -size.y / 2.0);
        let top_left = self.position + Vec2::new(-size.x / 2.0, size.y / 2.0);
        let bottom_right = self.position + Vec2::new(size.x / 2.0, -size.y / 2.0);

        // println!("{:?}", position);
        // println!("top_left: {:?}", top_left);

        if (top_right - position).length() < self.hover_radius {
            commands.entity(entity).insert(ResizePlotWindow {
                corner: Corner::TopRight,
                // previous_size: size,
                previous_position: self.position,
                previous_scale: self.scale,
            });
        }

        if (bottom_left - position).length() < self.hover_radius {
            commands.entity(entity).insert(ResizePlotWindow {
                corner: Corner::BottomLeft,
                // previous_size: size,
                previous_position: self.position,
                previous_scale: self.scale,
            });
        }

        if (top_left - position).length() < self.hover_radius {
            commands.entity(entity).insert(ResizePlotWindow {
                corner: Corner::TopLeft,
                // previous_size: self.size,
                previous_position: self.position,
                previous_scale: self.scale,
            });
            println!("top left");
        }

        if (bottom_right - position).length() < self.hover_radius {
            commands.entity(entity).insert(ResizePlotWindow {
                corner: Corner::BottomRight,
                // previous_size: self.size,
                previous_position: self.position,
                previous_scale: self.scale,
            });
        }
    }

    pub(crate) fn hovered_on_plot_edges(&self, position: Vec2, window_entity: Entity, mut commands: &mut Commands) {
        let size = self.original_size * self.scale;

        let top_right = self.position + Vec2::new(size.x / 2.0, size.y / 2.0);
        let bottom_left = self.position + Vec2::new(-size.x / 2.0, -size.y / 2.0);
        let top_left = self.position + Vec2::new(-size.x / 2.0, size.y / 2.0);
        let bottom_right = self.position + Vec2::new(size.x / 2.0, -size.y / 2.0);

        let mut set_to_default_cursor = true;

        if (top_left - position).length() < self.hover_radius {
            commands
                .entity(window_entity)
                .insert(CursorIcon::System(SystemCursorIcon::NwResize));
            set_to_default_cursor = false;
        }

        if (top_right - position).length() < self.hover_radius {
            commands
                .entity(window_entity)
                .insert(CursorIcon::System(SystemCursorIcon::NeResize));
            set_to_default_cursor = false;
        }

        if (bottom_left - position).length() < self.hover_radius {
            commands
                .entity(window_entity)
                .insert(CursorIcon::System(SystemCursorIcon::SwResize));
            set_to_default_cursor = false;
        }

        if (bottom_right - position).length() < self.hover_radius {
            commands
                .entity(window_entity)
                .insert(CursorIcon::System(SystemCursorIcon::SeResize));
            set_to_default_cursor = false;
        }

        if set_to_default_cursor {
            commands
                .entity(window_entity)
                .insert(CursorIcon::System(SystemCursorIcon::Default));
        }
    }
}

#[derive(Component)]
pub(crate) struct MoveAxes;

#[derive(Component)]
pub(crate) struct ZoomAxes {
    pub wheel_dir: f32,
    #[allow(dead_code)]
    pub mouse_pos: Vec2,
}

#[derive(Event)]
pub(crate) struct UpdatePlotLabelsEvent {
    pub plot_id: PlotId,
    pub canvas_entity: Entity,
}

#[derive(Event)]
pub(crate) struct UpdateTargetLabelEvent {
    pub plot_id: PlotId,
    pub canvas_entity: Entity,
    pub canvas_material_handle: MeshMaterial2d<CanvasMaterial>,
    // pub mouse_pos: Vec2,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct CanvasMaterial {
    #[uniform(1)]
    pub position: Vec2,

    /// Mouse position in the reference frame of the graph, corresponding to its axes coordinates
    #[uniform(0)]
    pub mouse_pos: Vec2,
    #[uniform(0)]
    pub tick_period: Vec2,

    /// Extreme points of the canvas
    #[uniform(0)]
    pub bound_up: Vec2,
    #[uniform(0)]
    pub bound_lo: Vec2,

    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub zoom: f32,
    #[uniform(0)]
    pub size: Vec2,
    #[uniform(0)]
    pub outer_border: Vec2,

    #[uniform(0)]
    pub show_target: f32,
    #[uniform(0)]
    pub hide_contour: f32,
    #[uniform(0)]
    pub target_pos: Vec2,

    #[uniform(0)]
    pub background_color1: Vec4,
    #[uniform(0)]
    pub background_color2: Vec4,
    #[uniform(0)]
    pub target_color: Vec4,

    #[uniform(0)]
    pub show_grid: f32,
    #[uniform(0)]
    pub show_axes: f32,
}

impl CanvasMaterial {
    pub fn new(plot: &Plot) -> Self {
        CanvasMaterial {
            mouse_pos: Vec2::ZERO,
            tick_period: plot.tick_period,
            // bounds: plot.bounds.clone(),
            bound_up: plot.bounds.up,
            bound_lo: plot.bounds.lo,
            time: 0.0,
            zoom: 1.0,
            size: plot.canvas_size,
            outer_border: plot.outer_border,
            position: plot.canvas_position,
            show_target: if plot.show_target && plot.target_toggle {
                1.0
            } else {
                0.0
            },
            hide_contour: if plot.hide_contour { 1.0 } else { 0.0 },
            target_pos: Vec2::ZERO,
            background_color1: col_to_vec4(plot.background_color1),
            background_color2: col_to_vec4(plot.background_color2),
            target_color: col_to_vec4(plot.target_color),
            show_grid: if plot.show_grid { 1.0 } else { 0.0 },
            show_axes: if plot.show_axes { 1.0 } else { 0.0 },
        }
    }

    /// Updates all the shader parameters except the mouse_pos, which is updated every frame anyway.
    pub fn update_all(&mut self, plot: &Plot) {
        // mouse_pos is supposed to be in World coordinates // self.mouse_pos = plot.plot_coord_mouse_pos;

        self.position = plot.canvas_position;
        self.tick_period = plot.tick_period;

        self.bound_up = plot.bounds.up;
        self.bound_lo = plot.bounds.lo;
        self.zoom = plot.zoom;
        self.time = plot.time;
        self.size = plot.canvas_size;
        self.outer_border = plot.outer_border;
        self.show_target = if plot.show_target && plot.target_toggle {
            1.0
        } else {
            0.0
        };
        // let v = Vec2::new(.x, plot.target_position.y);
        self.target_pos = plot.to_local(plot.target_position) + plot.canvas_position;

        self.background_color1 = col_to_vec4(plot.background_color1);
        self.background_color2 = col_to_vec4(plot.background_color2);
        self.target_color = col_to_vec4(plot.target_color);
        self.show_grid = if plot.show_grid { 1.0 } else { 0.0 };
        self.show_axes = if plot.show_axes { 1.0 } else { 0.0 };
    }

    /// Checks whether position is inside the plot bounderies or not.
    pub fn within_rect(&self, position: Vec2) -> bool {
        let size = self.size;
        if position.x < self.position.x + size.x / 2.0
            && position.x > self.position.x - size.x / 2.0
            && position.y < self.position.y + size.y / 2.0
            && position.y > self.position.y - size.y / 2.0
        {
            return true;
        }
        return false;
    }
}

// impl bevy::sprite::Material2d for CanvasMaterial {
//     fn fragment_shader() -> ShaderRef {
//         let handle_untyped = CANVAS_SHADER_HANDLE.clone();
//         let shader_handle: Handle<Shader> = handle_untyped.typed::<Shader>();
//         shader_handle.into()
//     }
// }

// Use hot-reload from file OR inline your wgsl. If using a path:
impl Material2d for CanvasMaterial {
    fn fragment_shader() -> ShaderRef {
        // For example if you keep canvas.wgsl in assets/canvas/canvas.wgsl:
        "shaders/canvas.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}
