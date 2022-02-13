use bevy::{
    core::FloatOrd,
    core_pipeline::Transparent2d,
    ecs::system::lifetimeless::{Read, SQuery, SRes},
    ecs::system::SystemParamItem,
    prelude::*,
    render::{
        mesh::Indices,
        render_asset::RenderAssets,
        render_component::{ComponentUniforms, DynamicUniformIndex, UniformComponentPlugin},
        render_phase::{
            AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
            SetItemPipeline, TrackedRenderPass,
        },
        render_resource::{std140::AsStd140, *},
        renderer::RenderDevice,
        view::VisibleEntities,
        RenderApp, RenderStage,
        {texture::BevyDefault, texture::GpuImage},
    },
    sprite::{
        DrawMesh2d, Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, Mesh2dUniform,
        SetMesh2dBindGroup, SetMesh2dViewBindGroup,
    },
};

use crate::canvas::RespawnAllEvent;
use crate::plot::*;
use crate::util::*;

// TODO: circular ends in mesh

pub(crate) fn segments_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut change_canvas_material_event: EventReader<RespawnAllEvent>,
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
fn make_df(ys: &Vec<Vec2>) -> (Vec<Vec2>, Vec<Vec2>) {
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

fn plot_segments(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    plot: &mut Plot,
    plot_handle: &Handle<Plot>,
) {
    let data = plot.data.clone();
    plot.compute_zeros();

    for segment_plot in data.segment_groups.iter() {
        let ys = segment_plot.data.clone();

        // TODO: is this still needed?
        // derivatives and normals
        let (_dfs, _ns) = make_df(&ys);

        let num_pts = ys.len();

        let ys_world = ys.iter().map(|y| plot.to_local(*y)).collect::<Vec<Vec2>>();

        let mut mesh0 = Vec::new();
        let mut mesh_attr_uvs = Vec::new();
        let mut inds = Vec::new();
        let mut ends = Vec::new();
        let mut mesh_attr_controls: Vec<[f32; 4]> = Vec::new();

        let line_width = 5.0;
        for k in 0..num_pts - 1 {
            let y0 = ys_world[k];
            let y1 = ys_world[k + 1];

            let dy = (y1 - y0).normalize();
            let n = Vec2::new(-dy.y, dy.x);

            // // short segments
            // let mut p0 = y0 + n * line_width;
            // let mut p1 = y0 - n * line_width;
            // let mut p2 = y1 + n * line_width;
            // let mut p3 = y1 - n * line_width;

            // if segment_plot.mech {
            //     p0 = y0 + n * line_width - dy * line_width * 1.0;
            //     p1 = y0 - n * line_width - dy * line_width * 1.0;
            //     p2 = y1 + n * line_width + dy * line_width * 1.0;
            //     p3 = y1 - n * line_width + dy * line_width * 1.0;
            // }

            // overlapping segments
            let p0 = y0 + n * line_width - dy * line_width * 1.0;
            let p1 = y0 - n * line_width - dy * line_width * 1.0;
            let p2 = y1 + n * line_width + dy * line_width * 1.0;
            let p3 = y1 - n * line_width + dy * line_width * 1.0;

            mesh0.push(p0);
            mesh0.push(p1);
            mesh0.push(p2);
            mesh0.push(p3);

            ends.push([y0.x, y0.y, y1.x, y1.y]);
            ends.push([y0.x, y0.y, y1.x, y1.y]);
            ends.push([y0.x, y0.y, y1.x, y1.y]);
            ends.push([y0.x, y0.y, y1.x, y1.y]);

            mesh_attr_controls.push([p0.x, p0.y, p1.x, p1.y]);
            mesh_attr_controls.push([p0.x, p0.y, p1.x, p1.y]);
            mesh_attr_controls.push([p0.x, p0.y, p1.x, p1.y]);
            mesh_attr_controls.push([p0.x, p0.y, p1.x, p1.y]);

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
        // let mut normals = Vec::new();
        // TODO: z position is here
        for position in mesh0 {
            mesh_pos_attributes.push([position.x, position.y, 0.0]);
            // normals.push([0.0, 0.0, 1.0]);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, mesh_pos_attributes.clone());
        mesh.set_attribute("Ends", ends);
        mesh.set_indices(Some(Indices::U32(inds)));
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, mesh_attr_uvs);

        mesh.set_attribute("Vertext_Control", mesh_attr_controls);

        commands
            .spawn_bundle((
                SegmentMesh2d::default(),
                Mesh2dHandle(meshes.add(mesh)),
                GlobalTransform::default(),
                Transform::from_translation(plot.canvas_position.extend(1.11)),
                Visibility::default(),
                ComputedVisibility::default(),
            ))
            .insert(plot_handle.clone())
            .insert(SegmentUniform {
                mech: if segment_plot.mech { 1.0 } else { 0.0 },
                segment_thickness: segment_plot.size,
                hole_size: 1.0,
                zoom: 1.0,
                color: col_to_vec4(segment_plot.color),
                inner_canvas_size_in_pixels: plot.canvas_size / (1.0 + plot.outer_border),
                canvas_position: plot.canvas_position,
            });
    }
}

/// A marker component for colored 2d meshes
#[derive(Component, Default)]
pub(crate) struct SegmentMesh2d;

/// Shader uniform parameters sent to segments.wgsl.
#[derive(Component, Clone, AsStd140)]
pub(crate) struct SegmentUniform {
    pub color: Vec4,
    /// gives segments a mechanical joint look if > 0.5
    pub mech: f32,
    pub segment_thickness: f32,
    /// unused
    pub hole_size: f32,
    pub zoom: f32,
    pub inner_canvas_size_in_pixels: Vec2,
    pub canvas_position: Vec2,
}

struct SegmentMesh2dPipeline {
    pub view_layout: BindGroupLayout,
    pub mesh_layout: BindGroupLayout,
    pub custom_uniform_layout: BindGroupLayout,

    // This dummy white texture is to be used in place of optional textures
    #[allow(dead_code)]
    pub dummy_white_gpu_image: GpuImage,
    pub shader_handle: Handle<Shader>,
}

impl FromWorld for SegmentMesh2dPipeline {
    fn from_world(world: &mut World) -> Self {
        let mesh2d_pipeline = Mesh2dPipeline::from_world(world).clone();

        let render_device = world.get_resource::<RenderDevice>().unwrap();

        let custom_uniform_layout =
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
                label: Some("custom_uniform_layout"),
            });

        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let shader_handle = asset_server.load("shaders/segments.wgsl");

        Self {
            view_layout: mesh2d_pipeline.view_layout,
            mesh_layout: mesh2d_pipeline.mesh_layout,
            custom_uniform_layout,
            dummy_white_gpu_image: mesh2d_pipeline.dummy_white_gpu_image,
            shader_handle,
        }
    }
}

// We implement `SpecializedPipeline` to customize the default rendering from `Mesh2dPipeline`
impl SpecializedPipeline for SegmentMesh2dPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        // Customize how to store the meshes' vertex attributes in the vertex buffer
        // Our meshes only have position and color
        let vertex_attributes = vec![
            // Position (GOTCHA! Vertex_Position isn't first in the buffer due to how Mesh sorts attributes (alphabetically))
            VertexAttribute {
                format: VertexFormat::Float32x3,
                // this offset is the size of the color attribute, which is stored first
                offset: 16,
                // position is available at location 0 in the shader
                shader_location: 0,
            },
            // Color
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 0,
                shader_location: 1,
            },
            // uv
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 28,
                shader_location: 2,
            },
            // Control Point
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 36,
                shader_location: 3,
            },
        ];
        // This is the sum of the size of position, color uv attributes (12 + 16 + 8 + 8 = 44)
        let vertex_array_stride = 52;

        RenderPipelineDescriptor {
            vertex: VertexState {
                // Use our custom shader
                shader: self.shader_handle.clone(),
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                // Use our custom vertex buffer
                buffers: vec![VertexBufferLayout {
                    array_stride: vertex_array_stride,
                    step_mode: VertexStepMode::Vertex,
                    attributes: vertex_attributes,
                }],
            },
            fragment: Some(FragmentState {
                // Use our custom shader
                shader: self.shader_handle.clone(),
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                }],
            }),
            // Use the two standard uniforms for 2d meshes
            layout: Some(vec![
                // Bind group 0 is the view uniform
                self.view_layout.clone(),
                // Bind group 1 is the mesh uniform
                self.mesh_layout.clone(),
                self.custom_uniform_layout.clone(),
                // texture
                // self.material_layout.clone(),
            ]),
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: key.primitive_topology(),
                strip_index_format: None,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("colored_mesh2d_pipeline".into()),
        }
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
    SetSegmentUniformBindGroup<2>,
    // Draw the mesh
    DrawMesh2d,
);

/// Plugin that renders [`SegmentMesh2d`]s
pub(crate) struct SegmentMesh2dPlugin;

impl Plugin for SegmentMesh2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UniformComponentPlugin::<SegmentUniform>::default());

        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        render_app
            .add_render_command::<Transparent2d, DrawSegmentMesh2d>()
            .init_resource::<SegmentMesh2dPipeline>()
            .init_resource::<SpecializedPipelines<SegmentMesh2dPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_colored_mesh2d)
            .add_system_to_stage(RenderStage::Queue, queue_customuniform_bind_group)
            .add_system_to_stage(RenderStage::Queue, queue_colored_mesh2d);
    }
}

fn extract_colored_mesh2d(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    query: Query<(Entity, &SegmentUniform, &ComputedVisibility), With<SegmentMesh2d>>,
) {
    let mut values = Vec::with_capacity(*previous_len);
    for (entity, custom_uni, computed_visibility) in query.iter() {
        if !computed_visibility.is_visible {
            continue;
        }
        values.push((entity, (custom_uni.clone(), SegmentMesh2d)));
    }
    *previous_len = values.len();
    commands.insert_or_spawn_batch(values);
}

struct SegmentUniformBindGroup {
    value: BindGroup,
}

fn queue_customuniform_bind_group(
    mut commands: Commands,
    mesh2d_pipeline: Res<SegmentMesh2dPipeline>,
    render_device: Res<RenderDevice>,
    mesh2d_uniforms: Res<ComponentUniforms<SegmentUniform>>,
) {
    if let Some(binding) = mesh2d_uniforms.uniforms().binding() {
        // println!("binding: {:?}", binding);

        commands.insert_resource(SegmentUniformBindGroup {
            value: render_device.create_bind_group(&BindGroupDescriptor {
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: binding,
                }],
                label: Some("customuniform_bind_group"),
                layout: &mesh2d_pipeline.custom_uniform_layout,
            }),
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn queue_colored_mesh2d(
    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
    colored_mesh2d_pipeline: Res<SegmentMesh2dPipeline>,
    mut pipelines: ResMut<SpecializedPipelines<SegmentMesh2dPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    colored_mesh2d: Query<(&Mesh2dHandle, &Mesh2dUniform), With<SegmentMesh2d>>,
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

struct SetSegmentUniformBindGroup<const I: usize>;
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
