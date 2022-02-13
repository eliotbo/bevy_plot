// #![warn(missing_docs)]

//! Plotting library for the Bevy game engine. To quickly get started, run a Bevy ```App```, instantiate
//! a ```Plot``` struct, and either use the
//! *  ```plot(my_data: impl Plotable)``` method for a regular graph or the
//! * ```plotm(my_data: impl Plotable)``` method for a scatter plot (or plot with markers) or the
//! * ```plot_func(arg: fn(f32, f32) -> 32)``` method that supports plotting of explicit functions.
//!
//! The ```my_data``` argument of either of the first two methods has to implement the ```Plotable``` trait
//! (e.g. ```Vec<Vec2>```, ```Vec<(f32, f32)>```, ```Vec<f32>```, etc.). In the third option, the
//! argument ```arg``` is a function that takes two arguments (x and time) and returns a ```f32```.
//!
//! The following code can be found in examples/minimal.rs:
//! ```
//!  use bevy::prelude::*;
//!  use bevy_plot::*;
//!  
//!  fn main() {
//!      App::new()
//!          .add_plugins(DefaultPlugins)
//!          .add_plugin(PlotPlugin)
//!          .add_startup_system(setup)
//!          .run();
//!  }
//!  
//!  fn setup(mut commands: Commands, mut plots: ResMut<Assets<Plot>>) {
//!      commands.spawn_bundle(OrthographicCameraBundle::new_2d());
//!  
//!      let mut plot = Plot::default();
//!  
//!      let xs = (0..30).map(|i| i as f32 / 30.0).collect::<Vec<f32>>();
//!  
//!      let ys = xs
//!          .iter()
//!          .map(|x| Vec2::new(*x, 0.5 * *x))
//!          .collect::<Vec<Vec2>>();
//!  
//!      plot.plot(ys);
//!  
//!      let plot_handle = plots.add(plot.clone());
//!      commands.spawn().insert(plot_handle);
//!  }
//! ```
//!
//!
//! For customizing the look of the curves and markers, see the ```Opt``` enum for the
//! available options together with the ```plotopt``` and
//! ```plotopt_func``` methods. For customizing the canvas (grid, colors, etc...), see the ```Plot``` fields.
//! Setting the range of the x and y axes is done with the ```set_bounds(lo, up)``` method, but bevy_plot
//! panics if lo.x > up.x or lo.y > up.y.
//!
//! Note that the library allows the user to
//! * zoom in and out with the mousewheel,
//! * move the origin with the mouse by pressing and dragging,
//! * spawn a target and the corresponding coordinates by pressing the Mouse::Middle button, and
//! * change the Plot fields at runtime (see examples/runtime_setter.rs).

mod plot;
pub use plot::*;

mod bezier;
mod canvas;
mod inputs;
mod markers;
mod segments;
mod util;
