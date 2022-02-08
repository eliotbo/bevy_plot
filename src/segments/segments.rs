use bevy::{
    core::FloatOrd,
    core_pipeline::Transparent2d,
    ecs::system::lifetimeless::{Read, SQuery, SRes},
    ecs::system::SystemParamItem,
    prelude::*,
    // reflect::TypeUuid,
    render::{
        mesh::GpuBufferInfo,
        mesh::Indices,
        render_asset::RenderAssets,
        render_component::{ComponentUniforms, DynamicUniformIndex, UniformComponentPlugin},
        render_component::{ExtractComponent, ExtractComponentPlugin},
        render_phase::{
            AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
            SetItemPipeline, TrackedRenderPass,
        },
        render_resource::{std140::AsStd140, *},
        renderer::RenderDevice,
        // texture::BevyDefault,
        // texture::GpuImage,
        view::VisibleEntities,
        RenderApp,
        RenderStage,
    },
    sprite::{
        DrawMesh2d, Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, Mesh2dUniform,
        SetMesh2dBindGroup, SetMesh2dViewBindGroup,
    },
};

use bytemuck::{Pod, Zeroable};
// use crate::canvas::*;
use crate::inputs::*;

use crate::plot::*;

use crate::canvas::ChangeCanvasMaterialEvent;
use crate::util::*;

// use flo_curves::*;
// use itertools_num::linspace;

pub fn change_segment_uni(
    mut query: Query<&mut SegmentUniform>,
    mouse_position: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for mut segment_uni in query.iter_mut() {
        let mouse_pos = mouse_position.position;

        if mouse_button_input.pressed(MouseButton::Left) {
            segment_uni.segment_size = mouse_pos.x / 100.0;
            // println!("left: {}, right: {}", segment_uni.left, segment_uni.mech);
        } else if mouse_button_input.pressed(MouseButton::Right) {
            segment_uni.hole_size = mouse_pos.x / 100.0;
            // segment_uni.ya.x = mouse_pos.x / 100.0;
            // segment_uni.ya.y = mouse_pos.y / 100.0;
            println!(
                "left: {}, right: {}",
                segment_uni.segment_size, segment_uni.hole_size
            );
        }
    }
}

pub fn segments_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut change_canvas_material_event: EventReader<ChangeCanvasMaterialEvent>,
    mut plots: ResMut<Assets<Plot>>,
    query: Query<(Entity, &Handle<Plot>), With<SegmentUniform>>,
) {
    for event in change_canvas_material_event.iter() {
        //
        for (entity, plot_handle) in query.iter() {
            if event.plot_handle == *plot_handle {
                commands.entity(entity).despawn();
            }
        }

        let mut plot = plots.get_mut(&event.plot_handle).unwrap();

        plot_segments(
            &mut commands,
            &mut meshes,
            // ys,
            &mut plot,
            &event.plot_handle,
        )
    }
}

// Compute derivatives at each point
pub fn make_df(ys: &Vec<Vec2>) -> (Vec<Vec2>, Vec<Vec2>) {
    let df0 = (ys[1].y - ys[0].y) / (ys[1].x - ys[0].x);
    let mut dfs = vec![df0];
    for i in 1..ys.len() - 1 {
        let y_m1 = ys[i - 1];
        // let x0 = ys[i];
        let y1 = ys[i + 1];
        let dfi = (y1.y - y_m1.y) / (y1.x - y_m1.x);

        dfs.push(dfi);
    }

    // for the first and last points, we need to extrapolate the first derivative using the second derivative
    dfs[0] = dfs[1] - (ys[1].x - ys[0].x) * (dfs[2] - dfs[1]) / (ys[2].x - ys[1].x);

    let la = ys.len() - 1;
    let df_final = dfs[la - 1]
        - (ys[la - 1].x - ys[la].x) * (dfs[la - 2] - dfs[la - 1]) / (ys[la - 2].x - ys[la - 1].x);

    dfs.push(df_final);

    // derivatives
    let dfs_vec2 = dfs
        .iter()
        .map(|q| Vec2::new(1.0, *q).normalize())
        .collect::<Vec<Vec2>>();

    // normals
    let ns_vec2 = dfs
        .iter()
        .map(|q| Vec2::new(*q, -1.0).normalize())
        .collect::<Vec<Vec2>>();

    return (dfs_vec2, ns_vec2);
}

pub fn plot_segments(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    plot: &mut Plot,
    plot_handle: &Handle<Plot>,
) {
    let data = plot.data.clone();
    plot.compute_zeros();

    for segment_plot in data.segment_groups.iter() {
        let ys = segment_plot.data.clone();

        // derivatives and normals
        let (dfs, ns) = make_df(&ys);
        println!("dfs: {:?}", ns);

        let num_pts = ys.len();

        let ys_world = ys.iter().map(|y| plot.to_world(*y)).collect::<Vec<Vec2>>();

        let quad_size = 30.0;

        let mut mesh0 = Vec::new();
        let mut mesh_attr_uvs = Vec::new();
        let mut inds = Vec::new();

        // TODO
        let line_width = 30.0;
        for k in 0..num_pts - 1 {
            // let quadt_offset = line_width * 1.0;

            // let p0 = Vec2::new(ys_world[k].x - quadt_offset, ys_world[k].y + quadt_offset);
            // let p1 = Vec2::new(ys_world[k].x - quadt_offset, ys_world[k].y - quadt_offset);
            // let p2 = Vec2::new(
            //     ys_world[k + 1].x + quadt_offset,
            //     ys_world[k].y + quadt_offset,
            // );
            // let p3 = Vec2::new(
            //     ys_world[k + 1].x + quadt_offset,
            //     ys_world[k].y - quadt_offset,
            // );

            let y0 = ys_world[k];
            let y1 = ys_world[k + 1];

            // let theta = (y1.x - y0.x).atan2(y1.y - y0.y);

            // let dy = (y1 - y0).normalize();
            // let n = Vec2::new(-dy.y, dy.x);
            let n0 = -ns[k];
            let n1 = -ns[k + 1];

            let p0 = y0 + n0 * line_width;
            let p1 = y0 - n0 * line_width;
            let p2 = y1 + n1 * line_width;
            let p3 = y1 - n1 * line_width;

            // let r = 50.0;
            // let p0 = Vec2::new(-r, -r);
            // let p1 = Vec2::new(-r, r);
            // let p2 = Vec2::new(r, r);
            // let p3 = Vec2::new(r, -r);

            mesh0.push(p0);
            mesh0.push(p1);
            mesh0.push(p2);
            mesh0.push(p3);

            mesh_attr_uvs.push([p0.x, p0.y]);
            mesh_attr_uvs.push([p1.x, p1.y]);
            mesh_attr_uvs.push([p2.x, p2.y]);
            mesh_attr_uvs.push([p3.x, p3.y]);

            let ki = k * 4;

            inds.push(ki as u32);
            inds.push((ki + 1) as u32);
            inds.push((ki + 2) as u32);

            inds.push((ki + 3) as u32);
            inds.push((ki + 2) as u32);
            inds.push((ki + 1) as u32);
        }

        let mut mesh_pos_attributes: Vec<[f32; 3]> = Vec::new();
        let mut normals = Vec::new();
        // TODO: z position is here
        for position in mesh0 {
            mesh_pos_attributes.push([position.x, position.y, -30.0]);
            normals.push([0.0, 0.0, 1.0]);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, mesh_pos_attributes.clone());
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals.clone());

        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, mesh_attr_uvs);
        mesh.set_indices(Some(Indices::U32(inds)));

        commands
            .spawn_bundle((
                Mesh2dHandle(meshes.add(mesh)),
                // Mesh2dHandle(meshes.add(Mesh::from(shape::Quad {
                //     size: Vec2::splat(50.0),
                //     flip: false,
                // }))),
                GlobalTransform::default(),
                Transform::from_translation(Vec3::new(0.0, 0.0, 3.0) + plot.position.extend(0.0)),
                Visibility::default(),
                ComputedVisibility::default(),
                SegmentInstanceMatData(
                    ys_world
                        .iter()
                        .map(|v| SegmentInstanceData {
                            //
                            // TODO: take inner border into account
                            //
                            // position: Vec3::new(v.x, v.y, 20.0) + plot.position.extend(0.0),
                            position: Vec3::new(0.0, 0.0, 20.0) + plot.position.extend(0.0),
                            scale: 1.0,
                            color: Color::rgba(0.8, 0.6, 0.1, 1.0).as_rgba_f32(),
                        })
                        .collect(),
                ),
                // NoFrustumCulling,
            ))
            .insert(plot_handle.clone())
            .insert(SegmentUniform {
                segment_size: segment_plot.size,
                hole_size: 1.0,
                zoom: 1.0,

                segment_point_color: col_to_vec4(segment_plot.segment_point_color),
                color: col_to_vec4(segment_plot.color),
                quad_size,
                inner_canvas_size_in_pixels: plot.size / (1.0 + plot.outer_border),
                canvas_position: plot.position,
                contour: if segment_plot.draw_contour { 1.0 } else { 0.0 },
            });
    }
}

#[derive(Component)]
pub struct SegmentInstanceMatData(Vec<SegmentInstanceData>);
impl ExtractComponent for SegmentInstanceMatData {
    type Query = &'static SegmentInstanceMatData;
    type Filter = ();

    fn extract_component(item: bevy::ecs::query::QueryItem<Self::Query>) -> Self {
        SegmentInstanceMatData(item.0.clone())
    }
}

/// A segment component for colored 2d meshes
#[derive(Component, Default)]
pub struct SegmentMesh2d;

#[derive(Clone, AsStd140)]
pub struct BoundsWorld {
    bx: Vec2,
    by: Vec2,
}

#[derive(Component, Clone, AsStd140)]
pub struct SegmentUniform {
    pub segment_size: f32,
    pub hole_size: f32,
    pub zoom: f32,
    pub quad_size: f32,
    pub contour: f32,
    pub inner_canvas_size_in_pixels: Vec2,
    pub canvas_position: Vec2,
    pub color: Vec4,
    pub segment_point_color: Vec4,
}

// TODO: we have instance data, but we don't use it at the moment.
// One use case would be to have segment size as an additional dimension.

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct SegmentInstanceData {
    position: Vec3,
    scale: f32,
    color: [f32; 4],
}

/// Custom pipeline for 2d meshes with vertex colors
pub struct SegmentMesh2dPipeline {
    /// this pipeline wraps the standard [`Mesh2dPipeline`]
    mesh2d_pipeline: Mesh2dPipeline,
    pub segment_uniform_layout: BindGroupLayout,
    pub shader: Handle<Shader>,
    // material_layout: BindGroupLayout,
}

impl FromWorld for SegmentMesh2dPipeline {
    fn from_world(world: &mut World) -> Self {
        let mesh2d_pipeline = Mesh2dPipeline::from_world(world).clone();

        let render_device = world.get_resource::<RenderDevice>().unwrap();

        let segment_uniform_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: BufferSize::new(
                            SegmentUniform::std140_size_static() as u64
                        ),
                    },
                    count: None,
                }],
                label: Some("segments_uniform_layout"),
            });

        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let shader = asset_server.load("shaders/segments.wgsl");

        let _result = asset_server.watch_for_changes();

        Self {
            mesh2d_pipeline,
            segment_uniform_layout,

            shader,
        }
    }
}

impl SpecializedPipeline for SegmentMesh2dPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh2d_pipeline.specialize(key);

        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<SegmentInstanceData>() as u64,
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
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        descriptor.layout = Some(vec![
            self.mesh2d_pipeline.view_layout.clone(),
            self.mesh2d_pipeline.mesh_layout.clone(),
            self.segment_uniform_layout.clone(),
        ]);

        descriptor
    }
}

// This specifies how to render a colored 2d mesh
type DrawSegmentMesh2d = (
    // Set the pipeline
    SetItemPipeline,
    // Set the view uniform as bind group 0
    SetMesh2dViewBindGroup<0>,
    // Set the mesh uniform as bind group 1
    SetMesh2dBindGroup<1>,
    // Set the segment uniform as bind group 2
    SetSegmentUniformBindGroup<2>,
    // Draw the mesh
    DrawSegmentMeshInstanced,
);

pub struct SegmentMesh2dPlugin;

impl Plugin for SegmentMesh2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UniformComponentPlugin::<SegmentUniform>::default());
        app.add_plugin(ExtractComponentPlugin::<SegmentInstanceMatData>::default());

        // Register our custom draw function and pipeline, and add our render systems
        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        render_app
            .add_render_command::<Transparent2d, DrawSegmentMesh2d>()
            .init_resource::<SegmentMesh2dPipeline>()
            .init_resource::<SpecializedPipelines<SegmentMesh2dPipeline>>()
            .add_system_to_stage(RenderStage::Prepare, prepare_instance_buffers)
            .add_system_to_stage(RenderStage::Extract, extract_colored_mesh2d)
            .add_system_to_stage(RenderStage::Queue, queue_segment_uniform_bind_group)
            .add_system_to_stage(RenderStage::Queue, queue_segment_mesh2d);
    }
}

/// Extract SegmentUniform
pub fn extract_colored_mesh2d(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    query: Query<(Entity, &SegmentUniform, &ComputedVisibility), With<SegmentInstanceMatData>>,
) {
    let mut values = Vec::with_capacity(*previous_len);
    for (entity, segment_uni, computed_visibility) in query.iter() {
        if !computed_visibility.is_visible {
            continue;
        }
        values.push((entity, (segment_uni.clone(), SegmentMesh2d)));
    }
    *previous_len = values.len();
    commands.insert_or_spawn_batch(values);
}

fn prepare_instance_buffers(
    mut commands: Commands,
    query: Query<(Entity, &SegmentInstanceMatData)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instance_data) in query.iter() {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("segment instance data buffer"),
            contents: bytemuck::cast_slice(instance_data.0.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        commands.entity(entity).insert(SegmentInstanceBuffer {
            buffer,
            length: instance_data.0.len(),
        });
    }
}

pub struct SegmentUniformBindGroup {
    pub value: BindGroup,
}

pub fn queue_segment_uniform_bind_group(
    mut commands: Commands,
    mesh2d_pipeline: Res<SegmentMesh2dPipeline>,
    render_device: Res<RenderDevice>,
    mesh2d_uniforms: Res<ComponentUniforms<SegmentUniform>>,
) {
    if let Some(binding) = mesh2d_uniforms.uniforms().binding() {
        commands.insert_resource(SegmentUniformBindGroup {
            value: render_device.create_bind_group(&BindGroupDescriptor {
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: binding,
                }],
                label: Some("SegmentsUniform_bind_group"),
                layout: &mesh2d_pipeline.segment_uniform_layout,
            }),
        });
    }
}

#[allow(clippy::too_many_arguments)]
pub fn queue_segment_mesh2d(
    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
    colored_mesh2d_pipeline: Res<SegmentMesh2dPipeline>,
    mut pipelines: ResMut<SpecializedPipelines<SegmentMesh2dPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    colored_mesh2d: Query<(&Mesh2dHandle, &Mesh2dUniform), With<SegmentInstanceMatData>>,
    mut views: Query<(&VisibleEntities, &mut RenderPhase<Transparent2d>)>,
) {
    if colored_mesh2d.is_empty() {
        return;
    }

    // Iterate each view (a camera is a view)
    for (visible_entities, mut transparent_phase) in views.iter_mut() {
        let draw_colored_mesh2d = transparent_draw_functions
            .read()
            .get_id::<DrawSegmentMesh2d>()
            .unwrap();

        let mesh_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples);

        // Queue all entities visible to that view
        for visible_entity in &visible_entities.entities {
            if let Ok((mesh2d_handle, mesh2d_uniform)) = colored_mesh2d.get(*visible_entity) {
                let mut mesh2d_key = mesh_key;
                if let Some(mesh) = render_meshes.get(&mesh2d_handle.0) {
                    mesh2d_key |=
                        Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology);
                }

                let pipeline_id =
                    pipelines.specialize(&mut pipeline_cache, &colored_mesh2d_pipeline, mesh2d_key);

                let mesh_z = mesh2d_uniform.transform.w_axis.z;
                transparent_phase.add(Transparent2d {
                    entity: *visible_entity,
                    draw_function: draw_colored_mesh2d,
                    pipeline: pipeline_id,
                    sort_key: FloatOrd(mesh_z),
                    batch_range: None,
                });
            }
        }
    }
}

pub struct SetSegmentUniformBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetSegmentUniformBindGroup<I> {
    type Param = (
        SRes<SegmentUniformBindGroup>,
        SQuery<Read<DynamicUniformIndex<SegmentUniform>>>,
    );
    #[inline]
    fn render<'w>(
        _view: Entity,
        item: Entity,
        (mesh2d_bind_group, mesh2d_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let mesh2d_index = mesh2d_query.get(item).unwrap();

        pass.set_bind_group(
            I,
            &mesh2d_bind_group.into_inner().value,
            &[mesh2d_index.index()],
        );
        RenderCommandResult::Success
    }
}

#[derive(Component)]
pub struct SegmentInstanceBuffer {
    buffer: Buffer,
    length: usize,
}

pub struct DrawSegmentMeshInstanced;
impl EntityRenderCommand for DrawSegmentMeshInstanced {
    type Param = (
        SRes<RenderAssets<Mesh>>,
        SQuery<Read<Mesh2dHandle>>,
        SQuery<Read<SegmentInstanceBuffer>>,
    );
    #[inline]
    fn render<'w>(
        _view: Entity,
        item: Entity,
        (meshes, mesh2d_query, instance_buffer_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let mesh_handle = &mesh2d_query.get(item).unwrap().0;
        let instance_buffer = instance_buffer_query.get(item).unwrap();

        let gpu_mesh = match meshes.into_inner().get(mesh_handle) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
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
                pass.draw_indexed(0..*vertex_count, 0, 0..instance_buffer.length as u32);
            }
        }
        RenderCommandResult::Success
    }
}
