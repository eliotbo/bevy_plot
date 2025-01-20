use bevy::prelude::*;

// use std::collections::HashMap;

#[derive(Event)]
pub(crate) struct ReleaseAllEvent;

#[derive(Component)]
pub(crate) struct Locked;

pub(crate) fn col_to_vec4(col: Color) -> Vec4 {
    let linear = col.to_linear();
    Vec4::new(linear.red, linear.green, linear.blue, linear.alpha)
}
