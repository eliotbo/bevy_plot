use bevy::prelude::*;

// use std::collections::HashMap;

pub(crate) struct ReleaseAllEvent;

#[derive(Component)]
pub(crate) struct Locked;

pub(crate) fn col_to_vec4(col: Color) -> Vec4 {
    Vec4::new(col.r(), col.g(), col.b(), col.a())
}
