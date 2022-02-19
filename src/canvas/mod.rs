// pub mod canvas;
pub mod canvas_actions;
#[allow(unused_imports)]
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
use crate::util::*;

#[derive(Component)]
pub(crate) struct PlotLabel;

#[derive(Component)]
pub(crate) struct TargetLabel;

pub(crate) struct SpawnGraphEvent {
    pub plot_handle: Handle<Plot>,
    pub canvas: Canvas,
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
pub(crate) struct Canvas {
    pub position: Vec2,
    #[allow(dead_code)]
    pub previous_position: Vec2,
    pub original_size: Vec2,
    pub scale: Vec2,
    pub previous_scale: Vec2,
    pub hover_radius: f32,
}

impl Canvas {
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

    pub(crate) fn clicked_on_plot_corner(
        &self,
        position: Vec2,
        commands: &mut Commands,
        entity: Entity,
    ) {
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

    pub(crate) fn hovered_on_plot_edges(&self, position: Vec2, windows: &mut ResMut<Windows>) {
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
pub(crate) struct MoveAxes;

#[derive(Component)]
pub(crate) struct ZoomAxes {
    pub wheel_dir: f32,
    #[allow(dead_code)]
    pub mouse_pos: Vec2,
}

pub(crate) struct UpdatePlotLabelsEvent {
    pub plot_handle: Handle<Plot>,
    pub canvas_entity: Entity,
}

pub(crate) struct UpdateTargetLabelEvent {
    pub plot_handle: Handle<Plot>,
    pub canvas_entity: Entity,
    pub canvas_material_handle: Handle<CanvasMaterial>,
    // pub mouse_pos: Vec2,
}

/// Canvas shader parameters
#[derive(TypeUuid, Debug, Clone, Component, AsStd140)]
#[uuid = "1e08866c-0b8a-437e-8bae-38844b21137e"]
#[allow(non_snake_case)]
pub(crate) struct CanvasMaterial {
    /// Mouse position in the reference frame of the graph, corresponding to its axes coordinates
    pub mouse_pos: Vec2,
    pub tick_period: Vec2,

    /// Extreme points of the canvas
    pub bounds: PlotCanvasBounds,

    pub time: f32,
    pub zoom: f32,
    pub size: Vec2,
    pub outer_border: Vec2,
    pub position: Vec2,
    pub show_target: f32,
    pub hide_contour: f32,
    pub target_pos: Vec2,

    pub background_color1: Vec4,
    pub background_color2: Vec4,
    pub target_color: Vec4,

    pub show_grid: f32,
    pub show_axes: f32,
}

impl CanvasMaterial {
    pub fn new(plot: &Plot) -> Self {
        CanvasMaterial {
            mouse_pos: Vec2::ZERO,
            tick_period: plot.tick_period,
            bounds: plot.bounds.clone(),
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
        self.bounds = plot.bounds.clone();
        self.zoom = plot.zoom;
        self.time = plot.time;
        self.size = plot.canvas_size;
        self.outer_border = plot.outer_border;
        self.show_target = if plot.show_target && plot.target_toggle {
            1.0
        } else {
            0.0
        };
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

#[derive(Clone)]
pub(crate) struct GpuCanvasMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

pub(crate) struct CanvasMesh2dPlugin;

pub const CANVAS_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 11248119131354745027);

impl Plugin for CanvasMesh2dPlugin {
    fn build(&self, app: &mut App) {
        // let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();

        let handle_untyped = CANVAS_SHADER_HANDLE.clone();

        shaders.set_untracked(
            handle_untyped.clone(),
            Shader::from_wgsl(include_str!("canvas.wgsl")),
        );

        // // at the moment, there seems to be no way to include a font in the crate
        // let mut fonts = app.world.get_resource_mut::<Assets<Font>>().unwrap();
    }
}

impl Material2d for CanvasMaterial {
    fn fragment_shader(_asset_server: &AssetServer) -> Option<Handle<Shader>> {
        let handle_untyped = CANVAS_SHADER_HANDLE.clone();
        let shader_handle: Handle<Shader> = handle_untyped.typed::<Shader>();
        Some(shader_handle)
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
