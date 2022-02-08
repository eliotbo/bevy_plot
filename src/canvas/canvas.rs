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

// use crate::plot_format::*;
use crate::plot::*;
use crate::util::*;

pub struct ChangeCanvasMaterialEvent {
    pub plot_handle: Handle<Plot>,
    pub canvas_material_handle: Handle<CanvasMaterial>,
}

pub fn update_canvas_material(
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

#[derive(Component)]
pub struct PlotLabel;

#[derive(Debug, Clone, AsStd140)]
pub struct GraphSize {
    pub size: Vec2,
    pub outer_border: Vec2,
}

pub struct SpawnGraphEvent {
    pub pos: Vec2,
    // pub shader_param_handle: Handle<CanvasMaterial>,
    pub plot_handle: Handle<Plot>,
    pub graph_sprite: GraphSprite,
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
    // pub previous_size: Vec2,
    pub previous_position: Vec2,
    pub previous_scale: Vec2,
}

#[derive(Component, Clone)]
pub struct GraphSprite {
    pub id: KnobId,
    pub position: Vec2,
    pub previous_position: Vec2,
    pub original_size: Vec2,
    // pub previous_size: Vec2,
    pub scale: Vec2,
    pub previous_scale: Vec2,
    pub hover_radius: f32,
    pub analytical_functions: [Option<fn(f32) -> f32>; 8],
    pub plot_handle: Handle<Plot>,
    // pub unlocked: bool,
}

impl GraphSprite {
    // a function that takes CanvasMaterial and makes curve segments of the expo function

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

// #[derive(Debug, Copy, Clone, AsStd140)]
// pub struct LineParams {
//     pub thickness: f32,
//     pub point_type: i32,
//     pub point_radius: f32,
//     pub number_of_points: i32,
//     pub transparency: f32,
//     pub point_color: Vec4,
//     pub color: Vec4,
// }

// impl Default for LineParams {
//     fn default() -> Self {
//         LineParams {
//             thickness: 1.0,
//             point_type: 0,
//             point_radius: 1.0,
//             number_of_points: 0,
//             transparency: 1.0,
//             point_color: Vec4::new(0.13, 0.28, 0.86, 1.0),
//             color: Vec4::new(0.13, 0.28, 0.86, 1.0),
//         }
//     }
// }

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
}

impl CanvasMaterial {
    // pub fn update_position(&mut self, plot: &Plot) {
    //     //
    //     self.position = plot.position;
    // }

    pub fn update_all(&mut self, plot: &Plot) {
        self.position = plot.position;
        // self.mouse_pos = plot.relative_mouse_pos;
        self.tick_period = plot.tick_period;
        self.bounds = plot.bounds.clone();
        self.globals = plot.globals;
        self.size = plot.size;
        self.outer_border = plot.outer_border;
    }

    pub fn new(plot: &Plot) -> Self {
        CanvasMaterial {
            mouse_pos: plot.relative_mouse_pos,
            tick_period: plot.tick_period,
            bounds: plot.bounds.clone(),
            globals: plot.globals,
            size: plot.size,
            outer_border: plot.outer_border,
            position: plot.position,
        }
    }
    // pub fn delta_axes(&self) -> Vec2 {
    //     self.bounds.up - self.bounds.lo
    // }

    // pub fn zoom_axes(&mut self, direction: f32) {
    //     let percent_factor = 10.0;

    //     let multiplier = 1.0 + direction * percent_factor / 100.0;

    //     self.bounds.up =
    //         self.relative_mouse_pos + (self.bounds.up - self.relative_mouse_pos) * multiplier;
    //     self.bounds.lo =
    //         self.relative_mouse_pos - (self.relative_mouse_pos - self.bounds.lo) * multiplier;

    //     self.globals.zoom *= multiplier;

    //     // self.update_thickness(multiplier);
    // }

    // pub fn move_axes(&mut self, mouse_delta: Vec2) {
    //     let mut axes = self.delta_axes();
    //     axes.x *= -1.0;
    //     let size = self.size / (1. + self.outer_border);

    //     self.bounds.up += mouse_delta * axes / size;
    //     self.bounds.lo += mouse_delta * axes / size;
    // }

    // pub fn clamp_tick_period(&mut self) {
    //     let max_num_ticks = 15.0;
    //     let min_num_ticks = 0.000001;

    //     self.tick_period.x = self.tick_period.x.clamp(
    //         self.delta_axes().x / max_num_ticks,
    //         self.delta_axes().x / min_num_ticks,
    //     );

    //     self.tick_period.y = self.tick_period.y.clamp(
    //         self.delta_axes().y / max_num_ticks,
    //         self.delta_axes().x / min_num_ticks,
    //     );
    // }

    // // TODO: take inner border into account
    // pub fn plot_to_world(&self, ys: &Vec<Vec2>) -> Vec<Vec2> {
    //     ys.iter()
    //         .map(|v| {
    //             Vec2::new(
    //                 v.x * self.size.x / (self.bounds.up.x - self.bounds.lo.x),
    //                 (v.y - self.bounds.lo.y) * self.size.y / (self.bounds.up.y - self.bounds.lo.y),
    //             )
    //         })
    //         .collect::<Vec<Vec2>>()
    // }
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
