use bevy::{prelude::*, render::render_resource::std140::AsStd140};

// use std::collections::HashMap;

pub struct ReleaseAllEvent;

#[derive(Component)]
pub struct Locked;

pub type KnobId = u32;

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

pub fn col_to_vec4(col: Color) -> Vec4 {
    Vec4::new(col.r(), col.g(), col.b(), col.a())
}
