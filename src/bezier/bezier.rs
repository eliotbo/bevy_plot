use bevy::{
    core::FloatOrd,
    core_pipeline::Transparent2d,
    ecs::system::lifetimeless::{Read, SQuery, SRes},
    ecs::system::SystemParamItem,
    prelude::*,
    // reflect::TypeUuid,
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
        texture::BevyDefault,
        texture::GpuImage,
        view::VisibleEntities,
        RenderApp, RenderStage,
    },
    sprite::{
        DrawMesh2d, Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, Mesh2dUniform,
        SetMesh2dBindGroup, SetMesh2dViewBindGroup,
    },
};

// use crate::canvas::*;
// use crate::inputs::*;

use crate::plot::*;

use crate::util::*;

use itertools_num::linspace;

#[derive(Copy, Clone, Debug)]
struct Line(Vec2, Vec2);

impl Line {
    // finding the intersection of two lines
    pub fn intersect(self, other: Self) -> Option<Vec2> {
        let a1 = self.1.y - self.0.y;
        let b1 = self.0.x - self.1.x;
        let c1 = a1 * self.0.x + b1 * self.0.y;

        let a2 = other.1.y - other.0.y;
        let b2 = other.0.x - other.1.x;
        let c2 = a2 * other.0.x + b2 * other.0.y;

        let delta = a1 * b2 - a2 * b1;

        if delta == 0.0 {
            return None;
        }

        Some(Vec2::new(
            (b2 * c1 - b1 * c2) / delta,
            (a1 * c2 - a2 * c1) / delta,
        ))
    }
}

// Compute derivatives at each point
pub(crate) fn make_df(xs: &Vec<f32>, time: f32, f: &fn(f32, f32) -> f32) -> (Vec<Vec2>, Vec<Vec2>) {
    let delta = (xs[1] - xs[0]) / 1000.0;

    // derivatives
    let dfs = xs
        .iter()
        .map(|x| Vec2::new(1.0, (f(x + delta, time) - f(x - delta, time)) / delta / 2.0))
        .collect::<Vec<Vec2>>();

    // normals
    let ns = dfs
        .iter()
        .map(|q| Vec2::new(q.y, -q.x).normalize())
        .collect::<Vec<Vec2>>();

    return (dfs, ns);

    // let df0 = (f(xs[1]) - f(xs[0])) / (xs[1] - xs[0]);
    // let mut dfs = vec![df0];
    // for i in 1..xs.len() - 1 {
    //     let x_m1 = xs[i - 1];
    //     // let x0 = xs[i];
    //     let x1 = xs[i + 1];
    //     let dfi = (f(x1) - f(x_m1)) / (x1 - x_m1);

    //     dfs.push(dfi);
    // }

    // // for the first and last points, we need to extrapolate the first derivative using the second derivative
    // dfs[0] = dfs[1] - (xs[1] - xs[0]) * (dfs[2] - dfs[1]) / (xs[2] - xs[1]);

    // let la = xs.len() - 1;
    // let df_final = dfs[la - 1]
    //     - (xs[la - 1] - xs[la]) * (dfs[la - 2] - dfs[la - 1]) / (xs[la - 2] - xs[la - 1]);

    // dfs.push(df_final);

    // // derivatives
    // let dfs_vec2 = dfs
    //     .iter()
    //     .map(|q| Vec2::new(1.0, *q).normalize())
    //     .collect::<Vec<Vec2>>();

    // // normals
    // let ns_vec2 = dfs
    //     .iter()
    //     .map(|q| Vec2::new(*q, -1.0).normalize())
    //     .collect::<Vec<Vec2>>();

    // return (dfs_vec2, ns_vec2);
}

#[derive(Component, Clone, AsStd140)]
pub struct BezierCurveUniform {
    pub mech: f32,
    pub zoom: f32,
    pub dummy: f32,
    pub inner_canvas_size_in_pixels: Vec2,
    pub canvas_position_in_pixels: Vec2,
    pub color: Vec4,
    pub size: f32,
    pub style: i32,
}

pub fn update_bezier_uniform(
    mut plots: ResMut<Assets<Plot>>,
    mut bez_events: EventReader<UpdateBezierShaderEvent>,
    mut query: Query<(&Handle<Plot>, &mut BezierCurveUniform)>,
) {
    for event in bez_events.iter() {
        if let Ok(query_mut) = query.get_mut(event.entity) {
            let (plot_handle, mut bezier_uniform) = query_mut;

            let plot = plots.get_mut(plot_handle).unwrap();
            plot.compute_zeros();

            let bezier_curve = &plot.data.bezier_groups[event.group_number];

            let bez_uni = bezier_uniform.as_mut();
            *bez_uni = BezierCurveUniform {
                mech: if bezier_curve.mech { 1.0 } else { 0.0 },
                dummy: plot.bezier_dummy,
                zoom: plot.zoom,
                inner_canvas_size_in_pixels: plot.canvas_size / (1.0 + plot.outer_border),
                canvas_position_in_pixels: plot.canvas_position,
                color: col_to_vec4(bezier_curve.color),
                size: bezier_curve.size,
                style: bezier_curve.line_style.clone().to_int32(),
            };
        }
    }
}

pub struct SpawnBezierCurveEvent {
    pub group_number: usize,
    pub plot_handle: Handle<Plot>,
}

pub fn spawn_bezier_function(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut plots: ResMut<Assets<Plot>>,
    mut spawn_beziercurve_event: EventReader<SpawnBezierCurveEvent>,
    // mut change_canvas_material_event: EventReader<RespawnAllEvent>,
    // mut change_canvas_material_event: EventReader<RespawnAllEvent>,
    query: Query<(Entity, &BezierCurveUniform, &BezierCurveNumber)>,
    time: Res<Time>,
) {
    // for event in spawn_beziercurve_event.iter() {
    for event in spawn_beziercurve_event.iter() {
        //
        if let Some(mut plot) = plots.get_mut(event.plot_handle.clone()) {
            //
            // remove all the bezier curves
            // TODO: currently runs proportionally to curve_number^2. Optimize
            for (entity, _bez_uni, curve_number) in query.iter() {
                if curve_number.0 == event.group_number {
                    commands.entity(entity).despawn();
                }
            }

            let num_pts = plot.bezier_num_points;
            let xs_linspace = linspace(plot.bounds.lo.x, plot.bounds.up.x, num_pts);
            let xs = xs_linspace.into_iter().collect::<Vec<f32>>();

            plot_fn(
                &mut commands,
                &mut meshes,
                xs,
                event.group_number,
                &mut plot,
                &event.plot_handle,
                &time,
            );
        }
    }
}

pub fn animate_bezier(
    mut event: EventWriter<SpawnBezierCurveEvent>,
    plots: Res<Assets<Plot>>,
    query: Query<(&Handle<Plot>, &BezierCurveNumber)>,
) {
    for (plot_handle, curve_number) in query.iter() {
        if let Some(plot) = plots.get(plot_handle) {
            if let Some(bezier_curve) = plot.data.bezier_groups.get(curve_number.0) {
                if bezier_curve.show_animation {
                    event.send(SpawnBezierCurveEvent {
                        plot_handle: plot_handle.clone(),
                        group_number: curve_number.0,
                    });
                }
            }
        }
    }
}

pub fn plot_fn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    xs: Vec<f32>,
    curve_number: usize,
    plot: &mut Plot,
    plot_handle: &Handle<Plot>,
    time: &Res<Time>,
) {
    plot.compute_zeros();

    if let Some(bezier_curve) = plot.data.bezier_groups.get(curve_number) {
        let func = bezier_curve.function.clone();

        let num_pts = plot.bezier_num_points;

        let t = time.seconds_since_startup() as f32;
        let ys = xs
            .iter()
            .map(|x| Vec2::new(*x, func(*x, t)))
            .collect::<Vec<Vec2>>();

        let ys_world = ys.iter().map(|y| plot.to_local(*y)).collect::<Vec<Vec2>>();

        let (dys, _) = make_df(&xs, t, &func);

        let dys_p_ys = dys
            .iter()
            .zip(ys.iter())
            .map(|(dy, y)| *dy + *y)
            .collect::<Vec<Vec2>>();

        // let dys_p_ys_world = plot.plot_to_local(&dys_p_ys);
        let dys_p_ys_world = dys_p_ys
            .iter()
            .map(|y| plot.to_local(*y))
            .collect::<Vec<Vec2>>();

        let mut ends = Vec::new();

        let mut mesh_attr_uvs: Vec<[f32; 2]> = Vec::new();
        let mut mesh_attr_controls: Vec<[f32; 4]> = Vec::new();

        let mut mesh0 = vec![];
        let mut inds: Vec<u32> = vec![];

        let mut controls = Vec::new();
        let mut kk = 0;

        let bounds_world = plot.compute_bounds_world();

        let line_width = 30.0;
        for k in 0..num_pts - 1 {
            // TODO: Figure out what quadt-offset does
            let quadt_offset = line_width * 10.0;

            mesh0.push(Vec2::new(ys_world[k].x - quadt_offset, bounds_world.up.y));
            mesh0.push(Vec2::new(ys_world[k].x - quadt_offset, bounds_world.lo.y));

            mesh0.push(Vec2::new(
                ys_world[k + 1].x + quadt_offset,
                bounds_world.up.y,
            ));
            mesh0.push(Vec2::new(
                ys_world[k + 1].x + quadt_offset,
                bounds_world.lo.y,
            ));

            mesh_attr_uvs.push([ys_world[k].x - quadt_offset, bounds_world.up.y]);
            mesh_attr_uvs.push([ys_world[k].x - quadt_offset, bounds_world.lo.y]);
            mesh_attr_uvs.push([ys_world[k + 1].x + quadt_offset, bounds_world.up.y]);
            mesh_attr_uvs.push([ys_world[k + 1].x + quadt_offset, bounds_world.lo.y]);

            let ki = kk * 4;

            inds.push(ki as u32);
            inds.push((ki + 1) as u32);
            inds.push((ki + 2) as u32);

            inds.push((ki + 3) as u32);
            inds.push((ki + 2) as u32);
            inds.push((ki + 1) as u32);

            let mut is_last = 0.0;
            if k == num_pts - 2 {
                is_last = 1.0;
            }

            // if the curvature is high enough, we need to locally estimate the function
            // as a bezier curve. Else, we estimate it as a line.

            // if the angle between the two tangents is greater than 10 degrees,
            // we use a bezier curve (cos(3 degrees) ~= 0.0.9986)))

            if (dys[k].normalize().dot(dys[k + 1].normalize())).abs() < 0.9986 {
                let line0 = Line(ys_world[k], dys_p_ys_world[k]);
                let line1 = Line(ys_world[k + 1], dys_p_ys_world[k + 1]);
                let intersection = line1.intersect(line0).unwrap();

                let control_point = intersection;

                mesh_attr_controls.push([control_point.x, control_point.y, is_last, is_last]);
                mesh_attr_controls.push([control_point.x, control_point.y, is_last, is_last]);
                mesh_attr_controls.push([control_point.x, control_point.y, is_last, is_last]);
                mesh_attr_controls.push([control_point.x, control_point.y, is_last, is_last]);

                controls.push(control_point);

                ends.push([
                    ys_world[k].x,
                    ys_world[k].y,
                    ys_world[k + 1].x,
                    ys_world[k + 1].y,
                ]);
                ends.push([
                    ys_world[k].x,
                    ys_world[k].y,
                    ys_world[k + 1].x,
                    ys_world[k + 1].y,
                ]);
                ends.push([
                    ys_world[k].x,
                    ys_world[k].y,
                    ys_world[k + 1].x,
                    ys_world[k + 1].y,
                ]);
                ends.push([
                    ys_world[k].x,
                    ys_world[k].y,
                    ys_world[k + 1].x,
                    ys_world[k + 1].y,
                ]);
            } else {
                // line
                controls.push(Vec2::new(0.50001, 0.00001));

                mesh_attr_controls.push([ys_world[k].x, ys_world[k].y, is_last, is_last]);
                mesh_attr_controls.push([ys_world[k].x, ys_world[k].y, is_last, is_last]);
                mesh_attr_controls.push([ys_world[k + 1].x, ys_world[k + 1].y, is_last, is_last]);
                mesh_attr_controls.push([ys_world[k + 1].x, ys_world[k + 1].y, is_last, is_last]);

                ends.push([
                    ys_world[k].x,
                    ys_world[k].y,
                    ys_world[k + 1].x,
                    ys_world[k + 1].y,
                ]);
                ends.push([
                    ys_world[k].x,
                    ys_world[k].y,
                    ys_world[k + 1].x,
                    ys_world[k + 1].y,
                ]);
                ends.push([
                    ys_world[k].x,
                    ys_world[k].y,
                    ys_world[k + 1].x,
                    ys_world[k + 1].y,
                ]);
                ends.push([
                    ys_world[k].x,
                    ys_world[k].y,
                    ys_world[k + 1].x,
                    ys_world[k + 1].y,
                ]);
            }
            kk = kk + 1;
        }

        let mut mesh_pos_attributes: Vec<[f32; 3]> = Vec::new();

        for position in mesh0 {
            mesh_pos_attributes.push([position.x, position.y, 0.0]);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, mesh_pos_attributes.clone());

        mesh.set_attribute("Ends", ends);

        mesh.set_indices(Some(Indices::U32(inds)));

        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, mesh_attr_uvs);

        mesh.set_attribute("Vertext_Control", mesh_attr_controls);

        commands
            .spawn_bundle((
                BezierMesh2d::default(),
                Mesh2dHandle(meshes.add(mesh)),
                GlobalTransform::default(),
                Transform::from_translation(plot.canvas_position.extend(1.10)),
                // Transform::from_translation(Vec3::new(0.0, 0.0, 3.0)),
                Visibility::default(),
                ComputedVisibility::default(),
            ))
            .insert(BezierCurveNumber(curve_number))
            .insert(plot_handle.clone())
            .insert(BezierCurveUniform {
                mech: if bezier_curve.mech { 1.0 } else { 0.0 },
                dummy: plot.bezier_dummy,
                zoom: plot.zoom,
                inner_canvas_size_in_pixels: plot.canvas_size / (1.0 + plot.outer_border),
                canvas_position_in_pixels: plot.canvas_position,
                color: col_to_vec4(bezier_curve.color),
                size: bezier_curve.size,
                style: bezier_curve.line_style.clone().to_int32(),
            });
    }
}

/// Component inserted in the entity corresponding to the kth curve group in Plot.data.bezier_groups.
#[derive(Component)]
pub struct BezierCurveNumber(pub usize);

/// A marker component for colored 2d meshes
#[derive(Component, Default)]
pub(crate) struct BezierMesh2d;

struct BezierMesh2dPipeline {
    pub view_layout: BindGroupLayout,
    pub mesh_layout: BindGroupLayout,
    pub custom_uniform_layout: BindGroupLayout,

    // This dummy white texture is to be used in place of optional textures
    #[allow(dead_code)]
    pub dummy_white_gpu_image: GpuImage,
    pub shader_handle: Handle<Shader>,
}

impl FromWorld for BezierMesh2dPipeline {
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
                            BezierCurveUniform::std140_size_static() as u64
                        ),
                    },
                    count: None,
                }],
                label: Some("custom_uniform_layout"),
            });

        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let shader_handle = asset_server.load("shaders/bezier_spline.wgsl");

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
impl SpecializedPipeline for BezierMesh2dPipeline {
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
            // Color ----> not truly color. It's actually Ends in the shader, but I am too tired to change it right now.
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
type DrawBezierMesh2d = (
    // Set the pipeline
    SetItemPipeline,
    // Set the view uniform as bind group 0
    SetMesh2dViewBindGroup<0>,
    // Set the mesh uniform as bind group 1
    SetMesh2dBindGroup<1>,
    SetBezierCurveUniformBindGroup<2>,
    // Draw the mesh
    DrawMesh2d,
);

/// Plugin that renders [`BezierMesh2d`]s
pub struct BezierMesh2dPlugin;

impl Plugin for BezierMesh2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UniformComponentPlugin::<BezierCurveUniform>::default());

        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        render_app
            .add_render_command::<Transparent2d, DrawBezierMesh2d>()
            .init_resource::<BezierMesh2dPipeline>()
            .init_resource::<SpecializedPipelines<BezierMesh2dPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_colored_mesh2d)
            .add_system_to_stage(RenderStage::Queue, queue_customuniform_bind_group)
            .add_system_to_stage(RenderStage::Queue, queue_colored_mesh2d);
    }
}

fn extract_colored_mesh2d(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    query: Query<(Entity, &BezierCurveUniform, &ComputedVisibility), With<BezierMesh2d>>,
) {
    let mut values = Vec::with_capacity(*previous_len);
    for (entity, custom_uni, computed_visibility) in query.iter() {
        if !computed_visibility.is_visible {
            continue;
        }
        values.push((entity, (custom_uni.clone(), BezierMesh2d)));
    }
    *previous_len = values.len();
    commands.insert_or_spawn_batch(values);
}

/// I can't make this private because it's tied to BezierCurveUniform, which is public
pub struct BezierCurveUniformBindGroup {
    pub value: BindGroup,
}

fn queue_customuniform_bind_group(
    mut commands: Commands,
    mesh2d_pipeline: Res<BezierMesh2dPipeline>,
    render_device: Res<RenderDevice>,
    mesh2d_uniforms: Res<ComponentUniforms<BezierCurveUniform>>,
) {
    if let Some(binding) = mesh2d_uniforms.uniforms().binding() {
        // println!("binding: {:?}", binding);

        commands.insert_resource(BezierCurveUniformBindGroup {
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
    colored_mesh2d_pipeline: Res<BezierMesh2dPipeline>,
    mut pipelines: ResMut<SpecializedPipelines<BezierMesh2dPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    colored_mesh2d: Query<(&Mesh2dHandle, &Mesh2dUniform), With<BezierMesh2d>>,
    mut views: Query<(&VisibleEntities, &mut RenderPhase<Transparent2d>)>,
) {
    if colored_mesh2d.is_empty() {
        return;
    }
    // Iterate each view (a camera is a view)
    for (visible_entities, mut transparent_phase) in views.iter_mut() {
        let draw_colored_mesh2d = transparent_draw_functions
            .read()
            .get_id::<DrawBezierMesh2d>()
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

pub struct SetBezierCurveUniformBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetBezierCurveUniformBindGroup<I> {
    type Param = (
        SRes<BezierCurveUniformBindGroup>,
        SQuery<Read<DynamicUniformIndex<BezierCurveUniform>>>,
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
