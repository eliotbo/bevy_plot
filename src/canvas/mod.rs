// pub mod canvas;
pub mod canvas_actions;
pub use canvas_actions::*;

use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            std140::{AsStd140, Std140},
            *,
        },
        renderer::RenderDevice,
    },
    sprite::Material2d,
    sprite::Material2dPipeline,
};

use crate::plot::*;
// use crate::util::*;

pub struct UpdateShadersEvent {
    pub plot_handle: Handle<Plot>,
    pub canvas_material_handle: Handle<CanvasMaterial>,
}

pub fn update_canvas_material(
    // mut commands: Commands,
    mut materials: ResMut<Assets<CanvasMaterial>>,
    plots: ResMut<Assets<Plot>>,
    mut change_mat_event: EventReader<UpdateShadersEvent>,
) {
    for event in change_mat_event.iter() {
        if let Some(material) = materials.get_mut(event.canvas_material_handle.clone()) {
            if let Some(plot) = plots.get(event.plot_handle.clone()) {
                material.update_all(&plot);
            }
        }
    }
}

#[derive(Component)]
pub struct PlotLabel;

#[derive(Component)]
pub struct TargetLabel;

#[derive(Debug, Clone, AsStd140)]
pub struct GraphSize {
    pub size: Vec2,
    pub outer_border: Vec2,
}

pub struct SpawnGraphEvent {
    // pub pos: Vec2,
    pub plot_handle: Handle<Plot>,
    pub canvas: Canvas,
}

pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Component)]
pub struct ResizePlotWindow {
    pub corner: Corner,
    pub previous_position: Vec2,
    pub previous_scale: Vec2,
}

#[derive(Component, Clone)]
pub struct Canvas {
    pub position: Vec2,
    pub previous_position: Vec2,
    pub original_size: Vec2,
    pub scale: Vec2,
    pub previous_scale: Vec2,
    pub hover_radius: f32,
}

impl Canvas {
    pub fn within_rect(&self, position: Vec2) -> bool {
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

    pub fn clicked_on_plot_corner(&self, position: Vec2, commands: &mut Commands, entity: Entity) {
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

    pub fn hovered_on_plot_edges(&self, position: Vec2, windows: &mut ResMut<Windows>) {
        let size = self.original_size * self.scale;

        let top_right = self.position + Vec2::new(size.x / 2.0, size.y / 2.0);
        let bottom_left = self.position + Vec2::new(-size.x / 2.0, -size.y / 2.0);
        let top_left = self.position + Vec2::new(-size.x / 2.0, size.y / 2.0);
        let bottom_right = self.position + Vec2::new(size.x / 2.0, -size.y / 2.0);

        let mut set_to_default_cursor = true;
        let window = windows.get_primary_mut().unwrap();

        if (top_left - position).length() < self.hover_radius {
            window.set_cursor_icon(CursorIcon::NwResize);
            set_to_default_cursor = false;
        }

        if (top_right - position).length() < self.hover_radius {
            window.set_cursor_icon(CursorIcon::NeResize);
            set_to_default_cursor = false;
        }

        if (bottom_left - position).length() < self.hover_radius {
            window.set_cursor_icon(CursorIcon::SwResize);
            set_to_default_cursor = false;
        }

        if (bottom_right - position).length() < self.hover_radius {
            window.set_cursor_icon(CursorIcon::SeResize);
            set_to_default_cursor = false;
        }

        if set_to_default_cursor {
            window.set_cursor_icon(CursorIcon::Default);
        }
    }
}

#[derive(Component)]
pub struct MoveAxes;

#[derive(Component)]
pub struct ZoomAxes {
    pub wheel_dir: f32,
    pub mouse_pos: Vec2,
}

pub struct UpdatePlotLabelsEvent {
    pub plot_handle: Handle<Plot>,
    pub canvas_entity: Entity,
}

pub struct UpdateTargetLabelEvent {
    pub plot_handle: Handle<Plot>,
    pub canvas_entity: Entity,
    pub canvas_material_handle: Handle<CanvasMaterial>,
    // pub mouse_pos: Vec2,
}

///// Shader parameters
#[derive(TypeUuid, Debug, Clone, Component, AsStd140)]
#[uuid = "1e08866c-0b8a-437e-8bae-38844b21137e"]
#[allow(non_snake_case)]
pub struct CanvasMaterial {
    // mouse_pos in the reference frame of the graph, corresponding to its axes coordinates
    pub mouse_pos: Vec2,
    pub tick_period: Vec2,
    pub bounds: PlotCanvasBounds,

    pub globals: PlotGlobals,
    pub size: Vec2,
    pub outer_border: Vec2,
    pub position: Vec2,
    pub show_target: f32,
    pub target_pos: Vec2,
}

impl CanvasMaterial {
    pub fn new(plot: &Plot) -> Self {
        CanvasMaterial {
            mouse_pos: Vec2::ZERO,
            tick_period: plot.tick_period,
            bounds: plot.bounds.clone(),
            globals: plot.globals,
            size: plot.canvas_size,
            outer_border: plot.outer_border,
            position: plot.canvas_position,
            show_target: 1.0,
            target_pos: Vec2::ZERO,
        }
    }

    pub fn update_all(&mut self, plot: &Plot) {
        // mouse_pos is supposed to be in World coordinates // self.mouse_pos = plot.plot_coord_mouse_pos;

        self.position = plot.canvas_position;
        self.tick_period = plot.tick_period;
        self.bounds = plot.bounds.clone();
        self.globals = plot.globals;
        self.size = plot.canvas_size;
        self.outer_border = plot.outer_border;
        self.show_target = if plot.show_target { 1.0 } else { 0.0 };
        self.target_pos = plot.to_local(plot.target_position) + plot.canvas_position;
    }

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

#[derive(Clone)]
pub struct GpuCanvasMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl Material2d for CanvasMaterial {
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        asset_server.watch_for_changes().unwrap();
        Some(asset_server.load("shaders/plot_canvas.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(CanvasMaterial::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }
}

impl RenderAsset for CanvasMaterial {
    type ExtractedAsset = CanvasMaterial;
    type PreparedAsset = GpuCanvasMaterial;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let custom_material_std140 = extracted_asset.as_std140();
        let custom_material_bytes = custom_material_std140.as_bytes();

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: custom_material_bytes,
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
            layout: &material_pipeline.material2d_layout,
        });

        Ok(GpuCanvasMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}
