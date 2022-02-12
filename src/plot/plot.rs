use bevy::{
    prelude::*, reflect::TypeUuid, render::render_resource::std140::AsStd140,
    sprite::Material2dPlugin,
};

use super::plot_format::*;
use super::colors::make_color_palette;
use crate::canvas::*;
use crate::bezier::*;

use crate::inputs::*;
use crate::markers::*;
use crate::util::*;
use crate::segments::*;

pub struct PlotPlugin;

// z planes from bottom to top:

// canvas: 0.0001
// text and labels: 1.0001
// bezier 1.10
// segments: 1.11
// markers: 1.12
// target text: 1.2


impl Plugin for PlotPlugin {
    fn build(&self, app: &mut App) {
        app
             // canvas
            .add_plugin(Material2dPlugin::<CanvasMaterial>::default())
            .add_plugin(MarkerMesh2dPlugin)
            .add_plugin(BezierMesh2dPlugin)
            .add_plugin(SegmentMesh2dPlugin)
            .add_event::<SpawnGraphEvent>()
            .add_event::<ReleaseAllEvent>()
            .add_event::<UpdatePlotLabelsEvent>()
            .add_event::<RespawnAllEvent>()
            .add_event::<WaitForUpdatePlotLabelsEvent>()
            .add_event::<UpdateTargetLabelEvent>()
            .add_event::<UpdateBezierShaderEvent>()
            .add_event::<SpawnBezierCurveEvent>()
            .add_asset::<Plot>()
            .insert_resource(make_color_palette())
            .insert_resource(Cursor::default())

            .add_system_set(
                SystemSet::new().label("model").before("shader_updates")             
                .with_system(adjust_graph_axes)
                .with_system(change_plot)
                
            )

            .add_system_set(
                SystemSet::new().label("shader_updates")
                // .with_system(update_canvas_material)
                .with_system(update_bezier_uniform)
                .with_system(spawn_bezier_function)
                .with_system(wait_for_graph_spawn)
                
            )
       
            .add_system_set(
                SystemSet::new().label("other").after("shader_updates")
                .with_system(release_all)
                .with_system(spawn_graph)
                .with_system(adjust_graph_size)
                .with_system(record_mouse_events_system)
                // .with_system(change_bezier_uni)
                // .with_system(change_marker_uni)
                // .with_system(change_segment_uni)
                .with_system(update_mouse_target)
                .with_system(update_plot_labels)
                .with_system(update_target)
                .with_system(do_spawn_plot)
                .with_system(animate_bezier)
            )
            .add_system(markers_setup.exclusive_system().at_end())
            .add_system(segments_setup.exclusive_system().at_end())
            // ...
            ;
    }
}


fn do_spawn_plot(
    mut commands: Commands,
    mut plots: ResMut<Assets<Plot>>, 
    query: Query<(Entity, &Handle<Plot>)>,
    mut spawn_plot_event: EventWriter<SpawnGraphEvent>
) {
    for (entity, plot_handle) in query.iter() {
        let plot = plots.get_mut(plot_handle).unwrap();
        if plot.do_spawn_plot {

            let canvas = plot.make_canvas();

            spawn_plot_event.send(SpawnGraphEvent {
                canvas,
                plot_handle: plot_handle.clone(),
            });

            plot.do_spawn_plot = false;

            // To access the plot handle, earlier we spawned an entity with the plot handle.
            // This entity's purpose has been served and it is time to despawn it already.
            commands.entity(entity).despawn();
        }
    }
}

pub struct UpdateBezierShaderEvent {
    pub plot_handle: Handle<Plot>,
    pub entity: Entity,
    pub group_number: usize,
}


pub struct WaitForUpdatePlotLabelsEvent {
    pub plot_handle: Handle<Plot>,
    pub quad_entity: Entity,
}

#[derive(Debug, Clone, AsStd140)]
pub struct PlotCanvasBounds {
    pub up: Vec2,
    pub lo: Vec2,
}
#[derive(Debug, Copy, Clone, AsStd140)]
pub struct PlotGlobals {
    pub time: f32,
    pub zoom: f32,
    pub dum1: f32,
    pub dum2: f32,
}

impl Default for PlotGlobals {
    fn default() -> Self {
        PlotGlobals {
            time: 0.0,
            zoom: 1.0,
            dum1: 0.0,
            dum2: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
// struct containing the data to be plotted and the color of the curves
pub struct BezierData {
    pub function: fn(f32, f32) -> f32,

    pub size: f32,
    pub line_style: LineStyle,
    pub draw_contour: bool,
    pub color: Color,
    pub mech: bool,
    pub num_points: usize,
    pub show_animation: bool,
}

impl Default for BezierData {
    fn default() -> Self {
        BezierData {
            function: |x: f32, _t: f32| x, // Vec<fn(f32) -> f32>,
            color: Color::rgb(0.2, 0.3, 0.8),
            size: 1.0,
            line_style: LineStyle::Solid,
            draw_contour: false,
            mech: false,
            num_points: 256,
            show_animation: false,
        }
    }
}


// #[derive(Debug, Clone)]
// pub struct Yo { bam: Box<dyn Fn(i32) -> i32>}

// unsafe impl Sync for Yo {}
// unsafe impl Send for Yo {}

#[derive(Debug, Clone)]
pub struct MarkerData {
    pub data: Vec<Vec2>,
    pub color: Color,
    pub marker_point_color: Color,
    pub marker_style: MarkerStyle,
    pub size: f32,
    pub draw_contour: bool,
}

impl Default for MarkerData {
    fn default() -> Self {
        MarkerData {
            data: vec![],
            color: Color::rgb(0.5, 0.5, 0.1),
            marker_point_color: Color::rgb(0.2, 0.3, 0.8),
            marker_style: MarkerStyle::Circle,
            size: 1.0,
            draw_contour: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SegmentData {
    pub data: Vec<Vec2>,
    pub color: Color,
    pub segment_point_color: Color,
    pub size: f32,
    pub line_style: LineStyle,
    pub draw_contour: bool,
    pub mech: bool,
}

impl Default for SegmentData {
    fn default() -> Self {
        SegmentData {
            data: vec![],
            color: Color::rgb(0.6, 0.3, 0.2),
            segment_point_color: Color::rgb(0.2, 0.3, 0.8),
            size: 1.0,
            line_style: LineStyle::Solid,
            draw_contour: false,
            mech: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlotData {
    pub marker_groups: Vec<MarkerData>,
    pub segment_groups: Vec<SegmentData>,
    pub bezier_groups: Vec<BezierData>,
    
}

impl Default for PlotData {
    fn default() -> Self {
        PlotData {
            marker_groups: Vec::new(),
            segment_groups: Vec::new(),
            bezier_groups: Vec::new(), 
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MarkerStyle {
    None,
    Circle,
    Square,
    Triangle,
    Heart,
    Cross,
    Rhombus,
    Star,
    Moon,
    X,
}

impl MarkerStyle {
    pub fn to_int32(&self) -> i32 {
        match self {
            MarkerStyle::None => -1,
            MarkerStyle::Square => 0,
            MarkerStyle::Heart => 1,
            MarkerStyle::Triangle => 3,
            MarkerStyle::Rhombus => 2,
            MarkerStyle::Star => 4,
            MarkerStyle::Moon => 5,
            MarkerStyle::Cross => 6,
            MarkerStyle::X => 7,
            MarkerStyle::Circle => 8, 
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum LineStyle{
    None,
    Solid,

    /// unimplemented
    Dashed,
    Dotted,
    DashDot,
    DashDotDot,
}

impl LineStyle {
    pub fn to_int32(&self) -> i32 {
        match self {
            LineStyle::None => -1,
            LineStyle::Solid => 0,
            LineStyle::Dashed => 1,
            LineStyle::Dotted => 2,
            LineStyle::DashDot => 3,
            LineStyle::DashDotDot => 4,
        }
    }
}

/// Options for customizing the appearance of the plot.
#[derive(Debug, Clone, PartialEq)]
// Options as the second argument the of plotop method
pub enum Opt {
    /// Shared between plotopt_analytical() and plotopt()
    Color(Color),
    Size(f32),
    LineStyle(LineStyle),
    Mech(bool),
    
    /// Works with plotopt_analytical() only.
    NumPoints(usize),
    Animate(bool),

    /// Use with plotopt() exclusively.
    MarkerColor(Color),
    MarkerSize(f32),
    MarkerStyle(MarkerStyle),
    MarkerInnerPointColor(Color),
    Contour(bool),
    
}


#[derive(Debug, Clone)]
pub struct PlotOptions {
    pub plot_type: Option<PlotType>,
}

#[derive(Debug, Clone)]
pub struct SendPlotEvent {
    pub markers: bool,
    pub bezier: bool,
    pub segments: bool,
}

#[derive(Debug, Clone, Component, TypeUuid)]
// #[derive(Component, TypeUuid)]
#[uuid = "a6354c45-cc21-48f7-99cc-8c1924d2427b"]
pub struct Plot {

    /// canvas related
    pub tick_period: Vec2,
    pub(crate) bounds: PlotCanvasBounds,
    pub globals: PlotGlobals,
    pub canvas_position: Vec2,
    // pub canvas_bs_color : Color, 
    pub outer_border: Vec2,
    pub canvas_size: Vec2,
    pub background_color1: Color,
    pub background_color2: Color,
    pub show_grid: bool,
    pub zero_world: Vec2,

    pub hide_contour: bool,
    pub hide_tick_labels: bool,
    pub hide_half_ticks: bool,
    pub significant_digits: usize,
    pub show_target: bool,
    pub show_axes: bool,
    pub(crate) target_toggle: bool,
    pub tick_label_color: Color,
    pub target_label_color: Color,
    pub target_color: Color,
    pub target_position: Vec2,
    pub target_significant_digits: usize,

    /// Only related to the plot_analytical() and plotopt_analytical() functions
    pub bezier_num_points: usize,
    pub bezier_dummy: f32,



    // mouse_pos in the reference frame of the graph, corresponding to its axes coordinates
    pub plot_coord_mouse_pos: Vec2,
    pub data: PlotData,
    pub options: PlotOptions,
    pub send_plot_event: SendPlotEvent,
    pub handle: Option<Handle<Plot>>,
    pub do_spawn_plot: bool,
}

impl Default for Plot {
    fn default() -> Plot {
        let size = Vec2::new(800.0, 500.0);

        let mut plot = Plot {
            plot_coord_mouse_pos: Vec2::ZERO,

            tick_period: Vec2::new(0.2, 0.2),

            bounds: PlotCanvasBounds {
                up: Vec2::new(1.2, 1.2), 
                lo: Vec2::new(-0.2, -0.2),
            },

            globals: PlotGlobals {
                time: 0.0, // unused
                zoom: 1.0,
                dum1: 0.0, // (for tests only)
                dum2: 0.0, // (for tests only)
            },

            show_grid: true,
            background_color1: Color::rgba(0.048, 0.00468, 0.0744, 1.0) ,
            background_color2: Color::rgba(0.0244, 0.0023, 0.0372, 1.0) ,

            canvas_size: size.clone(),
            outer_border: Vec2::new(0.03 * size.y / size.x, 0.03),
            zero_world: Vec2::new(0.0, 0.0),

            hide_contour: false,
            hide_tick_labels: false,
            hide_half_ticks: true,
            significant_digits: 2,
            show_axes: true,
            show_target: false,
            target_toggle: false,
            tick_label_color: Color::BLACK,
            target_label_color: Color::GRAY,
            target_color: Color::GRAY,
            target_position: Vec2::new(0.0, 0.0),
            target_significant_digits: 2,



            canvas_position: Vec2::ZERO,

            data: PlotData::default(),

            options: PlotOptions { plot_type: None },

            bezier_num_points: 100,
            bezier_dummy: 0.0,


            send_plot_event: SendPlotEvent {
                markers: false,
                bezier: false,
                segments: false,
            },

            handle: None,

            do_spawn_plot: true,

        };

        plot.compute_zeros();
        plot
    }
}



impl Plot {
    /// Customizable plotting function.
    pub fn plotopt<T: Plotable>(&mut self, v: T, options: Vec<Opt>) {
        //
        let data_in_plot_format: PlotFormat = v.into_plot_format();

        if !options.contains(&Opt::LineStyle(LineStyle::None)) {
            let mut data = SegmentData {
                data: data_in_plot_format.data.clone(),
                ..Default::default() 
            };

            for option in options.iter() {
                match option {
                    Opt::Color(col) => { data.color = *col; },

                    Opt::Size(si)=> {
                        data.size = *si;
                    },
                    Opt::LineStyle(style)=> { data.line_style = style.clone(); },

                    Opt::Mech(mech)=> { data.mech = *mech; },

                    _ => {},

                }
            }
                
            self.data.segment_groups.push(data);
            self.send_plot_event.segments = true;
        }

        // Decide whether to draw markers using the options.
        // If any of MarkerStyle or MarkerSize is specified, draw markers
        let draw_markers = options.iter().map(|opt| {
            if let &Opt::MarkerStyle(_) = opt  { true } 
            else if let &Opt::MarkerSize(_) = opt  { true } 
            else { false }
        }).any(|x| x);

        if draw_markers {
            let mut data = MarkerData {
                data: data_in_plot_format.data.clone(),
                ..Default::default() 
            };

            for option in options.iter() {
                match option {
                    Opt::MarkerColor(col) => { data.color = *col; },

                    Opt::MarkerSize(si)=> {
                        data.size = *si;
                    },
                    Opt::MarkerStyle(style)=> { data.marker_style = style.clone(); },
                    Opt::MarkerInnerPointColor(col) => { data.marker_point_color = col.clone();},
                    Opt::Contour(cont)=> { data.draw_contour = *cont; },
                    _ => {},

                }
            }
            
            self.data.marker_groups.push(data);
            self.send_plot_event.markers = true;
        }
    } 

    /// Quickly plot data points using segments to connect consecutive points. 
    pub fn plot<T: Plotable>(&mut self, v: T) {
        //
        let pf: PlotFormat = v.into_plot_format();

        let new_data = SegmentData {
            data: pf.data,
            ..Default::default()  
        };
        
        self.data.segment_groups.push(new_data);
        self.send_plot_event.segments = true;

        
    }

    /// Quickly plot data points using markers (scatter plot). 
    pub fn plotm<T: Plotable>(&mut self, v: T) {
        //
        let pf: PlotFormat = v.into_plot_format();

        let new_data = MarkerData {
            data: pf.data,
            ..Default::default()                   
        };
        
        self.data.marker_groups.push(new_data);
        self.send_plot_event.markers = true;
        
    }

    /// Plot a function by providing said function.
    pub fn plot_analytical(&mut self, f: fn(f32, f32) -> f32) {
        //
        let new_data = BezierData {
            function: f,
            ..Default::default()
        };
                
        self.data.bezier_groups.push(new_data);
        self.send_plot_event.bezier = true;

    }

    /// Plot a function by providing said function and options.
    pub fn plotopt_analytical(&mut self, f: fn(f32, f32) -> f32, options: Vec<Opt>) {
        //
        let mut data = BezierData {
            function: f,
            ..Default::default()
        };


        for option in options.iter() {
            match option {
                Opt::Color(col) => { data.color = *col; },

                Opt::Size(si)=> {
                    data.size = *si;
                },
                Opt::LineStyle(style)=> { data.line_style = style.clone(); },

                Opt::Mech(mech)=> { data.mech = *mech; },

                Opt::Animate(animate) => { data.show_animation = *animate; }

                Opt::MarkerStyle(_)=> { 
                    eprintln!("MarkerStyle is not a valid option for segments"); 
                },

                Opt::MarkerInnerPointColor(_)=> {  
                    eprintln!("MarkerInnerPointColor is not a valid option for segments"); 
                },

                Opt::Contour(_)=> { 
                     println!("Contour is not a valid option for segments");
                },
                
                Opt::NumPoints(_) => { 
                    eprintln!("NumPoints is not a valid option for segments"); 
                },

                Opt::MarkerColor(_) => { 
                    eprintln!("MarkerColor is not a valid option for segments"); 
                },

                Opt::MarkerSize(_) => { 
                    eprintln!("MarkerSize is not a valid option for segments"); 
                },

                // _ => {},
            }
        }
        self.data.bezier_groups.push(data);
        self.send_plot_event.bezier = true;
    }

    
    pub fn make_canvas(&self) -> Canvas {

        // generate a random id (collisions are possible)
        let canvas = Canvas {
            // id,
            position: self.canvas_position,
            previous_position: self.canvas_position,
            original_size: self.canvas_size,
            scale: Vec2::splat(1.0),
            previous_scale: Vec2::splat(1.0),
            hover_radius: 20.0,
            // plot_handle: None,
        };

        canvas

    }

    pub fn delta_axes(&self) -> Vec2 {
        self.bounds.up - self.bounds.lo
    }

    pub fn zoom_axes(&mut self, direction: f32) {
        let percent_factor = 10.0;

        let multiplier = 1.0 + direction * percent_factor / 100.0;

        self.bounds.up =
            self.plot_coord_mouse_pos + (self.bounds.up - self.plot_coord_mouse_pos) * multiplier;
        self.bounds.lo =
            self.plot_coord_mouse_pos - (self.plot_coord_mouse_pos - self.bounds.lo) * multiplier;

        self.globals.zoom *= multiplier;
    }

    pub fn move_axes(&mut self, mouse_delta: Vec2) {
        let mut axes = self.delta_axes();
        axes.x *= -1.0;
        let size = self.canvas_size / (1. + self.outer_border);

        self.bounds.up += mouse_delta * axes / size;
        self.bounds.lo += mouse_delta * axes / size;
    }

    pub fn clamp_tick_period(&mut self) {
        let max_num_ticks = 15.0;
        let min_num_ticks = 0.000001;

        self.tick_period.x = self.tick_period.x.clamp(
            self.delta_axes().x / max_num_ticks,
            self.delta_axes().x / min_num_ticks,
        );

        self.tick_period.y = self.tick_period.y.clamp(
            self.delta_axes().y / max_num_ticks,
            self.delta_axes().x / min_num_ticks,
        );
    }

    /// Override the default plot bounds. Beware! The tick period is automatically adjusted.
    /// Changing the tick period before setting the bounds will not have the intended effect.
    /// The bounds must be set before the ticks.
    pub fn set_bounds(&mut self, lo: Vec2, up: Vec2) {
        self.bounds = PlotCanvasBounds {
            lo,
            up,
        };

        let delta = up - lo;
        let exact_tick = delta / 10.0;

        // println!("exact_tick: {}", exact_tick);
        // find order of magnitude of dx
        let order_x = exact_tick.x.log10().floor();
        let mag_x = 10_f32.powf(order_x);
        // println!("magx: {}", magx);

        let p1x = mag_x * 1.0;
        let p2x = mag_x * 2.0;
        let p5x = mag_x * 5.0;

        let psx = [p1x, p2x, p5x];
        //  println!("psx: {:?}", psx);

        let vx = vec! [(p1x-exact_tick.x).abs() , (p2x-exact_tick.x).abs(), (p5x-exact_tick.x).abs()];
        
        use std::cmp::Ordering;
        let min_x_index = vx.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index);

        let tick_x = psx[min_x_index.unwrap()]; 


        let order_y = exact_tick.y.log10().floor();
        let mag_y = 10_f32.powf(order_y);
        // println!("magxy {}", mag_y);

        let p1y = mag_y * 1.0;
        let p2y = mag_y * 2.0;
        let p5y = mag_y * 5.0;

        let psy = [p1y, p2y, p5y];
        //  println!("psy: {:?}", psy);

        let vy = vec! [(p1y-exact_tick.y).abs() , (p2y-exact_tick.y).abs(), (p5y-exact_tick.y).abs()];
        
        let min_y_index = vy.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index);

        let tick_y = psy[min_y_index.unwrap()]; 
        // println!("tick_y: {}", tick_y);

  

        self.tick_period = Vec2::new(tick_x, tick_y);

        self.compute_zeros();
    }

    pub fn compute_zeros(&mut self) {
        let lo_world = -self.canvas_size / 2.0 / (1.0 + self.outer_border);

        let v = Vec2::new(
            self.bounds.lo.x * self.canvas_size.x
                / (1.0 + self.outer_border.x)
                / (self.bounds.up.x - self.bounds.lo.x),
            self.bounds.lo.y * self.canvas_size.y
                / (1.0 + self.outer_border.y)
                / (self.bounds.up.y - self.bounds.lo.y),
        );

        self.zero_world = lo_world - v ;
    }

    pub fn compute_bounds_world(&self) -> PlotCanvasBounds {

        let lo = self.to_local(self.bounds.lo);
        let up = self.to_local(self.bounds.up);

        PlotCanvasBounds { up, lo }
    }

    pub fn to_local(&self, v: Vec2) -> Vec2 {

                self.zero_world
                    + v * self.canvas_size
                        / (self.bounds.up - self.bounds.lo)
                        / (1.0 + self.outer_border.x)


    }

    pub fn world_to_plot(&self, y: Vec2) -> Vec2 {
        (y - self.zero_world - self.canvas_position) * (self.bounds.up - self.bounds.lo)
            / self.canvas_size
            * (1.0 + self.outer_border) 
    }
}
