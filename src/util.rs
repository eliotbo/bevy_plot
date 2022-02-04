use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::std140::AsStd140};

// use std::collections::HashMap;

pub struct ReleaseAllEvent;

#[derive(Component)]
pub struct Locked;

pub type KnobId = u32;

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

#[derive(Debug, Copy, Clone, AsStd140)]
pub struct LineParams {
    pub thickness: f32,
    pub point_type: i32,
    pub point_radius: f32,
    pub number_of_points: i32,
    pub transparency: f32,
    pub point_color: Vec4,
    pub color: Vec4,
}

impl Default for LineParams {
    fn default() -> Self {
        LineParams {
            thickness: 1.0,
            point_type: 0,
            point_radius: 1.0,
            number_of_points: 0,
            transparency: 1.0,
            point_color: Vec4::new(0.13, 0.28, 0.86, 1.0),
            color: Vec4::new(0.13, 0.28, 0.86, 1.0),
        }
    }
}

#[derive(Debug, Clone, Component, TypeUuid)]
#[uuid = "a6354c45-cc21-48f7-99cc-8c1924d2427b"]
pub struct Plot {
    // mouse_pos in the reference frame of the graph, corresponding to its axes coordinates
    pub tick_period: Vec2,
    pub bounds: PlotCanvasBounds,
    pub globals: PlotGlobals,
    pub size: Vec2,
    pub outer_border: Vec2,
    pub position: Vec2,
    pub zero_world: Vec2,
    // pub zero: Vec2,
    pub relative_mouse_pos: Vec2,
}

impl Plot {
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

        self.zero_world = lo_world - v;

        // let bottom_left = -self.size / 2.0 / (1.0 + self.outer_border);

        // self.zero_world = Vec2::new(
        //     lo_world.x - self.bounds.lo.x * self.size.x / (1.0 + self.outer_border.x),
        //     lo_world.y - self.bounds.lo.y * self.size.y / (1.0 + self.outer_border.y),
        // );
    }

    // TODO: take inner border into account
    pub fn plot_to_world(&mut self, ys: &Vec<Vec2>) -> Vec<Vec2> {
        self.compute_zeros();

        ys.iter()
            .map(|v| {
                self.zero_world + *v * self.size / (self.bounds.up - self.bounds.lo)

                // Vec2::new(
                //     self.zero_world.x + v.x * self.size.x / (self.bounds.up.x - self.bounds.lo.x),
                //     self.zero_world.y + v.y * self.size.y / (self.bounds.up.y - self.bounds.lo.y),
                // )
            })
            .collect::<Vec<Vec2>>()
    }
}
