use bevy::{
    // asset::Assets,
    core_pipeline::core_2d::Transparent2d,
    ecs::system::lifetimeless::{Read, SQuery, SRes},
    ecs::system::SystemParamItem,
    prelude::*,
    reflect::TypeUuid,
    render::{
        extract_component::{ComponentUniforms, DynamicUniformIndex, UniformComponentPlugin},
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        extract_resource::ExtractResource,
        mesh::{GpuBufferInfo, MeshVertexBufferLayout},
        render_asset::RenderAssets,
        render_phase::{
            AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase, SetItemPipeline,
            TrackedRenderPass,
        },
        // render_resource::{std140::AsStd140, *},
        render_resource::*,
        renderer::RenderDevice,

        view::{ComputedVisibility, Msaa, Visibility, VisibleEntities},
        view::{ExtractedView, NoFrustumCulling},
        RenderApp,
        RenderStage,
    },
    sprite::{
        Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, Mesh2dUniform, SetMesh2dBindGroup, SetMesh2dViewBindGroup,
    },
    utils::FloatOrd,
};

use crate::plot::*;
use crate::util::*;
use bytemuck::{Pod, Zeroable};
// use crevice::std140::AsStd140;

// TODOs:
// 1) Modify the transform instead of spawning brand new entities
// this way, the uniform will stay the same

pub(crate) fn markers_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut change_canvas_material_event: EventReader<RespawnAllEvent>,
    mut plots: ResMut<Assets<Plot>>,
    query: Query<(Entity, &Handle<Plot>), With<MarkerUniform>>,
) {
    for event in change_canvas_material_event.iter() {
        //
        for (entity, plot_handle) in query.iter() {
            if event.plot_handle == *plot_handle {
                commands.entity(entity).despawn();
            }
        }

        let mut plot = plots.get_mut(&event.plot_handle).unwrap();

        plot_points(
            &mut commands,
            &mut meshes,
            // ys,
            &mut plot,
            &event.plot_handle,
        )
    }
}

fn plot_points(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    plot: &mut Plot,
    plot_handle: &Handle<Plot>,
) {
    let data = plot.data.clone();
    // let color = data.marker_plot.color;
    for marker_plot in data.marker_groups.iter() {
        let ys = marker_plot.data.clone();
        // let color = marker_plot.color;
        // let ys_world = plot.plot_to_local(&ys);
        let ys_world = ys.iter().map(|y| plot.to_local(*y)).collect::<Vec<Vec2>>();

        let quad_size = 30.0;

        commands
            .spawn_bundle((
                Mesh2dHandle(meshes.add(Mesh::from(shape::Quad {
                    size: Vec2::splat(quad_size),
                    flip: false,
                }))),
                GlobalTransform::default(),
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.12)),
                Visibility::default(),
                ComputedVisibility::default(),
                MarkerInstanceMatData(
                    ys_world
                        .iter()
                        .map(|v| MarkerInstanceData {
                            //
                            // TODO: take inner border into account
                            //
                            position: Vec3::new(v.x, v.y, 1.01) + plot.canvas_position.extend(0.000),
                            scale: 1.0,
                            color: Color::rgba(0.8, 0.6, 0.1, 1.0).as_rgba_f32(),
                        })
                        .collect(),
                ),
                NoFrustumCulling,
                // NoFrustumCulling,
            ))
            .insert(plot_handle.clone())
            .insert(MarkerUniform {
                marker_size: marker_plot.size,
                hole_size: 1.0,
                zoom: 1.0,
                marker_type: marker_plot.marker_style.to_int32(),
                marker_point_color: col_to_vec4(marker_plot.marker_point_color),
                color: col_to_vec4(marker_plot.color),
                quad_size,
                inner_canvas_size_in_pixels: plot.canvas_size / (1.0 + plot.outer_border),
                // outer_border: plot.outer_border,
                canvas_position: plot.canvas_position,
                contour: if marker_plot.draw_contour { 1.0 } else { 0.0 },
            });
    }
}

#[derive(Component)]
pub(crate) struct MarkerInstanceMatData(Vec<MarkerInstanceData>);

impl ExtractComponent for MarkerInstanceMatData {
    type Query = &'static MarkerInstanceMatData;
    type Filter = ();

    fn extract_component(item: bevy::ecs::query::QueryItem<Self::Query>) -> Self {
        MarkerInstanceMatData(item.0.clone())
    }
}

#[derive(Component, Default)]
pub(crate) struct MarkerMesh2d;

// /// Uniform sent to markers.wgsl
// #[derive(Component, Clone, AsStd140, ExtractResource)]
// pub(crate) struct MarkerUniform {
//     pub marker_size: f32,
//     /// When the ```marker_point_color``` field is different from the ```color``` field,
//     /// there is a small visible circle within the marker. ```hole_size``` controls the size of the circle.
//     pub hole_size: f32,
//     pub zoom: f32,
//     pub marker_type: i32,
//     /// Size of the instanced square quad for one marker.
//     pub quad_size: f32,

//     /// Shows a black contour around the marker if the value is > 0.5.
//     pub contour: f32,
//     pub inner_canvas_size_in_pixels: crevice::std140::Vec2,
//     pub canvas_position: crevice::std140::Vec2,
//     pub color: crevice::std140::Vec4,

//     /// Color of the small circle within the marker.
//     pub marker_point_color: crevice::std140::Vec4,
// }

#[derive(Component, Clone, AsBindGroup, ShaderType)]
pub(crate) struct MarkerUniform {
    #[uniform(0)]
    pub marker_size: f32,
    /// When the ```marker_point_color``` field is different from the ```color``` field,
    /// there is a small visible circle within the marker. ```hole_size``` controls the size of the circle.
    #[uniform(0)]
    pub hole_size: f32,
    #[uniform(0)]
    pub zoom: f32,
    #[uniform(0)]
    pub marker_type: i32,
    /// Size of the instanced square quad for one marker.
    #[uniform(0)]
    pub quad_size: f32,

    /// Shows a black contour around the marker if the value is > 0.5.
    #[uniform(0)]
    pub contour: f32,
    #[uniform(0)]
    pub inner_canvas_size_in_pixels: Vec2,
    #[uniform(0)]
    pub canvas_position: Vec2,
    #[uniform(0)]
    pub color: Vec4,

    /// Color of the small circle within the marker.
    #[uniform(0)]
    pub marker_point_color: Vec4,
}

impl ExtractComponent for MarkerUniform {
    type Query = &'static MarkerUniform;
    type Filter = ();

    fn extract_component(item: bevy::ecs::query::QueryItem<Self::Query>) -> Self {
        item.clone()

        // MarkerUniform {
        //     marker_size: item.marker_size,
        //     hole_size: item.hole_size,
        //     zoom: item.0.zoom,
        //     marker_type: item.0.marker_type,
        //     quad_size: item.0.quad_size,
        //     contour: item.0.contour,
        //     inner_canvas_size_in_pixels: item.0.inner_canvas_size_in_pixels,
        //     canvas_position: item.0.canvas_position,
        //     color: item.0.color,
        //     marker_point_color: item.0.marker_point_color,
        // }
    }
}

// TODO: we have instance data, but we don't use it at the moment.
// One use case would be to have marker size as an additional dimension.

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct MarkerInstanceData {
    position: Vec3,
    scale: f32,
    color: [f32; 4],
}

/// Custom pipeline for 2d meshes with vertex colors
pub(crate) struct MarkerMesh2dPipeline {
    /// this pipeline wraps the standard [`Mesh2dPipeline`]
    mesh2d_pipeline: Mesh2dPipeline,
    pub custom_uniform_layout: BindGroupLayout,
    // pub shader: Handle<Shader>,
    // material_layout: BindGroupLayout,
}

impl FromWorld for MarkerMesh2dPipeline {
    fn from_world(world: &mut World) -> Self {
        let mesh2d_pipeline = Mesh2dPipeline::from_world(world).clone();

        let render_device = world.get_resource::<RenderDevice>().unwrap();

        let custom_uniform_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: BufferSize::new(MarkerUniform::min_size().into()),
                },
                count: None,
            }],
            label: Some("markers_uniform_layout"),
        });

        // let world = world.cell();
        // let asset_server = world.get_resource::<AssetServer>().unwrap();

        // let shader = asset_server.load("../assets/shaders/markers.wgsl");

        // let _result = asset_server.watch_for_changes();

        Self {
            mesh2d_pipeline,
            custom_uniform_layout,
            // shader,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct MarkerPipelineKey {
    mesh: Mesh2dPipelineKey,
    shader_handle: Handle<Shader>,
}

impl SpecializedMeshPipeline for MarkerMesh2dPipeline {
    type Key = MarkerPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh2d_pipeline.specialize(key.mesh, layout)?;

        descriptor.vertex.shader = key.shader_handle.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<MarkerInstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: VertexFormat::Float32x4.size(),
                    shader_location: 4,
                },
                // VertexAttribute {
                //     format: VertexFormat::Float32x4,
                //     offset: VertexFormat::Float32x4.size(),
                //     shader_location: 5,
                // },
            ],
        });
        descriptor.fragment.as_mut().unwrap().shader = key.shader_handle.clone();
        descriptor.layout = Some(vec![
            self.mesh2d_pipeline.view_layout.clone(),
            self.custom_uniform_layout.clone(),
            self.mesh2d_pipeline.mesh_layout.clone(),
        ]);

        Ok(descriptor)
    }
}

// This specifies how to render a colored 2d mesh
type DrawMarkerMesh2d = (
    // Set the pipeline
    SetItemPipeline,
    // Set the view uniform as bind group 0
    SetMesh2dViewBindGroup<0>,
    // Set the marker uniform as bind group 1
    SetMarkerUniformBindGroup<1>,
    // Set the mesh uniform as bind group 2
    SetMesh2dBindGroup<2>,
    // Draw the mesh
    DrawMarkerMeshInstanced,
);

pub(crate) struct MarkerMesh2dPlugin;

pub(crate) struct MarkerShaderHandle(pub Handle<Shader>);

pub const MARKER_SHADER_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 9826352034109932589);

impl Plugin for MarkerMesh2dPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();

        let handle_untyped = MARKER_SHADER_HANDLE.clone();

        shaders.set_untracked(handle_untyped.clone(), Shader::from_wgsl(include_str!("markers.wgsl")));

        let shader_typed_handle = shaders.get_handle(handle_untyped);

        // app.add_plugin(UniformComponentPlugin::<MarkerUniform>::default());
        app.add_plugin(ExtractComponentPlugin::<MarkerInstanceMatData>::default());
        app.add_plugin(UniformComponentPlugin::<MarkerUniform>::default());
        app.add_plugin(ExtractComponentPlugin::<MarkerUniform>::default());

        // Register our custom draw function and pipeline, and add our render systems
        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        render_app
            .add_render_command::<Transparent2d, DrawMarkerMesh2d>()
            .init_resource::<MarkerMesh2dPipeline>()
            .init_resource::<SpecializedMeshPipelines<MarkerMesh2dPipeline>>()
            .insert_resource(MarkerShaderHandle(shader_typed_handle))
            .add_system_to_stage(RenderStage::Prepare, prepare_instance_buffers)
            // .add_system_to_stage(RenderStage::Extract, extract_colored_mesh2d)
            .add_system_to_stage(RenderStage::Queue, queue_marker_uniform_bind_group)
            .add_system_to_stage(RenderStage::Queue, queue_colored_mesh2d);
    }
}

#[allow(clippy::too_many_arguments)]
fn queue_colored_mesh2d(
    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
    colored_mesh2d_pipeline: Res<MarkerMesh2dPipeline>,
    mut pipelines: ResMut<SpecializedMeshPipelines<MarkerMesh2dPipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    shader_handle: Res<MarkerShaderHandle>,
    colored_mesh2d: Query<(Entity, &Mesh2dHandle, &Mesh2dUniform), With<MarkerInstanceMatData>>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent2d>)>,
) {
    if colored_mesh2d.is_empty() {
        return;
    }

    // Iterate each view (a camera is a view)
    // for (visible_entities, mut transparent_phase) in views.iter_mut() {
    for (_view, mut transparent_phase) in views.iter_mut() {
        let draw_colored_mesh2d = transparent_draw_functions.read().get_id::<DrawMarkerMesh2d>().unwrap();

        // let draw_colored_mesh2d = transparent_draw_functions.read().id::<DrawMarkerMesh2d>();

        // let mesh_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples);

        let mesh_key = MarkerPipelineKey {
            mesh: Mesh2dPipelineKey::from_msaa_samples(msaa.samples),
            shader_handle: shader_handle.0.clone(),
        };

        // let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples());

        // Queue all entities visible to that view
        // for visible_entity in &visible_entities.entities {
        for (entity, mesh2d_handle, mesh2d_uniform) in colored_mesh2d.iter() {
            // if let Ok((mesh2d_handle, mesh2d_uniform)) = colored_mesh2d.get(*visible_entity) {
            let mut mesh2d_key = mesh_key.clone();
            if let Some(mesh) = render_meshes.get(&mesh2d_handle.0) {
                mesh2d_key.mesh |= Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology);

                if let Ok(pipeline_id) = pipelines.specialize(
                    &mut pipeline_cache,
                    &colored_mesh2d_pipeline,
                    mesh2d_key,
                    &mesh.layout.clone(),
                ) {
                    let mesh_z = mesh2d_uniform.transform.w_axis.z;
                    transparent_phase.add(Transparent2d {
                        entity,
                        draw_function: draw_colored_mesh2d,
                        pipeline: pipeline_id,
                        sort_key: FloatOrd(mesh_z),
                        batch_range: None,
                    });
                }
            }
            // }
        }
    }
}

fn prepare_instance_buffers(
    mut commands: Commands,
    query: Query<(Entity, &MarkerInstanceMatData)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instance_data) in query.iter() {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("marker instance data buffer"),
            contents: bytemuck::cast_slice(instance_data.0.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        commands.entity(entity).insert(MarkerInstanceBuffer {
            buffer,
            length: instance_data.0.len(),
        });
    }
}

struct MarkerUniformBindGroup {
    pub value: BindGroup,
}

fn queue_marker_uniform_bind_group(
    mut commands: Commands,
    mesh2d_pipeline: Res<MarkerMesh2dPipeline>,
    render_device: Res<RenderDevice>,
    // mesh2d_uniforms: Res<MarkerUniform>,
    mesh2d_uniforms: Res<ComponentUniforms<MarkerUniform>>,
) {
    if let Some(binding) = mesh2d_uniforms.uniforms().binding() {
        commands.insert_resource(MarkerUniformBindGroup {
            value: render_device.create_bind_group(&BindGroupDescriptor {
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: binding,
                }],
                label: Some("MarkersUniform_bind_group"),
                layout: &mesh2d_pipeline.custom_uniform_layout,
            }),
        });
    }
}

struct SetMarkerUniformBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetMarkerUniformBindGroup<I> {
    type Param = (
        SRes<MarkerUniformBindGroup>,
        SQuery<Read<DynamicUniformIndex<MarkerUniform>>>,
    );
    #[inline]
    fn render<'w>(
        _view: Entity,
        item: Entity,
        (mesh2d_bind_group, mesh2d_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let mesh2d_index = mesh2d_query.get(item).unwrap();

        pass.set_bind_group(I, &mesh2d_bind_group.into_inner().value, &[mesh2d_index.index()]);
        RenderCommandResult::Success
    }
}

#[derive(Component)]
struct MarkerInstanceBuffer {
    buffer: Buffer,
    length: usize,
}

struct DrawMarkerMeshInstanced;
impl EntityRenderCommand for DrawMarkerMeshInstanced {
    type Param = (
        SRes<RenderAssets<Mesh>>,
        SQuery<Read<Mesh2dHandle>>,
        SQuery<Read<MarkerInstanceBuffer>>,
    );

    //     type Param = SRes<RenderAssets<Mesh>>;
    // type ViewWorldQuery = ();
    // type ItemWorldQuery = (Read<Handle<Mesh>>, Read<InstanceBuffer>);

    #[inline]
    fn render<'w>(
        _view: Entity,
        item: Entity,
        (meshes, mesh_query, instance_buffer_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let mesh_handle = &mesh_query.get(item).unwrap().0;
        let instance_buffer = instance_buffer_query.get_inner(item).unwrap();

        let gpu_mesh = match meshes.into_inner().get(mesh_handle) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..instance_buffer.length as u32);
            }
            GpuBufferInfo::NonIndexed { vertex_count } => {
                pass.draw(0..*vertex_count, 0..instance_buffer.length as u32);
            }
        }
        RenderCommandResult::Success
    }
}
