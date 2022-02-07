use bevy::{
    prelude::*, reflect::TypeUuid, render::render_resource::std140::AsStd140,
    sprite::Material2dPlugin,
};

use super::plot_format::*;
use crate::bezier::*;
use crate::canvas::*;
use crate::inputs::*;
use crate::markers::*;
use crate::util::*;

pub struct PlotPlugin;

impl Plugin for PlotPlugin {
    fn build(&self, app: &mut App) {
        app
             // canvas
            .add_plugin(Material2dPlugin::<CanvasMaterial>::default())
            .add_plugin(MarkerMesh2dPlugin)
            .add_plugin(BezierMesh2dPlugin)
            .add_event::<SpawnGraphEvent>()
            .add_event::<ReleaseAllEvent>()
            .add_event::<UpdatePlotLabelsEvent>()
            .add_event::<ChangeCanvasMaterialEvent>()
            .add_event::<SpawnBezierCurveEvent>()
            .add_asset::<Plot>()
            .insert_resource(Cursor::default())

            .add_system_set(
                SystemSet::new().label("model").before("updates")             
                .with_system(adjust_graph_axes)
                .with_system(change_plot)
                .with_system(update_plot_labels)
            )
            .add_system_set(
                SystemSet::new().label("updates").before("axes")
                .with_system(update_canvas_material)
                .with_system(spawn_bezier_function)
                
            )
       
            .add_system(release_all)
            .add_system(spawn_graph)
            
            .add_system(adjust_graph_size)
            .add_system(record_mouse_events_system)
            .add_system(change_bezier_uni)
            .add_system(change_marker_uni)
            .add_system(update_mouse_target)
            .add_system(markers_setup.exclusive_system().at_end())
            // ...
            ;
    }
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
    // pub function:  Yo, //Box<dyn Fn(i32) -> i32>,
    
    pub color: Color,
}

impl Default for BezierData {
    fn default() -> Self {
        BezierData {
            // data: vec![],
            function: |x: f32| x, // Vec<fn(f32) -> f32>,
            color: Color::rgb(0.2, 0.3, 0.8),
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
}

impl Default for MarkerData {
    fn default() -> Self {
        MarkerData {
            data: vec![],
            color: Color::rgb(0.5, 0.5, 0.1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SegmentData {
    pub data: Vec<Vec2>,
    pub color: Color,
}

impl Default for SegmentData {
    fn default() -> Self {
        SegmentData {
            data: vec![],
            color: Color::rgb(0.2, 0.3, 0.8),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlotData {
    pub marker_plots: Vec<MarkerData>,
    pub segment_plots: Vec<SegmentData>,
    pub bezier_plots: Vec<BezierData>,
    
}

impl Default for PlotData {
    fn default() -> Self {
        PlotData {
            // marker_plot: MarkerData::default(),
            // segment_plot: SegmentData::default(),
            // bezier_plot: BezierData::default(),
            marker_plots: Vec::new(),
            segment_plots: Vec::new(),
            bezier_plots: Vec::new(), 
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlotOption {
    Color(Color),
}


#[derive(Debug, Clone)]
pub struct PlotOptions {
    pub ptype: Option<PlotType>,
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

            options: PlotOptions { ptype: None },

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
    pub fn pelotte<T: Plotable>(&mut self, v: T, options: Vec<PlotOptions>) {
        //
    } 

    pub fn plot<T: Plotable>(&mut self, v: T) {
        //

        let pf: PlotFormat = v.into_plot_format();
        let mut plot_type = pf.ptype;

        // coerce plot type if specified in plot options
        if let Some(ptype) = self.options.ptype.clone() {
            plot_type = ptype;
        }

        match plot_type {
            PlotType::Marker => {
                let new_data = MarkerData {
                    data: pf.data,
                    color: MarkerData::default().color,
                };
                
                self.data.marker_plots.push(new_data);
                self.send_plot_event.markers = true;
            }
            PlotType::Segment => {
                                let new_data = SegmentData {
                    data: pf.data,
                    color: SegmentData::default().color,
                };
                
                self.data.segment_plots.push(new_data);
                self.send_plot_event.segments = true;

                // self.data.segment_plot.data.push(pf.data);
            }
            _ => { println!("argument cannot be parsed") }
            // PlotType::Bezier => {
                
            //     let new_data = BezierData {
            //         function: pf.maybe_func.unwrap(),
            //         color: BezierData::default().color,
            //     };
                
            //     self.data.bezier_plots.push(new_data);
            //     self.send_plot_event.bezier = true;


            //     // self.data.bezier_plot.data.push(pf.data);
            //     // self.send_plot_event.bezier = true;
            // }
        }

        // for (i, mut data_point) in pf.iter().enumerate() {
        //     self.data[index][i] = Vec4::new(data_point.0, data_point.1, 0.0, 1.0);
        // }

        // self.lines_params[index].number_of_points = pf.len() as i32;
        // self.lines_params[index].transparency = 0.5;
    }

    pub fn plot_analytical(&mut self, f: fn(f32) -> f32) {
        // self.smooth_functions.push(f);
        let new_data = BezierData {
            function: f,
            color: BezierData::default().color,
        };
                
        self.data.bezier_plots.push(new_data);
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

        // self.update_thickness(multiplier);
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

        self.zero_world = lo_world - v + self.position * 0.0;

        // let bottom_left = -self.size / 2.0 / (1.0 + self.outer_border);

        // self.zero_world = Vec2::new(
        //     lo_world.x - self.bounds.lo.x * self.size.x / (1.0 + self.outer_border.x),
        //     lo_world.y - self.bounds.lo.y * self.size.y / (1.0 + self.outer_border.y),
        // );
    }

    pub fn compute_bounds_world(&self) -> PlotCanvasBounds {
        let lo = self.zero_world
            + self.bounds.lo * self.size
                / (self.bounds.up - self.bounds.lo)
                / (1.0 + self.outer_border.x);

        let up = self.zero_world
            + self.bounds.up * self.size
                / (self.bounds.up - self.bounds.lo)
                / (1.0 + self.outer_border.x);

        PlotCanvasBounds { up, lo }
    }

    // TODO: take inner border into account
    pub fn plot_to_world(&self, ys: &Vec<Vec2>) -> Vec<Vec2> {
        ys.iter()
            .map(|v| {
                self.zero_world
                    + *v * self.size
                        / (self.bounds.up - self.bounds.lo)
                        / (1.0 + self.outer_border.x)

                // Vec2::new(
                //     self.zero_world.x + v.x * self.size.x / (self.bounds.up.x - self.bounds.lo.x),
                //     self.zero_world.y + v.y * self.size.y / (self.bounds.up.y - self.bounds.lo.y),
                // )
            })
            .collect::<Vec<Vec2>>()
    }
}
