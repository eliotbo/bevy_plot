use bevy::{
    // asset::Assets,
    core_pipeline::core_2d::Transparent2d,
    ecs::system::lifetimeless::{Read, SQuery, SRes},
    ecs::system::SystemParamItem,
    prelude::*,
    reflect::TypeUuid,
    // reflect::TypeUuid,
    render::{
        extract_component::{ComponentUniforms, DynamicUniformIndex, UniformComponentPlugin},
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        mesh::{Indices, MeshVertexAttribute, MeshVertexBufferLayout},
        render_asset::RenderAssets,
        render_phase::{
            AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
            SetItemPipeline, TrackedRenderPass,
        },
        render_resource::*,
        // render_resource::{
        //     BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
        //     BufferBindingType, BufferSize, PrimitiveTopology, RenderPipelineDescriptor,
        //     ShaderStages, SpecializedRenderPipeline, VertexBufferLayout, VertexFormat,
        //     VertexStepMode,
        // },
        renderer::RenderDevice,
        texture::BevyDefault,
        texture::GpuImage,
        view::VisibleEntities,
        RenderApp,
        RenderStage,
    },
    sprite::{
        DrawMesh2d, Material2d, Material2dKey, Material2dPlugin, MaterialMesh2dBundle,
        Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, Mesh2dUniform, SetMesh2dBindGroup,
        SetMesh2dViewBindGroup,
    },
    utils::FloatOrd,
};

use crevice::std140::AsStd140;

use crate::plot::*;
use crate::util::*;

use itertools_num::linspace;

const ATTRIBUTE_ENDS: MeshVertexAttribute =
    MeshVertexAttribute::new("Ends", 335119774, VertexFormat::Float32x4);

const ATTRIBUTE_CONTROL_POINT: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertext_Control", 465542875, VertexFormat::Float32x4);

// pub const BEZIER_SHADER_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(
//     bevy::render::render_resource::Shader::TYPE_UUID,
//     3363029648115043461,
// );

#[derive(Copy, Clone, Debug)]
struct Line(Vec2, Vec2);

impl Line {
    // finding the intersection of two lines. Used to get the control point of a
    // quadratic bezier curve
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

    // // Code for computing the derivatives of an array instead of a function
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

/// Uniform sent to bezier_spline.wgsl
// #[derive(Component, Clone, AsStd140, ExtractResource)]
// pub(crate) struct BezierCurveUniformCrevice {
//     /// If set to > 0.5, the curve will be split into mechanical joints, but it's just a look
//     pub mech: f32,
//     pub zoom: f32,
//     pub inner_canvas_size_in_pixels: crevice::std140::Vec2,
//     pub canvas_position_in_pixels: crevice::std140::Vec2,
//     pub color: crevice::std140::Vec4,

//     /// Curve thickness
//     pub size: f32,

//     /// unused
//     pub dummy: f32,
//     /// unused
//     pub style: i32,
// }

#[derive(TypeUuid, Component, Clone, ExtractResource, AsBindGroup, ShaderType)]
#[uuid = "968a0c66-6019-454a-a1d7-551fa42c9de4"]
pub(crate) struct BezierCurveUniform {
    /// If set to > 0.5, the curve will be split into mechanical joints, but it's just a look
    #[uniform(0)]
    pub mech: f32,
    #[uniform(0)]
    pub zoom: f32,
    #[uniform(0)]
    pub inner_canvas_size_in_pixels: Vec2,
    #[uniform(0)]
    pub canvas_position_in_pixels: Vec2,
    #[uniform(0)]
    pub color: Vec4,

    /// Curve thickness
    #[uniform(0)]
    pub size: f32,

    /// unused
    #[uniform(0)]
    pub dummy: f32,
    /// unused
    #[uniform(0)]
    pub style: i32,
}

pub(crate) fn update_bezier_uniform(
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

pub(crate) struct SpawnBezierCurveEvent {
    pub group_number: usize,
    pub plot_handle: Handle<Plot>,
}

pub(crate) fn spawn_bezier_function(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut plots: ResMut<Assets<Plot>>,
    mut spawn_beziercurve_event: EventReader<SpawnBezierCurveEvent>,
    mut bezier_materials: ResMut<Assets<BezierCurveUniform>>,
    // mut change_canvas_material_event: EventReader<RespawnAllEvent>,
    // mut change_canvas_material_event: EventReader<RespawnAllEvent>,
    query: Query<(Entity, &BezierCurveNumber)>,
    time: Res<Time>,
) {
    // for event in spawn_beziercurve_event.iter() {
    for event in spawn_beziercurve_event.iter() {
        //
        if let Some(mut plot) = plots.get_mut(&event.plot_handle.clone()) {
            //
            // remove all the bezier curves
            // TODO: currently runs proportionally to curve_number^2. Optimize
            for (entity, curve_number) in query.iter() {
                println!(
                    "number: {} despawned: {}",
                    curve_number.0, event.group_number
                );
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
                &mut bezier_materials,
                &time,
            );
        }
    }
}

pub(crate) fn animate_bezier(
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

fn plot_fn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    xs: Vec<f32>,
    curve_number: usize,
    plot: &mut Plot,
    plot_handle: &Handle<Plot>,
    bezier_materials: &mut ResMut<Assets<BezierCurveUniform>>,
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
        let mut normals = Vec::new();
        for position in mesh0 {
            mesh_pos_attributes.push([position.x, position.y, 0.0]);
            normals.push([0.0, 0.0, 1.0]);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_pos_attributes.clone());

        // let mva_ends = MeshVertexAttribute::new("Ends", 1, VertexFormat::Float32x4);

        mesh.insert_attribute(ATTRIBUTE_ENDS, ends);

        mesh.set_indices(Some(Indices::U32(inds)));

        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_attr_uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        // let mva_controls = MeshVertexAttribute::new("Vertext_Control", 3, VertexFormat::Float32x4);

        mesh.insert_attribute(ATTRIBUTE_CONTROL_POINT, mesh_attr_controls);

        // println!("mesh: {:?}", mesh.iter().map(|x| ));

        let bezier_material = BezierCurveUniform {
            mech: if bezier_curve.mech { 1.0 } else { 0.0 },
            dummy: plot.bezier_dummy,
            zoom: plot.zoom,
            inner_canvas_size_in_pixels: plot.canvas_size / (1.0 + plot.outer_border),
            canvas_position_in_pixels: plot.canvas_position,
            color: col_to_vec4(bezier_curve.color),
            size: bezier_curve.size,
            style: bezier_curve.line_style.clone().to_int32(),
        };

        let bezier_material_handle = bezier_materials.add(bezier_material);
        // commands
        //     .spawn_bundle((
        //         BezierMesh2d::default(),
        //         Mesh2dHandle(meshes.add(mesh)),
        //         GlobalTransform::default(),
        //         Transform::from_translation(plot.canvas_position.extend(1.10)),
        //         Visibility::default(),
        //         ComputedVisibility::default(),
        //     ))
        //     .insert(BezierCurveNumber(curve_number))
        //     .insert(plot_handle.clone());

        commands
            .spawn()
            // .spawn_bundle((
            //     SegmentMesh2d::default(),
            //     Mesh2dHandle(meshes.add(mesh)),
            //     GlobalTransform::default(),
            //     Transform::from_translation(plot.canvas_position.extend(1.11)),
            //     Visibility::default(),
            //     ComputedVisibility::default(),
            // ))
            .insert_bundle(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(mesh)),
                material: bezier_material_handle.clone(),
                transform: Transform::from_translation(plot.canvas_position.extend(1.10)),
                ..Default::default()
            })
            .insert(BezierCurveNumber(curve_number))
            .insert(plot_handle.clone());
        // .insert();
    }
}

/// A marker component for colored 2d meshes
#[derive(Component, Default)]
pub(crate) struct BezierMesh2d;

struct BezierMesh2dPipeline {
    // pub view_layout: BindGroupLayout,
    // pub mesh_layout: BindGroupLayout,
    pub mesh2d_pipeline: Mesh2dPipeline,
    pub custom_uniform_layout: BindGroupLayout,
    // This dummy white texture is to be used in place of optional textures
    // #[allow(dead_code)]
    // pub dummy_white_gpu_image: GpuImage,
    // pub shader_handle: Handle<Shader>,
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
                        min_binding_size: BufferSize::new(BezierCurveUniform::min_size().into()),
                    },
                    count: None,
                }],
                label: Some("custom_uniform_layout"),
            });

        // let world = world.cell();

        // let asset_server = world.get_resource::<AssetServer>().unwrap();
        // let shader_handle = asset_server.load("../assets/shaders/bezier_spline.wgsl");

        // let bezier_handle = world.get_resource::<BezierShaderHandle>().unwrap();

        Self {
            // view_layout: mesh2d_pipeline.view_layout,
            // mesh_layout: mesh2d_pipeline.mesh_layout,
            mesh2d_pipeline,
            custom_uniform_layout,
            // dummy_white_gpu_image: mesh2d_pipeline.dummy_white_gpu_image,
            // shader_handle: bezier_handle.0.clone(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct BezierPipelineKey {
    mesh: Mesh2dPipelineKey,
    shader_handle: Handle<Shader>,
}

// We implement `SpecializedPipeline` to customize the default rendering from `Mesh2dPipeline`
impl SpecializedRenderPipeline for BezierMesh2dPipeline {
    type Key = BezierPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        // layout: &MeshVertexBufferLayout,
    ) -> RenderPipelineDescriptor {
        // let mut descriptor = self.mesh2d_pipeline.specialize(key.mesh);

        let formats = vec![
            // Position
            VertexFormat::Float32x3,
            // Color
            VertexFormat::Float32x4,
            // UV
            VertexFormat::Float32x2,
            // Controls
            VertexFormat::Float32x4,
        ];

        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);

        RenderPipelineDescriptor {
            vertex: VertexState {
                // Use our custom shader
                shader: key.shader_handle.clone(),
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                // Use our custom vertex buffer
                // buffers: vec![VertexBufferLayout {
                //     array_stride: vertex_array_stride,
                //     step_mode: VertexStepMode::Vertex,
                //     attributes: vertex_attributes,
                // }],
                buffers: vec![vertex_layout],
            },
            fragment: Some(FragmentState {
                // Use our custom shader
                shader: key.shader_handle.clone(),
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            // Use the two standard uniforms for 2d meshes
            layout: Some(vec![
                // Bind group 0 is the view uniform
                self.mesh2d_pipeline.view_layout.clone(),
                // Bind group 1 is the mesh uniform
                self.mesh2d_pipeline.mesh_layout.clone(),
                //
                self.custom_uniform_layout.clone(),
            ]),
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: key.mesh.primitive_topology(),
                strip_index_format: None,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: key.mesh.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("colored_mesh2d_pipeline".into()),
        }
    }
}

/// Plugin that renders [`BezierMesh2d`]s
pub(crate) struct BezierMesh2dPlugin;

pub(crate) struct BezierShaderHandle(pub Handle<Shader>);

pub const BEZIER_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 1616035468340571005);

impl Plugin for BezierMesh2dPlugin {
    fn build(&self, app: &mut App) {
        // let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();

        let handle_untyped = BEZIER_SHADER_HANDLE.clone();

        shaders.set_untracked(
            handle_untyped.clone(),
            Shader::from_wgsl(include_str!("bezier_spline.wgsl")),
        );

        // let shader_typed_handle = shaders.get_handle(handle_untyped);

        app.add_plugin(Material2dPlugin::<BezierCurveUniform>::default());

        // let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        // render_app
        //     // .add_render_command::<Transparent2d, DrawBezierMesh2d>()
        //     .init_resource::<BezierMesh2dPipeline>()
        //     .init_resource::<SpecializedRenderPipelines<BezierMesh2dPipeline>>()
        //     .insert_resource(BezierShaderHandle(shader_typed_handle))
        //     .add_system_to_stage(RenderStage::Extract, extract_colored_mesh2d)
        //     .add_system_to_stage(RenderStage::Queue, queue_customuniform_bind_group)
        //     .add_system_to_stage(RenderStage::Queue, queue_colored_mesh2d);
    }
}

impl bevy::sprite::Material2d for BezierCurveUniform {
    fn vertex_shader() -> ShaderRef {
        let handle_untyped = BEZIER_SHADER_HANDLE.clone();
        let shader_handle: Handle<Shader> = handle_untyped.typed::<Shader>();
        shader_handle.into()
    }
    fn fragment_shader() -> ShaderRef {
        println!("frag shader");
        let handle_untyped = BEZIER_SHADER_HANDLE.clone();
        let shader_handle: Handle<Shader> = handle_untyped.typed::<Shader>();
        shader_handle.into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        println!("specialize");
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            // ATTRIBUTE_COLOR.at_shader_location(1),
            ATTRIBUTE_ENDS.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_CONTROL_POINT.at_shader_location(3),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

// let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

// mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_pos_attributes.clone());

// let mva_ends = MeshVertexAttribute::new("Ends", 1, VertexFormat::Float32x4);

// mesh.insert_attribute(mva_ends, ends);

// mesh.set_indices(Some(Indices::U32(inds)));

// mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_attr_uvs);

// let mva_controls = MeshVertexAttribute::new("Vertext_Control", 3, VertexFormat::Float32x4);

// mesh.insert_attribute(mva_controls, mesh_attr_controls);

// // println!("mesh: {:?}", mesh.iter().map(|x| ));
