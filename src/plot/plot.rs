use bevy::{
    prelude::*, reflect::TypeUuid, render::render_resource::std140::AsStd140,
    sprite::Material2dPlugin,
};

use super::plot_format::*;
use super::colors::make_color_palette;
use crate::bezier::*;
use crate::canvas::*;
use crate::inputs::*;
use crate::markers::*;
use crate::util::*;
use crate::segments::*;

pub struct PlotPlugin;

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
            .add_event::<ChangeCanvasMaterialEvent>()
            .add_event::<WaitForUpdatePlotLabelsEvent>()
            // .add_event::<SpawnBezierCurveEvent>()
            .add_asset::<Plot>()
            .insert_resource(make_color_palette())
            .insert_resource(Cursor::default())

            .add_system_set(
                SystemSet::new().label("model").before("updates")             
                .with_system(adjust_graph_axes)
                .with_system(change_plot)
                
            )

            .add_system_set(
                SystemSet::new().label("updates")
                .with_system(update_canvas_material)
                .with_system(spawn_bezier_function)
                .with_system(wait_for_graph_spawn)
                
            )
       
            .add_system_set(
                SystemSet::new().label("other").after("updates")
                .with_system(release_all)
                .with_system(spawn_graph)
                .with_system(adjust_graph_size)
                .with_system(record_mouse_events_system)
                .with_system(change_bezier_uni)
                .with_system(change_marker_uni)
                .with_system(change_segment_uni)
                .with_system(update_mouse_target)
                .with_system(update_plot_labels)
            )
            .add_system(markers_setup.exclusive_system().at_end())
            .add_system(segments_setup.exclusive_system().at_end())
            // ...
            ;
    }
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
    pub function: fn(f32) -> f32,
    pub size: f32,
    pub line_style: LineStyle,
    pub draw_contour: bool,
    pub color: Color,
    pub mech: bool,
}

impl Default for BezierData {
    fn default() -> Self {
        BezierData {
            // data: vec![],
            function: |x: f32| x, // Vec<fn(f32) -> f32>,
            color: Color::rgb(0.2, 0.3, 0.8),
            size: 1.0,
            line_style: LineStyle::Solid,
            draw_contour: false,
            mech: false,
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
            draw_contour: true,
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

#[derive(Debug, Clone)]
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


#[derive(Debug, Clone)]
pub enum LineStyle{
    None,
    Solid,

    /// the following are not implemented yet
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

#[derive(Debug, Clone)]
// Options as the second argument the of plotop method
pub enum Opt {
    Color(Color),
    Size(f32),
    Contour(bool),

    // Bezier curve and segments only
    LineStyle(LineStyle),
    Mech(bool),

    

    // markers only
    MarkerStyle(MarkerStyle),
    MarkerInnerPointColor(Color),
    
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
    pub tick_period: Vec2,
    pub bounds: PlotCanvasBounds,
    pub globals: PlotGlobals,
    pub size: Vec2,
    pub outer_border: Vec2,
    pub position: Vec2,
    pub zero_world: Vec2,
    pub bezier_num_points: usize,
    // pub smooth_functions: Vec<fn(f32) -> f32>,

    // mouse_pos in the reference frame of the graph, corresponding to its axes coordinates
    pub relative_mouse_pos: Vec2,
    pub data: PlotData,
    pub options: PlotOptions,
    pub send_plot_event: SendPlotEvent,
}

impl Default for Plot {
    fn default() -> Plot {
        let size = Vec2::new(800.0, 500.0) * 1.0;

        let mut plot = Plot {
            relative_mouse_pos: Vec2::ZERO,

            tick_period: Vec2::new(0.2, 0.2),

            bounds: PlotCanvasBounds {
                up: Vec2::new(2.1, 3.4) * 1.0,
                lo: Vec2::new(-1.03, -0.85) * 1.0,
            },

            globals: PlotGlobals {
                time: 0.0,
                zoom: 1.0,
                dum1: 0.0, // for future use
                dum2: 0.0, // for future use
            },

            size: size.clone(),
            outer_border: Vec2::new(0.03 * size.y / size.x, 0.03),
            zero_world: Vec2::new(0.0, 0.0),

            // position: Vec2::new(65.0, 28.0) * 1.,
            // smooth_functions: Vec::new(),

            position: Vec2::ZERO,

            data: PlotData::default(),

            options: PlotOptions { plot_type: None },

            bezier_num_points: 100,

            send_plot_event: SendPlotEvent {
                markers: false,
                bezier: false,
                segments: false,
            },
        };

        plot.compute_zeros();
        plot
    }
}



impl Plot {
    pub fn plotopt<T: Plotable>(&mut self, v: T, options: Vec<Opt>) {
        //
        let pf: PlotFormat = v.into_plot_format();
        let mut plot_type = pf.ptype;

        // coerce plot type if specified in plot options
        if let Some(ptype) = self.options.plot_type.clone() {
            plot_type = ptype;
        }

        match plot_type {
            PlotType::Marker => {
                let mut data = MarkerData {
                    data: pf.data,
                    ..Default::default() 
                };

                for option in options.iter() {
                    match option {
                        Opt::Color(col) => { data.color = *col; },

                        Opt::Size(si)=> {
                            data.size = *si;
                        },
                        Opt::MarkerStyle(style)=> { data.marker_style = style.clone(); },
                        Opt::MarkerInnerPointColor(col)=> { data.marker_point_color = col.clone();},
                        Opt::LineStyle(_)=> {  eprintln!("LineStyle is not a valid option for markers"); },
                        Opt::Mech(_)=> { eprintln!("Mech is not a valid option for markers");  },
                        Opt::Contour(cont)=> { data.draw_contour = *cont; },
                    }
                }
                
                self.data.marker_groups.push(data);
                self.send_plot_event.markers = true;
            }
            PlotType::Segment => {
                let mut data = SegmentData {
                    data: pf.data,
                    ..Default::default() 
                };

                for option in options.iter() {
                    match option {
                        Opt::Color(col) => { data.color = *col; },

                        Opt::Size(si)=> {
                            data.size = *si;
                        },
                        Opt::LineStyle(style)=> { data.line_style = style.clone(); },

                        Opt::Contour(cont)=> { data.draw_contour = *cont;  },

                        Opt::Mech(mech)=> { data.mech = *mech; },

                        Opt::MarkerStyle(_)=> { 
                            eprintln!("MarkerStyle is not a valid option for segments"); 
                            },
                        Opt::MarkerInnerPointColor(_)=> {  
                            eprintln!("MarkerInnerPointColor is not a valid option for segments"); 
                            },
                        
                        
                        
                    }
                }
                
                self.data.segment_groups.push(data);
                self.send_plot_event.segments = true;

            }
            _ => { println!("argument cannot be parsed") }

        }
    } 

    pub fn plot<T: Plotable>(&mut self, v: T) {
        //

        let pf: PlotFormat = v.into_plot_format();
        let mut plot_type = pf.ptype;

        // coerce plot type if specified in plot options
        if let Some(ptype) = self.options.plot_type.clone() {
            plot_type = ptype;
        }

        match plot_type {
            PlotType::Marker => {
                let new_data = MarkerData {
                    data: pf.data,
                    ..Default::default()                   
                };
                
                self.data.marker_groups.push(new_data);
                self.send_plot_event.markers = true;
            }
            PlotType::Segment => {
                let new_data = SegmentData {
                    data: pf.data,
                    ..Default::default()  
                };
                
                self.data.segment_groups.push(new_data);
                self.send_plot_event.segments = true;

            }
            _ => { println!("argument cannot be parsed") }

        }

    }

    pub fn plot_analytical(&mut self, f: fn(f32) -> f32) {
        // self.smooth_functions.push(f);
        let new_data = BezierData {
            function: f,
            ..Default::default()
        };
                
        self.data.bezier_groups.push(new_data);
        self.send_plot_event.bezier = true;

    }

    pub fn plotopt_analytical(&mut self, f: fn(f32) -> f32, options: Vec<Opt>) {
        // self.smooth_functions.push(f);
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

                Opt::Contour(cont)=> { data.draw_contour = *cont;  },

                Opt::Mech(mech)=> { data.mech = *mech; },

                Opt::MarkerStyle(_)=> { 
                    eprintln!("MarkerStyle is not a valid option for segments"); 
                    },
                Opt::MarkerInnerPointColor(_)=> {  
                    eprintln!("MarkerInnerPointColor is not a valid option for segments"); 
                    },

                
            }
        }
                

                
        self.data.bezier_groups.push(data);
        self.send_plot_event.bezier = true;

    }

    pub fn delta_axes(&self) -> Vec2 {
        self.bounds.up - self.bounds.lo
    }

    pub fn zoom_axes(&mut self, direction: f32) {
        let percent_factor = 10.0;

        let multiplier = 1.0 + direction * percent_factor / 100.0;

        self.bounds.up =
            self.relative_mouse_pos + (self.bounds.up - self.relative_mouse_pos) * multiplier;
        self.bounds.lo =
            self.relative_mouse_pos - (self.relative_mouse_pos - self.bounds.lo) * multiplier;

        self.globals.zoom *= multiplier;
    }

    pub fn move_axes(&mut self, mouse_delta: Vec2) {
        let mut axes = self.delta_axes();
        axes.x *= -1.0;
        let size = self.size / (1. + self.outer_border);

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

    pub fn compute_zeros(&mut self) {
        let lo_world = -self.size / 2.0 / (1.0 + self.outer_border);

        let v = Vec2::new(
            self.bounds.lo.x * self.size.x
                / (1.0 + self.outer_border.x)
                / (self.bounds.up.x - self.bounds.lo.x),
            self.bounds.lo.y * self.size.y
                / (1.0 + self.outer_border.y)
                / (self.bounds.up.y - self.bounds.lo.y),
        );

        self.zero_world = lo_world - v ;
    }

    pub fn compute_bounds_world(&self) -> PlotCanvasBounds {

        let lo = self.to_world(self.bounds.lo);
        let up = self.to_world(self.bounds.up);

        PlotCanvasBounds { up, lo }
    }

    pub fn to_world(&self, v: Vec2) -> Vec2 {

                self.zero_world
                    + v * self.size
                        / (self.bounds.up - self.bounds.lo)
                        / (1.0 + self.outer_border.x)


    }

    pub fn world_to_plot(&self, y: Vec2) -> Vec2 {
        (y - self.zero_world - self.position) * (self.bounds.up - self.bounds.lo)
            / self.size
            * (1.0 + self.outer_border) 
    }
}
